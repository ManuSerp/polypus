use crate::Result;

use bollard::Docker;
use bollard::plugin::ContainerSummary;
use bollard::query_parameters::ListContainersOptions;
use std::collections::HashMap;

pub async fn ps() -> Result<Vec<ContainerSummary>> {
    let docker = Docker::connect_with_unix_defaults()?;

    Ok(docker
        .list_containers(Some(ListContainersOptions {
            all: true, // == ?all=1
            filters: Some(HashMap::new()),
            ..Default::default()
        }))
        .await?)
}
