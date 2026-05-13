pub mod config;
pub mod docker;
pub mod ui;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
use bollard::plugin::ContainerSummary;

use crate::config::DCService;
use crate::docker::ps;
pub struct ServiceStatus<'a> {
    pub service: &'a DCService,
    pub status: StatusEnum,
    pub containers_status: Vec<ContainerStatus>,
}

pub struct ContainerStatus {
    pub name: String,
    pub image: String,
    pub status: StatusEnum,
    pub id: String,
}

impl<'a> ServiceStatus<'a> {
    pub fn new(service: &'a DCService) -> Self {
        Self {
            service,
            status: StatusEnum::Unknown,
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
                    // Docker prefixes names with /, so store both versions
                    container_map.insert(name.clone(), &container);
                    if let Some(stripped) = name.strip_prefix('/') {
                        container_map.insert(stripped.to_string(), &container);
                    }
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
                    status: StatusEnum::from(container.status.clone().unwrap_or_default()),
                    id: container.id.clone().unwrap_or_default(),
                };
                self.containers_status.push(container_status);
            } else {
                self.containers_status.push(ContainerStatus {
                    name: name.clone(),
                    image: s_container.image.clone(),
                    status: StatusEnum::NotFound,
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
            match c.status {
                StatusEnum::Unhealthy => {
                    self.status = StatusEnum::Unhealthy;
                    return Ok(());
                }
                StatusEnum::Exited => {
                    self.status = StatusEnum::Unhealthy;
                    return Ok(());
                }
                StatusEnum::Up | StatusEnum::Healthy => continue,
                StatusEnum::NotFound => {
                    self.status = StatusEnum::Unknown;
                    return Ok(());
                }
                StatusEnum::Unknown => {
                    self.status = StatusEnum::Unknown;
                    return Ok(());
                }
            }
        }

        self.status = StatusEnum::Up;
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

#[derive(PartialEq, Eq)]
pub enum StatusEnum {
    Up,
    Exited,
    Unhealthy,
    NotFound,
    Unknown,
    Healthy,
}

impl From<String> for StatusEnum {
    fn from(s: String) -> Self {
        let s_lower = s.to_lowercase();
        if s_lower.contains("up") && s_lower.contains("healthy") {
            StatusEnum::Healthy
        } else if s_lower.contains("running") || s_lower.contains("up") {
            StatusEnum::Up
        } else if s_lower.contains("exited") {
            StatusEnum::Exited
        } else if s_lower.contains("unhealthy") {
            StatusEnum::Unhealthy
        } else if s_lower.contains("healthy") {
            StatusEnum::Healthy
        } else if s_lower.contains("not found") {
            StatusEnum::NotFound
        } else {
            StatusEnum::Unknown
        }
    }
}

impl std::fmt::Display for StatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            StatusEnum::Up => "Up",
            StatusEnum::Exited => "Exited",
            StatusEnum::Unhealthy => "Unhealthy",
            StatusEnum::NotFound => "Not Found",
            StatusEnum::Unknown => "Unknown",
            StatusEnum::Healthy => "Healthy",
        };
        write!(f, "{}", s)
    }
}
