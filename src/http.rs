use std::collections::HashMap;
use std::fmt::Debug;
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
    base_url: Option<String>,
    endpoint: String,
    method: HttpMethod,
    headers: HeaderMap,
    basic_auth: Option<(String, String)>,
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

    pub fn base_url(&mut self, url: &str) -> &mut HttpRequest {
        self.base_url = Some(url.to_string());
        self
    }

    pub fn basic_auth(&mut self, username: &str, password: &str) -> &mut HttpRequest {
        self.basic_auth = Some((username.to_string(), password.to_string()));
        self
    }

    pub fn method(&mut self, method: HttpMethod) -> &mut HttpRequest {
        self.method = method;
        self
    }

    pub fn header(&mut self, key: &'static str, value: &str) -> &mut HttpRequest {
        self.headers.insert(key, value.to_string().parse().unwrap());
        self
    }

    pub fn query(&mut self, query: HashMap<String, String>) -> &mut HttpRequest {
        self.query = Some(query);
        self
    }

    pub fn body(&mut self, body: serde_json::Value) -> &mut HttpRequest {
        self.body = Some(body);
        self
    }

    pub fn form_field(&mut self, key: &str, value: &str) -> &mut HttpRequest {
        match self.form {
            | Some(ref mut form) => {
                form.insert(key.to_string(), value.to_string());
            },
            | None => {
                self.form = Some(HashMap::new());
                self.form
                    .as_mut()
                    .unwrap()
                    .insert(key.to_string(), value.to_string());
            },
        };

        self
    }

    pub async fn send<T>(&self) -> Result<T>
    where
        T: Debug + DeserializeOwned,
    {
        let base = if self.base_url.is_some() {
            self.base_url.as_ref().unwrap()
        } else {
            spotify::BASE_URL
        };

        let url = format!("{}{}", base, self.endpoint);
        let mut req = client()
            .request(self.method.clone(), &url)
            .headers(self.headers.clone());

        if self.basic_auth.is_some() {
            let kv = self.basic_auth.as_ref().unwrap();
            req = req.basic_auth(kv.0.to_string(), Some(kv.1.to_string()))
        }
        if self.form.is_some() {
            req = req.form(&self.form.clone());
        }

        let res = req.send().await?;
        let json = res.json::<T>().await.unwrap();

        Ok(json)
    }
}
