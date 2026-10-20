use base64::{Engine as _, engine::general_purpose};
use futures_util::{SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::{connect_async, tungstenite::Message};

#[derive(Clone)]
pub struct TcpTunnelOpts {
    pub host: String,
    pub tls: bool,
    pub local_host: String,
    pub local_port: u16,
    pub remote_host: Option<String>,
    pub remote_port: Option<u16>,
    pub access_type: String,
    pub apikey_client_id: Option<String>,
    pub apikey_client_secret: Option<String>,
    pub bearer_token: Option<String>,
    pub session_token: Option<String>,
}

pub struct TcpTunnel {}

impl TcpTunnel {
    pub async fn start(opts: TcpTunnelOpts) -> () {
        let listen_addr = format!("{}:{}", opts.local_host, opts.local_port);
        let remote_info = match (&opts.remote_host, opts.remote_port) {
            (Some(rh), Some(rp)) => format!("{}:{}", rh, rp),
            (Some(rh), None) => rh.clone(),
            (None, Some(rp)) => format!(":{}", rp),
            (None, None) => "(defined in otoroshi target)".to_string(),
        };
        info!(
            "TCP tunnel listening on {} → {} via Otoroshi ({})",
            listen_addr, remote_info, opts.host
        );

        let listener = match TcpListener::bind(&listen_addr).await {
            Ok(l) => l,
            Err(e) => {
                error!("Failed to bind to {}: {}", listen_addr, e);
                std::process::exit(-1);
            }
        };

        info!("TCP tunnel ready, waiting for connections ...");
        info!("");

        while let Ok((inbound, peer_addr)) = listener.accept().await {
            info!("New TCP connection from: {}", peer_addr);
            let opts = opts.clone();
            tokio::spawn(async move {
                if let Err(e) = Self::handle_connection(opts, inbound).await {
                    error!("Connection error: {}", e);
                }
            });
        }
    }

    async fn handle_connection(
        opts: TcpTunnelOpts,
        tcp_stream: TcpStream,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let scheme = if opts.tls { "wss" } else { "ws" };

        // Build WebSocket URL with routing query parameters
        let mut url_str = format!(
            "{}://{}/.well-known/otoroshi/tunnel?transport=tcp",
            scheme, opts.host
        );
        if let Some(ref rh) = opts.remote_host {
            url_str.push_str(&format!("&remoteHost={}", rh));
        }
        if let Some(rp) = opts.remote_port {
            url_str.push_str(&format!("&remotePort={}", rp));
        }

        // Session token goes into the query string
        if opts.access_type == "session" {
            if let Some(token) = &opts.session_token {
                url_str.push_str(&format!("&pappsToken={}", token));
            }
        }

        debug!("Connecting WebSocket to: {}", url_str);

        // Build the WebSocket upgrade request with authentication headers
        let mut request = url_str.into_client_request()?;

        match opts.access_type.as_str() {
            "apikey" => {
                if let (Some(cid), Some(csec)) = (
                    opts.apikey_client_id.as_ref(),
                    opts.apikey_client_secret.as_ref(),
                ) {
                    let credentials = general_purpose::STANDARD.encode(format!("{}:{}", cid, csec));
                    request
                        .headers_mut()
                        .insert("Authorization", format!("Basic {}", credentials).parse()?);
                }
            }
            "bearer" | "jwt" => {
                if let Some(token) = &opts.bearer_token {
                    request
                        .headers_mut()
                        .insert("Authorization", format!("Bearer {}", token).parse()?);
                }
            }
            _ => {} // "public" or "session" (token already in URL)
        }

        // Connect to Otoroshi via WebSocket
        let (ws_stream, _) = connect_async(request).await?;
        let (mut ws_write, mut ws_read) = ws_stream.split();
        let (mut tcp_read, mut tcp_write) = tcp_stream.into_split();

        // TCP → WebSocket: forward raw TCP bytes as binary WebSocket frames
        let tcp_to_ws = async move {
            let mut buf = [0u8; 32768];
            loop {
                match tcp_read.read(&mut buf).await {
                    Ok(0) => break,
                    Ok(n) => {
                        if ws_write
                            .send(Message::Binary(buf[..n].to_vec().into()))
                            .await
                            .is_err()
                        {
                            break;
                        }
                    }
                    Err(e) => {
                        debug!("TCP read error: {}", e);
                        break;
                    }
                }
            }
        };

        // WebSocket → TCP: forward binary WebSocket frames back to TCP
        let ws_to_tcp = async move {
            while let Some(msg) = ws_read.next().await {
                match msg {
                    Ok(Message::Binary(data)) => {
                        if tcp_write.write_all(&data).await.is_err() {
                            break;
                        }
                    }
                    Ok(Message::Close(_)) => break,
                    Err(e) => {
                        debug!("WebSocket read error: {}", e);
                        break;
                    }
                    _ => {}
                }
            }
        };

        // Run both directions concurrently; stop when either side closes
        futures::future::select(Box::pin(tcp_to_ws), Box::pin(ws_to_tcp)).await;

        Ok(())
    }
}
