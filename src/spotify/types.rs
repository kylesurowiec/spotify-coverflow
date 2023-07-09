use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OAuth {
    pub oauth_token: String,
    pub oauth_refresh_token: String,
}
