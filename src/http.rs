use std::collections::HashMap;
use std::sync::OnceLock;

use anyhow::Result;
use reqwest::header::HeaderMap;
use reqwest::{Client, Method};
use serde::de::DeserializeOwned;

use crate::spotify;

static CLIENT: OnceLock<Client> = OnceLock::new();

fn client() -> &'static Client {
    CLIENT.get_or_init(|| Client::new())
}

pub type HttpMethod = Method;

#[derive(Default)]
pub struct HttpRequest {
    endpoint: String,
    method: HttpMethod,
    headers: HeaderMap,
    body: Option<serde_json::Value>,
    form: Option<HashMap<String, String>>,
    query: Option<HashMap<String, String>>,
}

impl HttpRequest {
    pub fn new(endpoint: &str) -> HttpRequest {
        HttpRequest {
            endpoint: endpoint.to_string(),
            ..HttpRequest::default()
        }
    }

    pub fn method(&mut self, method: HttpMethod) -> &mut HttpRequest {
        self.method = method;
        self
    }

    pub fn header(&mut self, key: &'static str, value: &str) -> &mut HttpRequest {
        self.headers.insert(key, value.to_string().parse().unwrap());
        self
    }

    pub fn body(&mut self, body: serde_json::Value) -> &mut HttpRequest {
        self.body = Some(body);
        self
    }

    pub fn query(&mut self, query: HashMap<String, String>) -> &mut HttpRequest {
        self.query = Some(query);
        self
    }

    pub async fn send<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}{}", spotify::BASE_URL, self.endpoint);
        let req = client()
            .request(self.method.clone(), &url)
            .headers(self.headers.clone());

        // if self.form.is_some() {
        //     req.form(&self.form.clone());
        // }

        let res = req.send().await?;
        let json = res.json::<T>().await.unwrap();

        Ok(json)
    }
}
