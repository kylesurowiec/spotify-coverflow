use std::fs;
use std::io::{BufWriter, Write};

use anyhow::Result;
use serde::{Deserialize, Serialize};

const CONFIG_PATH: &str = "./config.json";

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Config {
    pub client_id: String,
    pub client_secret: String,
    pub token: Option<String>,
    pub refresh_token: Option<String>,
}

pub fn get() -> Result<Config> {
    let file = fs::read_to_string(CONFIG_PATH)?;
    let config = serde_json::from_str::<Config>(&file)?;

    Ok(config)
}

pub fn update_token(token: String) -> Result<Config> {
    let config = get()?;
    let config = Config {
        token: Some(token),
        ..config
    };

    save_to_disk(config.clone())?;

    Ok(config)
}

fn save_to_disk(config: Config) -> Result<()> {
    let file = fs::File::create(CONFIG_PATH)?;
    let mut writer = BufWriter::new(file);

    serde_json::to_writer(&mut writer, &config)?;
    writer.flush()?;

    Ok(())
}
