// to do: geenric trait for services that can be implemted for docker compose or other
use std::path::Path;

use crate::Result;
use serde::{Deserialize, Serialize};

fn is_file(path: &str) -> bool {
    Path::new(path).is_file()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolypusConfig {
    pub metadata: String,
    pub registered: Vec<DCService>,
    pub path: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DCService {
    pub name: String,
    pub config_path: String,
    pub folder_is_config: bool,
    pub kind: String,
}

impl PolypusConfig {
    pub fn load_config_file(&mut self) -> Result<()> {
        if !is_file(&self.path) {
            return Err(format!("Config file not found at path: {}", self.path).into());
        }
        *self = serde_json::from_str::<PolypusConfig>(&std::fs::read_to_string(&self.path)?)?;
        Ok(())
    }

    pub fn write_config_file(&self) -> Result<()> {
        std::fs::write(&self.path, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }

    pub fn register(&mut self, service: DCService) -> Result<()> {
        self.registered.push(service);
        self.write_config_file()?;
        Ok(())
    }

    pub fn new_from_path(path: String) -> Result<Self> {
        if !is_file(&path) {
            let config = Self {
                metadata: "Polypus Config".to_string(),
                registered: vec![],
                path,
            };
            config.write_config_file()?;
            Ok(config)
        } else {
            // this is a bit ugly

            let mut config = Self {
                metadata: "Polypus Config".to_string(),
                registered: vec![],
                path,
            };
            config.load_config_file()?;
            Ok(config)
        }
    }
}
