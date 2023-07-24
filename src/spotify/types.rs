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

#[derive(Debug, Deserialize)]
pub struct RefreshToken {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
    pub expires_in: i64,
}
