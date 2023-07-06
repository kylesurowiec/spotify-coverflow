use std::collections::HashMap;

const BASE_URL: &str = "https://api.spotify.com/v1";

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

impl HttpRequest<'_> {
    pub fn new(endpoint: &str) -> HttpRequest {
        HttpRequest {
            endpoint,
            method: HttpMethod::default(),
            body: None,
            query: None,
        }
    }

    pub fn method(&mut self, method: HttpMethod) -> &HttpRequest {
        self.method = method;
        self
    }

    pub fn body(&mut self, body: serde_json::Value) -> &HttpRequest {
        self.body = Some(body);
        self
    }

    pub fn query(&mut self, query: HashMap<String, String>) -> &HttpRequest {
        self.query = Some(query);
        self
    }
}
