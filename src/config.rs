use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub custom_browsers: Vec<(String, String)>,
    pub hidden_browsers: Vec<String>,
}

impl Config {
    pub fn load() -> Self {
        let config_path = Self::get_config_path();
        if let Ok(mut file) = File::open(config_path) {
            let mut contents = String::new();
            if file.read_to_string(&mut contents).is_ok() {
                if let Ok(config) = serde_json::from_str(&contents) {
                    return config;
                }
            }
        }
        Self::default()
    }
    
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path();
        let config_dir = config_path.parent().unwrap();
        std::fs::create_dir_all(config_dir)?;
        
        let json = serde_json::to_string_pretty(self)?;
        let mut file = File::create(config_path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
    
    fn get_config_path() -> PathBuf {
        let local_app_data = std::env::var("LOCALAPPDATA")
            .unwrap_or_else(|_| ".".to_string());
        PathBuf::from(local_app_data).join("BrowserPicker").join("config.json")
    }
} 
