use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
    pub timeout: u64,
}

const CONFIG_PATH: &str = "server_config.toml";

impl ServerConfig {
    pub fn toml() -> Self {
        log::info!("Loading server config from `{}`", CONFIG_PATH);
        let file = std::fs::read_to_string(CONFIG_PATH).expect("Failed to read file");
        toml::from_str::<Self>(&file).expect("Failed to parse file")
    }
}
