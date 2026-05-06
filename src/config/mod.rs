// to do: geenric trait for services that can be implemted for docker compose or other
pub mod compose;

use std::path::Path;

use crate::Result;
use crate::config::compose::DockerCompose;
use serde::{Deserialize, Serialize};
pub fn is_file(path: &str) -> bool {
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
    pub containers: Vec<Container>,
    pub folder_is_config: bool,
    pub kind: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Container {
    pub id: String,
    pub name: String,
    pub image: String,
}

impl DCService {
    pub fn new_from_dc(service_name: String, dc_config_path: String) -> Result<Self> {
        let mut service = Self {
            name: service_name,
            config_path: dc_config_path,
            containers: vec![],
            folder_is_config: false,
            kind: "docker-compose".to_string(),
        };
        let compose = DockerCompose::from_file(&service.config_path)?;
        for serv in compose.service_names() {
            if let Some(dserv) = compose.get_service(&serv) {
                if let Some(cname) = &dserv.container_name {
                    service.containers.push(Container {
                        id: "".to_string(),
                        name: cname.clone(),
                        image: dserv.image.clone().unwrap_or_default(),
                    });
                } else {
                    service.containers.push(Container {
                        id: "".to_string(),
                        name: serv.clone(),
                        image: dserv.image.clone().unwrap_or_default(),
                    });
                }
            } else {
                return Err(format!("Service {} not found in compose file", serv).into());
            }
        }
        Ok(service)
    }
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
    pub fn get_default() -> Result<Self> {
        let home = std::env::var("HOME")?;
        let config_path = format!("{}/.polypus_config.json", home);
        Self::new_from_path(config_path)
    }
}
