use base64::{Engine as _, engine::general_purpose};
use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::{connect_async, tungstenite::Message};

#[derive(Clone)]
pub struct UdpTunnelOpts {
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

pub struct UdpTunnel {}

impl UdpTunnel {
    pub async fn start(opts: UdpTunnelOpts) -> () {
        let listen_addr = format!("{}:{}", opts.local_host, opts.local_port);
        let remote_info = match (&opts.remote_host, opts.remote_port) {
            (Some(rh), Some(rp)) => format!("{}:{}", rh, rp),
            (Some(rh), None) => rh.clone(),
            (None, Some(rp)) => format!(":{}", rp),
            (None, None) => "(defined in otoroshi target)".to_string(),
        };
        info!(
            "UDP tunnel listening on {} → {} via Otoroshi ({})",
            listen_addr, remote_info, opts.host
        );

        // Bind the UDP socket once; reuse it across WebSocket reconnections
        let udp_socket = match UdpSocket::bind(&listen_addr).await {
            Ok(s) => Arc::new(s),
            Err(e) => {
                error!("Failed to bind UDP socket to {}: {}", listen_addr, e);
                std::process::exit(-1);
            }
        };

        info!("UDP tunnel ready, waiting for datagrams ...");
        info!("");

        loop {
            info!("Connecting WebSocket tunnel ...");
            match Self::run_tunnel(opts.clone(), udp_socket.clone()).await {
                Ok(_) => debug!("WebSocket tunnel closed"),
                Err(e) => error!("WebSocket tunnel error: {}", e),
            }
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        }
    }

    async fn run_tunnel(
        opts: UdpTunnelOpts,
        udp_socket: Arc<UdpSocket>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let scheme = if opts.tls { "wss" } else { "ws" };

        // Build WebSocket URL with routing query parameters
        let mut url_str = format!(
            "{}://{}/.well-known/otoroshi/tunnel?transport=udp",
            scheme, opts.host
        );
        if let Some(ref rh) = opts.remote_host {
            url_str.push_str(&format!("&remoteHost={}", rh));
        }
        if let Some(rp) = opts.remote_port {
            url_str.push_str(&format!("&remotePort={}", rp));
        }

        if opts.access_type == "session" {
            if let Some(token) = &opts.session_token {
                url_str.push_str(&format!("&pappsToken={}", token));
            }
        }

        debug!("Connecting WebSocket to: {}", url_str);

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
            _ => {}
        }

        let (ws_stream, _) = connect_async(request).await?;
        info!("WebSocket tunnel connected");

        let (ws_write, mut ws_read) = ws_stream.split();
        let ws_write = Arc::new(Mutex::new(ws_write));

        // UDP → WebSocket: each received datagram is JSON-encoded and sent as a binary frame
        //   { "address": "<src_ip>", "port": <src_port>, "data": "<base64_payload>" }
        let udp_recv = udp_socket.clone();
        let ws_write_udp = ws_write.clone();
        let udp_to_ws = async move {
            let mut buf = [0u8; 65535];
            loop {
                match udp_recv.recv_from(&mut buf).await {
                    Ok((n, src_addr)) => {
                        let data = general_purpose::STANDARD.encode(&buf[..n]);
                        let json = serde_json::json!({
                            "address": src_addr.ip().to_string(),
                            "port":    src_addr.port(),
                            "data":    data,
                        });
                        let bytes = match serde_json::to_vec(&json) {
                            Ok(b) => b,
                            Err(e) => {
                                debug!("JSON encode error: {}", e);
                                continue;
                            }
                        };
                        let mut ws = ws_write_udp.lock().await;
                        if ws.send(Message::Binary(bytes.into())).await.is_err() {
                            break;
                        }
                    }
                    Err(e) => {
                        debug!("UDP recv error: {}", e);
                        break;
                    }
                }
            }
        };

        // WebSocket → UDP: each binary frame is a JSON packet; decode and send back to origin
        //   { "address": "<dest_ip>", "port": <dest_port>, "data": "<base64_payload>" }
        let udp_send = udp_socket.clone();
        let ws_to_udp = async move {
            while let Some(msg) = ws_read.next().await {
                match msg {
                    Ok(Message::Binary(data)) => {
                        if let Ok(json) = serde_json::from_slice::<serde_json::Value>(&data) {
                            let address = json.get("address").and_then(|v| v.as_str());
                            let port = json.get("port").and_then(|v| v.as_u64());
                            let encoded = json.get("data").and_then(|v| v.as_str());

                            if let (Some(address), Some(port), Some(encoded)) =
                                (address, port, encoded)
                            {
                                if let Ok(payload) = general_purpose::STANDARD.decode(encoded) {
                                    let dest = format!("{}:{}", address, port);
                                    if let Ok(dest_addr) = dest.parse::<SocketAddr>() {
                                        if let Err(e) = udp_send.send_to(&payload, dest_addr).await
                                        {
                                            debug!("UDP send error: {}", e);
                                        }
                                    }
                                }
                            }
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

        futures::future::select(Box::pin(udp_to_ws), Box::pin(ws_to_udp)).await;

        Ok(())
    }
}
