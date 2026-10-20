use hyper::{Request, body::Bytes};

use crate::http_utils::client::{
    RequestBody, error_chain, http_client, native_roots_client, read_body,
};

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
        let req: Request<RequestBody> = Request::builder()
            .method("GET")
            .uri(url)
            .header("accept", "application/json")
            .header("authorization", format!("Bearer {}", token))
            .body(RequestBody::default())
            .unwrap();

        let resp_result = if tls {
            native_roots_client().request(req).await
        } else {
            http_client().request(req).await
        };
        match resp_result {
            Err(err) => std::result::Result::Err(format!(
                "error while fetching content at '{}': \n\n{}",
                url,
                error_chain(&err)
            )),
            Ok(resp) => match resp.status().as_u16() {
                200 => {
                    let content_type: String = resp
                        .headers()
                        .get("content-type")
                        .map(|v| v.to_str().unwrap().to_string())
                        .unwrap_or("application/json".to_string());
                    let body_bytes = read_body(resp).await.unwrap();
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
                }
                code => {
                    let body_bytes = read_body(resp).await.unwrap();
                    let error_msg = String::from_utf8(body_bytes.to_vec()).unwrap_or_default();
                    std::result::Result::Err(format!(
                        "bad response status {} while fetching content at '{}': {}",
                        code, url, error_msg
                    ))
                }
            },
        }
    }

    pub async fn get(url: &String) -> Result<HttpContent, String> {
        let tls = url.starts_with("https://");
        let req: Request<RequestBody> = Request::builder()
            .method("GET")
            .uri(url)
            .header("accept", "application/yaml, application/json".to_string())
            .body(RequestBody::default())
            .unwrap();

        let resp_result = if tls {
            native_roots_client().request(req).await
        } else {
            http_client().request(req).await
        };
        match resp_result {
            Err(err) => std::result::Result::Err(format!(
                "error while fetching content at '{}': \n\n{}",
                url,
                error_chain(&err)
            )),
            Ok(resp) => match resp.status().as_u16() {
                200 => {
                    let content_type: String = resp
                        .headers()
                        .get("content-type")
                        .map(|v| v.to_str().unwrap().to_string())
                        .unwrap_or("application/yaml".to_string());
                    let body_bytes = read_body(resp).await.unwrap();
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
                }
                code => std::result::Result::Err(format!(
                    "bad response status {} while fetching content at '{}'",
                    code, url
                )),
            },
        }
    }
}
