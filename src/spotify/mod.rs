mod types;
pub use types::*;

use anyhow::Result;

use crate::config;
use crate::http::{HttpMethod, HttpRequest};
use crate::os;

const AUTH_URL: &str = "https://accounts.spotify.com";
pub const BASE_URL: &str = "https://api.spotify.com/api";
const REDIRECT_URI: &str = "http://localhost:3000/callback";
const SCOPES: [&str; 1] = ["user-read-playback-state"];

pub fn prompt_auth_flow() -> Result<()> {
    let config = config::get()?;
    let scopes = SCOPES.join(" ");
    let url = format!(
        "{AUTH_URL}/authorize?response_type=code&client_id={}&scope={scopes}&redirect_uri={REDIRECT_URI}",
        config.client_id,
    );

    os::open_url(url)?;

    Ok(())
}

pub async fn get_oauth_token(code: &str) -> Result<OAuth> {
    let config = config::get()?;

    let request = HttpRequest::new("/api/token")
        .base_url(AUTH_URL)
        .method(HttpMethod::POST)
        .basic_auth(&config.client_id, &config.client_secret)
        .form_field("code", code)
        .form_field("redirect_uri", REDIRECT_URI)
        .form_field("grant_type", "authorization_code")
        .send::<OAuth>()
        .await?;

    Ok(request)
}

pub async fn get_new_token(refresh_token: &str) -> Result<RefreshToken> {
    let config = config::get()?;

    let request = HttpRequest::new("/api/token")
        .base_url(AUTH_URL)
        .method(HttpMethod::POST)
        .basic_auth(&config.client_id, &config.client_secret)
        .form_field("refresh_token", refresh_token)
        .form_field("grant_type", "refresh_token")
        .send::<RefreshToken>()
        .await?;

    Ok(request)
}
