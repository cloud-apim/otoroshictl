//! Accept loop shared by the HTTP servers run by otoroshictl.

use std::error::Error as StdError;
use std::future::Future;
use std::net::SocketAddr;
use std::time::Duration;

use hyper::body::{Body, Incoming};
use hyper::service::Service;
use hyper::{Request, Response};
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server::conn::auto;
use hyper_util::server::graceful::GracefulShutdown;
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;

/// Maximum duration of a TLS handshake, so that a silent client can neither
/// hold a task forever nor delay a graceful shutdown.
const TLS_HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(10);

/// Bind a listener on `addr`, panicking when the address is not available as
/// `hyper::Server::bind` used to.
pub async fn bind(addr: SocketAddr) -> TcpListener {
    TcpListener::bind(addr)
        .await
        .unwrap_or_else(|e| panic!("error binding to {}: {}", addr, e))
}

/// Serve HTTP/1 and HTTP/2 connections accepted on `listener` until `shutdown`
/// resolves, then wait for the in-flight connections to complete.
///
/// When `tls` is set, every connection goes through a TLS handshake first; the
/// HTTP version is then negotiated through ALPN.
pub async fn serve<S, B>(
    listener: TcpListener,
    tls: Option<TlsAcceptor>,
    service: S,
    shutdown: impl Future<Output = ()>,
) where
    S: Service<Request<Incoming>, Response = Response<B>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
    B: Body + Send + 'static,
    B::Data: Send,
    B::Error: Into<Box<dyn StdError + Send + Sync>>,
{
    let builder = auto::Builder::new(TokioExecutor::new());
    let graceful = GracefulShutdown::new();
    tokio::pin!(shutdown);
    loop {
        let (stream, peer) = tokio::select! {
            accepted = listener.accept() => match accepted {
                Ok(accepted) => accepted,
                Err(e) => {
                    // Usually a lack of file descriptors: back off like hyper 0.14 did.
                    warn!("failed to accept connection: {}", e);
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    continue;
                }
            },
            _ = &mut shutdown => break,
        };
        let builder = builder.clone();
        let service = service.clone();
        let tls = tls.clone();
        let watcher = graceful.watcher();
        // The TLS handshake runs in the connection task so that a slow client
        // never blocks the accept loop.
        tokio::spawn(async move {
            let result = match tls {
                Some(acceptor) => {
                    match tokio::time::timeout(TLS_HANDSHAKE_TIMEOUT, acceptor.accept(stream)).await
                    {
                        Ok(Ok(stream)) => {
                            let conn = builder.serve_connection(TokioIo::new(stream), service);
                            watcher.watch(conn.into_owned()).await
                        }
                        Ok(Err(e)) => {
                            debug!("TLS handshake with {} failed: {}", peer, e);
                            return;
                        }
                        Err(_) => {
                            debug!("TLS handshake with {} timed out", peer);
                            return;
                        }
                    }
                }
                None => {
                    let conn = builder.serve_connection(TokioIo::new(stream), service);
                    watcher.watch(conn.into_owned()).await
                }
            };
            if let Err(e) = result {
                debug!("connection with {} closed with error: {}", peer, e);
            }
        });
    }
    // Stop accepting new connections while the in-flight ones complete.
    drop(listener);
    graceful.shutdown().await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http_utils::client::{RequestBody, http_client, read_body};
    use http_body_util::Full;
    use hyper::body::Bytes;
    use hyper::service::service_fn;
    use std::convert::Infallible;
    use std::sync::Arc;
    use tokio::sync::Notify;

    async fn get(port: u16, path: &str) -> Bytes {
        let req = Request::get(format!("http://127.0.0.1:{}{}", port, path))
            .body(RequestBody::default())
            .unwrap();
        read_body(http_client().request(req).await.unwrap())
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn test_graceful_shutdown_drains_requests_and_refuses_new_connections() {
        // `/slow` signals `started`, then waits for `release` before answering.
        let started = Arc::new(Notify::new());
        let release = Arc::new(Notify::new());
        let svc = {
            let (started, release) = (started.clone(), release.clone());
            service_fn(move |req: Request<Incoming>| {
                let (started, release) = (started.clone(), release.clone());
                async move {
                    if req.uri().path() == "/slow" {
                        started.notify_one();
                        release.notified().await;
                    }
                    let body = format!("{} {}", req.method(), req.uri().path());
                    Ok::<_, Infallible>(Response::new(Full::new(Bytes::from(body))))
                }
            })
        };
        let listener = bind(SocketAddr::from(([127, 0, 0, 1], 0))).await;
        let port = listener.local_addr().unwrap().port();
        let (stop, stopped) = tokio::sync::oneshot::channel::<()>();
        let server = tokio::spawn(serve(listener, None, svc, async {
            let _ = stopped.await;
        }));

        assert_eq!(get(port, "/ping").await, "GET /ping");

        let slow = tokio::spawn(get(port, "/slow"));
        started.notified().await;
        stop.send(()).unwrap();

        // The listener closes while the slow request is still in flight. Probes are
        // few enough to never fill the accept backlog of a listener left open.
        let mut closed = false;
        for _ in 0..50 {
            if tokio::net::TcpStream::connect(("127.0.0.1", port))
                .await
                .is_err()
            {
                closed = true;
                break;
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
        assert!(
            closed,
            "the listener must be closed while in-flight requests drain"
        );
        assert!(!server.is_finished(), "in-flight requests must be drained");

        release.notify_one();
        assert_eq!(slow.await.unwrap(), "GET /slow");
        tokio::time::timeout(Duration::from_secs(5), server)
            .await
            .expect("server did not stop after the shutdown signal")
            .unwrap();
    }
}
