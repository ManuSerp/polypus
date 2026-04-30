use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct DockerCompose {
    #[serde(default)]
    pub version: Option<String>,

    #[serde(default)]
    pub services: HashMap<String, Service>,

    #[serde(default)]
    pub networks: Option<HashMap<String, Network>>,

    #[serde(default)]
    pub volumes: Option<HashMap<String, Volume>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Service {
    #[serde(default)]
    pub image: Option<String>,

    #[serde(default)]
    pub build: Option<BuildConfig>,

    #[serde(default)]
    pub ports: Vec<String>,

    #[serde(default)]
    pub environment: Option<Environment>,

    #[serde(default)]
    pub volumes: Vec<String>,

    #[serde(default)]
    pub depends_on: Vec<String>,

    #[serde(default)]
    pub command: Option<String>,

    #[serde(default)]
    pub container_name: Option<String>,

    #[serde(default)]
    pub restart: Option<String>,

    #[serde(default)]
    pub networks: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BuildConfig {
    Simple(String),
    Detailed {
        context: String,
        #[serde(default)]
        dockerfile: Option<String>,
        #[serde(default)]
        args: Option<HashMap<String, String>>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Environment {
    List(Vec<String>),
    Map(HashMap<String, String>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Network {
    #[serde(default)]
    pub driver: Option<String>,

    #[serde(default)]
    pub external: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Volume {
    #[serde(default)]
    pub driver: Option<String>,

    #[serde(default)]
    pub external: Option<bool>,
}

impl DockerCompose {
    /// Load a docker-compose.yaml file from the given path
    pub fn from_file(path: &str) -> Result<Self> {
        let contents = fs::read_to_string(path)?;
        let compose: DockerCompose = serde_yaml::from_str(&contents)?;
        Ok(compose)
    }

    /// Get all service names
    pub fn service_names(&self) -> Vec<&String> {
        self.services.keys().collect()
    }

    /// Get a specific service by name
    pub fn get_service(&self, name: &str) -> Option<&Service> {
        self.services.get(name)
    }

    /// Get all services that depend on a given service
    pub fn get_dependents(&self, service_name: &str) -> Vec<&String> {
        self.services
            .iter()
            .filter(|(_, service)| service.depends_on.contains(&service_name.to_string()))
            .map(|(name, _)| name)
            .collect()
    }

    /// Get services that have exposed ports
    pub fn services_with_ports(&self) -> Vec<(&String, &Service)> {
        self.services
            .iter()
            .filter(|(_, service)| !service.ports.is_empty())
            .collect()
    }
}

impl Service {
    /// Check if the service uses an image or builds from source
    pub fn uses_image(&self) -> bool {
        self.image.is_some()
    }

    /// Check if the service builds from source
    pub fn has_build(&self) -> bool {
        self.build.is_some()
    }

    /// Get environment variables as a HashMap (regardless of format)
    pub fn get_env_map(&self) -> HashMap<String, String> {
        match &self.environment {
            Some(Environment::Map(map)) => map.clone(),
            Some(Environment::List(list)) => list
                .iter()
                .filter_map(|s| {
                    let parts: Vec<&str> = s.splitn(2, '=').collect();
                    if parts.len() == 2 {
                        Some((parts[0].to_string(), parts[1].to_string()))
                    } else {
                        None
                    }
                })
                .collect(),
            None => HashMap::new(),
        }
    }
}
