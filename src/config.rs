use std::fs;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub database: String,
    pub username: String,
    pub password: String,
    pub sql: String
}

impl Config {
    pub fn read_config(config : &str) -> Result<Config> {
        let config_str = fs::read_to_string(config).unwrap();
        let config : Config = serde_json::from_str(&config_str)?;
        Ok(config)
    }
}