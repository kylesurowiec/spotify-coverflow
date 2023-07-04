use std::fs;
use std::io::prelude::*;
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

static CONFIG: OnceLock<Config> = OnceLock::new();
static CONFIG_PATH: &str = "./config.json";

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config {
    client_id: String,
    client_secret: String,
}

pub fn get_config() -> &'static Config {
    CONFIG.get_or_init(|| {
        let file = fs::read_to_string(CONFIG_PATH);
        let config = match file {
            | Ok(file) => {
                serde_json::from_str::<Config>(&file).expect("Failed to parse config.json")
            },
            | Err(_) => {
                let mut file = fs::File::create(CONFIG_PATH).expect("Failed to create config.json");

                file.write_all(
                    &serde_json::to_vec(&Config::default()).expect("Failed to serialize Config"),
                )
                .expect("Failed to write to config.json");

                panic!("Coverflow started without a config.json");
            },
        };
        Config {
            client_id: config.client_id,
            client_secret: config.client_secret,
        }
    })
}
