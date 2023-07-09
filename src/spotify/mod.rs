mod types;
pub use types::*;

use anyhow::Result;
use base64::engine::general_purpose::STANDARD;
use base64::engine::Engine;

use crate::config;
use crate::http::{HttpMethod, HttpRequest};
use crate::os;

const AUTH_URL: &str = "https://accounts.spotify.com/authorize";
const REDIRECT_URI: &str = "http://localhost:3000/callback";
const SCOPES: [&str; 1] = ["user-read-playback-state"];

pub const BASE_URL: &str = "https://api.spotify.com/api";

pub async fn prompt_auth_flow() -> Result<()> {
    let config = config::from_cache();
    let scopes = SCOPES.join(" ");
    let url = format!(
        "{AUTH_URL}?response_type=code&client_id={}&scope={scopes}&redirect_uri={REDIRECT_URI}",
        config.client_id,
    );

    os::open_url(url)?;

    Ok(())
}

pub async fn get_oauth_token() -> Result<OAuth> {
    let config = config::from_cache();
    let encoded_pair = STANDARD.encode(format!("{}:{}", config.client_id, config.client_secret));
    let header_value = format!("Basic {}", encoded_pair);

    let request = HttpRequest::new("/token")
        .method(HttpMethod::POST)
        .header("Authorization", &header_value)
        .send::<OAuth>()
        .await?;

    Ok(request)
}
