//! HTTP clients used for every outgoing call.

use http_body_util::{BodyExt, Full};
use hyper::Response;
use hyper::body::{Body, Bytes, Incoming};
use hyper_rustls::HttpsConnector;
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::client::legacy::{Builder, Client};
use hyper_util::rt::{TokioExecutor, TokioTimer};
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls::{ClientConfig, RootCertStore};

/// Body of the requests built by otoroshictl itself (API calls, downloads).
pub type RequestBody = Full<Bytes>;

/// Client able to reach both `http://` and `https://` URLs.
pub type HttpsClient<B = RequestBody> = Client<HttpsConnector<HttpConnector>, B>;

/// Client restricted to plain `http://` URLs.
pub type HttpClient<B = RequestBody> = Client<HttpConnector, B>;

/// Client builder running on Tokio. The timer lets the pool close idle
/// connections, as the hyper 0.14 client did.
fn builder() -> Builder {
    let mut builder = Client::builder(TokioExecutor::new());
    builder.pool_timer(TokioTimer::new());
    builder
}

/// Plain HTTP client.
pub fn http_client<B>() -> HttpClient<B>
where
    B: Body + Send,
    B::Data: Send,
{
    builder().build_http()
}

/// HTTPS client trusting the certificate store of the operating system.
pub fn native_roots_client<B>() -> HttpsClient<B>
where
    B: Body + Send,
    B::Data: Send,
{
    let https = hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()
        .expect("no usable CA certificate found in the system trust store")
        .https_or_http()
        .enable_http1()
        .build();
    builder().build(https)
}

/// HTTPS client presenting a client certificate, trusting the Mozilla root
/// certificates embedded by `webpki-roots`.
pub fn mtls_client<B>(
    cert_chain: Vec<CertificateDer<'static>>,
    key: PrivateKeyDer<'static>,
) -> HttpsClient<B>
where
    B: Body + Send,
    B::Data: Send,
{
    let roots = RootCertStore {
        roots: webpki_roots::TLS_SERVER_ROOTS.to_vec(),
    };
    let tls = ClientConfig::builder()
        .with_root_certificates(roots)
        .with_client_auth_cert(cert_chain, key)
        .expect("invalid client certificate or private key");
    let https = hyper_rustls::HttpsConnectorBuilder::new()
        .with_tls_config(tls)
        .https_or_http()
        .enable_http1()
        .build();
    builder().build(https)
}

/// Read a whole response body into memory.
pub async fn read_body(response: Response<Incoming>) -> Result<Bytes, hyper::Error> {
    Ok(response.into_body().collect().await?.to_bytes())
}

/// Describe an error with its chain of causes, so that the reason of a failure
/// (DNS, TLS, socket...) is not hidden behind a generic message such as
/// `client error (Connect)`.
pub fn error_chain(err: &(dyn std::error::Error + 'static)) -> String {
    let mut message = err.to_string();
    let mut source = err.source();
    while let Some(cause) = source {
        message.push_str(": ");
        message.push_str(&cause.to_string());
        source = cause.source();
    }
    message
}
