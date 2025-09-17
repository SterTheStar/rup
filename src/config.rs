use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use anyhow::{Result, Context};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub api: ApiConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiConfig {
    pub api_type: String,
    pub options: HashMap<String, String>,
}

impl Config {
    pub fn load() -> Result<Self> {
        let path = Config::config_path();
        if !path.exists() {
            return Err(anyhow::anyhow!("Config file not found. Please run 'rup config' to create one."));
        }
        let content = fs::read_to_string(&path).context("Failed to read config file")?;
        let config: Config = toml::from_str(&content).context("Failed to parse config file")?;
        Ok(config)
    }

    pub fn default() -> Self {
        Config {
            api: ApiConfig {
                api_type: "litterbox".to_string(),
                options: HashMap::new(),
            },
        }
    }

    pub fn save(&self) -> Result<()> {
        let content = toml::to_string_pretty(self).context("Failed to serialize config")?;
        fs::write(Config::config_path(), content).context("Failed to write config file")?;
        Ok(())
    }

    pub fn config_path() -> PathBuf {
        let mut dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        dir.push("rup");
        fs::create_dir_all(&dir).ok();
        dir.push("config.toml");
        dir
    }
}
