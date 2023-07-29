pub mod auth;
mod types;
pub use types::*;

use anyhow::Result;

use crate::http::{HttpMethod, HttpRequest};

pub const BASE_URL: &str = "https://api.spotify.com/v1";

pub async fn get_current_song() -> Result<Player> {
    let request = HttpRequest::new("/me/player/currently-playing")
        .authenticated()
        .method(HttpMethod::GET)
        .send::<Player>()
        .await?;

    Ok(request)
}
