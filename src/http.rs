use std::collections::HashMap;

const BASE_URL: &'static str = "https://api.spotify.com/v1";

#[derive(Clone, Debug, Default)]
pub enum HttpMethod {
    #[default]
    GET,
    #[allow(unused)]
    DELETE,
    #[allow(unused)]
    PATCH,
    #[allow(unused)]
    POST,
    #[allow(unused)]
    PUT,
}

#[derive(Default)]
pub struct HttpRequest<'a> {
    endpoint: &'a str,
    method: HttpMethod,
    body: Option<serde_json::Value>,
    query: Option<HashMap<String, String>>,
}

impl<'a> HttpRequest<'a> {
    pub fn new(endpoint: &str) -> HttpRequest {
        HttpRequest {
            endpoint,
            method: HttpMethod::default(),
            body: None,
            query: None,
        }
    }

    pub fn method(&mut self, method: HttpMethod) -> &'a HttpRequest {
        self.method = method;
        self
    }

    pub fn body(&mut self, body: serde_json::Value) -> &'a HttpRequest {
        self.body = Some(body);
        self
    }

    pub fn query(&mut self, query: Option<HashMap<String, String>>) -> &'a HttpRequest {
        self
    }
}
