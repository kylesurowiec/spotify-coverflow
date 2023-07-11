use std::fs;
use std::io::prelude::*;

use anyhow::Result;
use serde::{Deserialize, Serialize};

const CONFIG_PATH: &str = "./config.json";

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Config {
    pub client_id: String,
    pub client_secret: String,
    pub oauth_token: String,
    pub oauth_refresh_token: String,
}
pub fn get() -> Result<Config> {
    let file = fs::read_to_string(CONFIG_PATH)?;
    let config = serde_json::from_str::<Config>(&file)?;

    Ok(config)
}

pub fn update(oauth_token: String, oauth_refresh_token: String) -> Result<Config> {
    let config = get()?;
    let config = Config {
        oauth_token,
        oauth_refresh_token,
        ..config
    };

    save_to_disk(config.clone())?;

    println!("{config:#?}");

    Ok(config)
}

fn save_to_disk(config: Config) -> Result<()> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(CONFIG_PATH)?;
    let bytes = &serde_json::to_vec(&config)?;

    file.write_all(&bytes)?;

    Ok(())
}
