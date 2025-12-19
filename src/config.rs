use serde::Deserialize;
use config::{Config as ConfigLoader, File};
use crate::error::AppError;

#[derive(Clone, Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

impl Config {
    pub fn new() -> Result<Self, AppError> {
        let builder = ConfigLoader::builder()
            .add_source(File::with_name("config.yaml"));

        let config = builder
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to load configuration: {}", e))?;

        config.try_deserialize().map_err(|e| anyhow::anyhow!("Failed to parse configuration: {}", e).into())
    }
}
