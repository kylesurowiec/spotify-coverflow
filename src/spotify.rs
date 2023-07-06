use anyhow::Result;

use crate::os;
use crate::util;
use crate::util::Config;

const REDIRECT_URI: &str = "http://localhost:3000";
const SCOPES: [&str; 1] = ["user-read-playback-state"];
const SPOTIFY_URL: &str = "https://accounts.spotify.com/authorize";

pub fn start_auth_flow() -> Result<()> {
    let Config { client_id, .. } = util::get_config();
    let scopes = SCOPES.join(" ");
    let url = format!(
        "{SPOTIFY_URL}?response_type=code&client_id={client_id}&scope={scopes}&redirect_uri={REDIRECT_URI}"
    );

    os::open_url(url)?;

    Ok(())
}
