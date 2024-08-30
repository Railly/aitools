use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub output_format: String,
    pub ignore_patterns: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            output_format: "markdown".to_string(),
            ignore_patterns: vec!["node_modules".to_string(), ".git".to_string()],
        }
    }
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = Path::new("config.json");
    if config_path.exists() {
        let contents = fs::read_to_string(config_path)?;
        Ok(serde_json::from_str(&contents)?)
    } else {
        let config = Config::default();
        let json = serde_json::to_string_pretty(&config)?;
        fs::write(config_path, json)?;
        Ok(config)
    }
}
