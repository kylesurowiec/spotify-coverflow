use std::fs;
use std::io::prelude::*;
use std::sync::OnceLock;

use anyhow::Result;
use serde::{Deserialize, Serialize};

static CONFIG: OnceLock<Config> = OnceLock::new();
const CONFIG_PATH: &str = "./config.json";

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config {
    pub client_id: String,
    pub client_secret: String,
}

pub fn get_config() -> &'static Config {
    CONFIG.get_or_init(|| {
        match read_config() {
            | Ok(config) => Config {
                client_id: config.client_id,
                client_secret: config.client_secret,
            },
            | Err(_) => {
                create_config().expect("Failed to automatically create config.json. Please create this file manually at the project root");
                panic!("Coverflow started without a config.json");
            },
        }
    })
}

fn read_config() -> Result<Config> {
    let file = fs::read_to_string(CONFIG_PATH)?;
    let config = serde_json::from_str::<Config>(&file)?;

    Ok(config)
}

fn create_config() -> Result<()> {
    let mut file = fs::File::create(CONFIG_PATH)?;
    let bytes = &serde_json::to_vec(&Config::default())?;

    file.write_all(&bytes)?;

    Ok(())
}
