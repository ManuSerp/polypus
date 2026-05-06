pub mod config;
pub mod docker;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
use bollard::plugin::ContainerSummary;

use crate::config::DCService;
use crate::docker::ps;
pub struct ServiceStatus<'a> {
    pub service: &'a DCService,
    pub status: String,
    pub containers_status: Vec<ContainerStatus>,
}

pub struct ContainerStatus {
    pub name: String,
    pub image: String,
    pub status: String,
    pub id: String,
}

impl<'a> ServiceStatus<'a> {
    pub fn new(service: &'a DCService) -> Self {
        Self {
            service,
            status: "unknown".to_string(),
            containers_status: vec![],
        }
    }
    pub async fn new_and_update(service: &'a DCService) -> Result<Self> {
        let mut status = Self::new(service);
        status.update_status().await?;
        Ok(status)
    }
    async fn update_containers_from_docker(&mut self) -> Result<()> {
        let docker_status = ps().await?;
        self.containers_status.clear();
        let mut container_map: std::collections::HashMap<String, &ContainerSummary> =
            std::collections::HashMap::new();

        for container in &docker_status {
            if let Some(names) = &container.names {
                for name in names {
                    container_map.insert(name.clone(), &container);
                }
            }
        }
        for s_container in &self.service.containers {
            let name = s_container.name.to_string();
            // get container from map:
            if let Some(container) = container_map.get(&name) {
                let container_status = ContainerStatus {
                    name: name.clone(),
                    image: s_container.image.clone(),
                    status: container.status.clone().unwrap_or_default(),
                    id: container.id.clone().unwrap_or_default(),
                };
                self.containers_status.push(container_status);
            } else {
                self.containers_status.push(ContainerStatus {
                    name: name.clone(),
                    image: s_container.image.clone(),
                    status: "not found".to_string(),
                    id: "".to_string(),
                });
            }
        }

        Ok(())
    }
    pub async fn update_status(&mut self) -> Result<()> {
        self.update_containers_from_docker().await?;
        // compute global status
        for c in &self.containers_status {
            if c.status.contains("unhealthy") {
                self.status = "unhealthy".to_string();
                return Ok(());
            } else if c.status.contains("exited") {
                self.status = "exited".to_string();
                return Ok(());
            } else if c.status.contains("running") {
                continue;
            } else {
                self.status = "unknown".to_string();
                return Ok(());
            }
        }
        Ok(())
    }
    pub fn pretty_print_containers(&self) -> Vec<String> {
        let res: &Vec<String> = &self
            .containers_status
            .iter()
            .map(|c| format!("{}: {}", c.name, c.status))
            .collect();

        res.clone()
    }
    pub fn pretty_print(&self) -> String {
        format!(
            "Service: {}, Status: {}, Containers: {}",
            self.service.name,
            self.status,
            self.containers_status.len()
        )
    }
}
