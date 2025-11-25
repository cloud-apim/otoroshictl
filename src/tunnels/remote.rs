use crate::cli::cliopts::CliOpts;
use crate::utils::otoroshi::Otoroshi;
use serde::{Serialize, Deserialize};

use std::collections::HashMap;
use hyper::http::request::Builder;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{Result, Message},
};
use url::Url;
use base64::{Engine as _, engine::general_purpose};
use hyper::{Request, Client};
use std::time::Instant;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RemoteTunnelCommandOpts {
    pub local_host: String,
    pub local_port: i32,
    pub local_tls: bool,
    pub expose: bool,
    pub tls: bool,
    pub remote_domain: Option<String>,
    pub remote_subdomain: Option<String>,
    pub tunnel: String,
}

pub struct RemoteTunnelCommand {}

impl RemoteTunnelCommand {

    pub async fn start(cli_opts: CliOpts, tunnel_opts: RemoteTunnelCommandOpts) -> () {

        let infos = Otoroshi::get_remote_tunnels_infos(cli_opts.clone()).await.unwrap();

        if tunnel_opts.expose {
            let domain = Otoroshi::maybe_expose_local_process(tunnel_opts.clone(), cli_opts.clone(), infos.clone()).await;
            let port = if tunnel_opts.tls {
                if infos.exposed_port_https == 443 {
                    String::from("")
                } else {
                    format!(":{}", infos.exposed_port_https)
                }
            } else if infos.exposed_port_https == 443 {
                String::from("")
            } else {
                format!(":{}", infos.exposed_port_http)
            };
            if tunnel_opts.tls {
                info!("");
                info!("your service will be available at: https://{}{}", domain, port);
                info!("");
            } else {
                info!("");
                info!("your service will be available at: http://{}{}", domain, port);
                info!("");
            }
        }
        loop {
            info!("connecting the tunnel ...");
            match Tunnel::connect(cli_opts.clone(), tunnel_opts.clone()).await {
                Ok(_) => debug!("connection closed"),
                Err(e) => debug!("connection closed with error: {}", e),
            };
            std::thread::sleep(std::time::Duration::from_secs(2));
        }
    }
}


pub struct Tunnel {}

impl Tunnel {
    pub async fn connect(cli_opts: CliOpts, opts: RemoteTunnelCommandOpts) -> Result<()> {
        let config = Otoroshi::get_connection_config(cli_opts).await;
        let tunnel_id = opts.tunnel;
        let scheme = if config.tls {
            "wss"
        } else {
            "ws"
        };
        let host = config.host;
        let client = Client::new();
        let credentials = general_purpose::STANDARD_NO_PAD.encode(format!("{}:{}", config.cid, config.csec));
        let url_raw = format!("{}://{}/api/tunnels/register?tunnel_id={}&basic_auth={}&pong_ping=true", scheme, host, tunnel_id, credentials);
        // debug!("url_raw {} {}", opts.tls, url_raw);
        let url =  Url::parse(url_raw.as_str()).expect("Bad URL");
        // TODO: connect tls
        let (ws_stream, _) = Box::leak(Box::new(connect_async(url).await?));

        info!("connection done !");
        info!("");
        
        while let Some(msg) = ws_stream.next().await {
            let msg = msg?;
            if msg.is_binary() {
                let json: serde_json::Value = serde_json::from_slice(&msg.into_data()).unwrap();
                let msg_type = json.get("type").unwrap().as_str().unwrap();
                // info!("{}", msg_type);
                if msg_type == "request" {
                    // TODO: cookies
                    let start = Instant::now();
                    let request_id = json.get("request_id").unwrap().as_str().unwrap();
                    let addr = json.get("addr").unwrap().as_str().unwrap();
                    let version = json.get("version").unwrap().as_str().unwrap();
                    let path = json.get("path").unwrap().as_str().unwrap();
                    let method = json.get("method").unwrap().as_str().unwrap();
                    let uri = json.get("url").unwrap().as_str().unwrap();
                    let headers = json.get("headers").unwrap().as_object().unwrap();
                    // debug!("headers: {:?}", headers);
                    let mut builder: Builder = Request::builder()
                        .method(method)
                        .uri(uri);
                    for header in headers.iter() {
                        builder = builder.header(header.0, header.1.as_str().unwrap());
                    }
                    let maybe_body = json.get("body");
                    let req: Request<hyper::Body> = if maybe_body.is_some() {
                        let body_bytes = general_purpose::STANDARD_NO_PAD.decode(maybe_body.unwrap().as_str().unwrap()).unwrap();
                        builder.body(hyper::Body::from(body_bytes)).unwrap()
                    } else {
                        builder.body(hyper::Body::empty()).unwrap()
                    };
                    let resp = client.request(req).await.unwrap();
                    let status = resp.status();
                    let resp_headers = resp.headers().clone();
                    let mut resp_headers_map = HashMap::new();
                    for (key, value) in resp_headers.iter() {
                        resp_headers_map.insert(key.as_str(), value.to_str().unwrap());
                    }
                    let body_bytes = hyper::body::to_bytes(resp).await.unwrap();
                    let body_str = general_purpose::STANDARD_NO_PAD.encode(body_bytes);
                    let resp_json = serde_json::json!({
                        "status": status.as_u16(),
                        "headers": resp_headers_map,
                        "body": body_str,
                        "request_id": request_id,
                        "type": "response",
                    });
                    let resp_json_bin = serde_json::to_vec(&resp_json).unwrap();
                    let elasped = start.elapsed();
                    info!("{} - - [{:?}] \"{} {} {}\" {} {}", addr, chrono::offset::Utc::now(), method, path, version, status.as_u16(), elasped.as_millis());
                    match ws_stream.send(Message::Binary(resp_json_bin)).await {
                        Ok(_) => (),
                        Err(e) => debug!("error while sending response: {}", e)
                    };
                } else if msg_type == "pong" {
                    let json = serde_json::json!({
                        "tunnel_id": tunnel_id.clone(), 
                        "type": "ping"
                    });
                    let bytes = serde_json::to_vec(&json).unwrap();
                    match ws_stream.send(Message::Binary(bytes)).await {
                        Ok(_) => (),
                        Err(e) => debug!("error while sending pong: {}", e)
                    };
                    let local_scheme = if opts.tls {
                        "https"
                    } else {
                        "http"
                    };
                    let json1 = serde_json::json!({
                        "tunnel_id": tunnel_id.clone(), 
                        "type": "tunnel_meta",
                        "name": tunnel_id.clone(),
                        "routes": serde_json::json!([
                            serde_json::json!({
                                "id": tunnel_id.clone(),
                                "name": tunnel_id.clone(),
                                "frontend": format!("{}://{}:{}", local_scheme, opts.local_host, opts.local_port)
                            })

                        ])
                    });
                    let bytes1 = serde_json::to_vec(&json1).unwrap();
                    match ws_stream.send(Message::Binary(bytes1)).await {
                        Ok(_) => (),
                        Err(e) => debug!("error while sending meta: {}", e)
                    };
                }
            }
        }
        Ok(())
    }
}