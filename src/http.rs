use std::collections::HashMap;
use std::sync::OnceLock;

use reqwest::Client;

static BASE_URL: &'static str = "https://";
static CLIENT: OnceLock<Client> = OnceLock::new();

#[derive(Default)]
pub enum HttpMethod {
    #[default]
    GET,
    DELETE,
    PATCH,
    POST,
    PUT,
}

#[derive(Default)]
pub struct HttpRequest<'a> {
    endpoint: &'a str,
    method: HttpMethod,
    body: Option<HashMap<String, String>>,
}

impl<'a> HttpRequest<'a> {
    pub fn new(endpoint: &str) -> HttpRequest {
        HttpRequest {
            endpoint,
            method: HttpMethod::default(),
            body: None,
        }
    }
}
