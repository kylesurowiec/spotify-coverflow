use std::fs;
use std::io::prelude::*;
use std::sync::OnceLock;

use anyhow::Result;
use serde::{Deserialize, Serialize};

static CONFIG: OnceLock<Config> = OnceLock::new();
const CONFIG_PATH: &str = "./config.json";

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Config {
    pub client_id: String,
    pub client_secret: String,
    pub oauth_token: String,
    pub oauth_refresh_token: String,
}

pub fn update(oauth_token: String, oauth_refresh_token: String) -> Result<Config> {
    let config = from_cache().to_owned();
    let config = Config {
        oauth_token,
        oauth_refresh_token,
        ..config
    };

    save_to_disk(config.clone())?;
    rehydrate_cache(config.clone());

    Ok(config)
}

pub fn from_cache() -> &'static Config {
    CONFIG.get_or_init(|| {
        match from_disk() {
            | Ok(config) => Config {
                client_id: config.client_id,
                client_secret: config.client_secret,
                ..Config::default()
            },
            | Err(_) => {
                save_to_disk(Config::default()).expect(
                    "Failed to create config.json. Please create this file manually at the project root"
                );
                panic!("Coverflow started without a config.json");
            },
        }
    })
}

fn from_disk() -> Result<Config> {
    let file = fs::read_to_string(CONFIG_PATH)?;
    let config = serde_json::from_str::<Config>(&file)?;

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

fn rehydrate_cache(config: Config) {
    let update = CONFIG.set(config);
    if let Err(_) = update {
        println!("Failed to update cached config")
    };
}
