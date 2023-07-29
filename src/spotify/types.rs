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

#[derive(Debug, Deserialize)]
pub struct Player {
    pub item: Item,
}

#[derive(Debug, Deserialize)]
pub struct Item {
    pub name: String,
    pub album: Album,
    pub artists: Vec<Artist>,
}

#[derive(Debug, Deserialize)]
pub struct Album {
    pub name: String,
    pub images: Vec<Image>,
}

#[derive(Debug, Deserialize)]
pub struct Artist {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Image {
    pub url: String,
    pub height: i32,
    pub width: i32,
}
