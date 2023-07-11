use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AuthCode {
    pub code: String,
}

#[derive(Debug, Deserialize)]
pub struct OAuth {
    pub access_token: String,
    pub refresh_token: String,
}
