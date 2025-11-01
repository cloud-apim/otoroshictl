use hyper::{body::Bytes, Request, Client};

pub enum HttpContentKind {
    JSON,
    YAML,
}

pub struct HttpContent {
    pub kind: HttpContentKind,
    pub content: Bytes,
}

pub struct Http {}

impl Http {

    pub async fn get_with_bearer(url: &String, token: &String) -> Result<HttpContent, String> {
        let tls = url.starts_with("https://");
        let req: Request<hyper::Body> = Request::builder()
            .method("GET")
            .uri(url)
            .header("accept", "application/json")
            .header("authorization", format!("Bearer {}", token))
            .body(hyper::Body::empty())
            .unwrap();
            
        let resp_result = if tls {
            let https = hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_or_http()
                .enable_http1()
                .build();
            let client = Client::builder().build::<_, hyper::Body>(https);
            client.request(req).await
        } else {
            let client = Client::new();
            client.request(req).await
        };
        match resp_result {
            Err(err) => {
                std::result::Result::Err(format!("error while fetching content at '{}': \n\n{}", url, err))
            },
            Ok(resp) => {
                match resp.status().as_u16() {
                    200 => {
                        let content_type: String = resp.headers().get("content-type").map(|v| v.to_str().unwrap().to_string()).unwrap_or("application/json".to_string());
                        let body_bytes = hyper::body::to_bytes(resp).await.unwrap();
                        if content_type.contains("json") {
                            Ok(HttpContent {
                                kind: HttpContentKind::JSON,
                                content: body_bytes,
                            })
                        } else {
                            Ok(HttpContent {
                                kind: HttpContentKind::YAML,
                                content: body_bytes,
                            })
                        }
                    },
                    code => {
                        let body_bytes = hyper::body::to_bytes(resp).await.unwrap();
                        let error_msg = String::from_utf8(body_bytes.to_vec()).unwrap_or_default();
                        std::result::Result::Err(format!("bad response status {} while fetching content at '{}': {}", code, url, error_msg))
                    }
                }
            }
        }
    }

    pub async fn get(url: &String) -> Result<HttpContent, String> {
        let tls = url.starts_with("https://");
        let req: Request<hyper::Body> = Request::builder()
            .method("GET")
            .uri(url)
            .header("accept", "application/yaml, application/json".to_string())
            .body(hyper::Body::empty())
            .unwrap();
            
        let resp_result = if tls {
            let https = hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_or_http()
                .enable_http1()
                .build();
            let client = Client::builder().build::<_, hyper::Body>(https);
            client.request(req).await
        } else {
            let client = Client::new();
            client.request(req).await
        };
        match resp_result {
            Err(err) => {
                std::result::Result::Err(format!("error while fetching content at '{}': \n\n{}", url, err))
            },
            Ok(resp) => {
                match resp.status().as_u16() {
                    200 => {
                        let content_type: String = resp.headers().get("content-type").map(|v| v.to_str().unwrap().to_string()).unwrap_or("application/yaml".to_string());
                        let body_bytes = hyper::body::to_bytes(resp).await.unwrap();
                        if content_type.contains("json") {
                            Ok(HttpContent {
                                kind: HttpContentKind::JSON,
                                content: body_bytes,
                            })
                        } else {
                            Ok(HttpContent {
                                kind: HttpContentKind::YAML,
                                content: body_bytes,
                            })
                        }
                    },
                    code => {
                        std::result::Result::Err(format!("bad response status {} while fetching content at '{}'", code, url))
                    }
                }
            }
        }
    }
}