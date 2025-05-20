use config::{Config, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub tracker_api_url: String,
}

impl Settings {
    pub fn load() -> Self {
        let mut cfg = Config::builder()
            .add_source(config::File::with_name("config.toml"))
            .add_source(Environment::with_prefix("APP")) 
            .build()
            .expect("Failed to build config");

        
        let environment = std::env::var("APP_ENV").unwrap_or_else(|_| "default".to_string());
        
        cfg.get::<Settings>(&environment)
            .expect("Failed to load profile from config")
    }
}