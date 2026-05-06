use crate::Result;

use bollard::Docker;
use bollard::query_parameters::ListContainersOptions;
use std::collections::HashMap;

pub async fn ps() -> Result<()> {
    let docker = Docker::connect_with_unix_defaults()?;

    let containers = docker
        .list_containers(Some(ListContainersOptions {
            all: true, // == ?all=1
            filters: Some(HashMap::new()),
            ..Default::default()
        }))
        .await?;

    for c in containers {
        let id = c.id.as_deref().unwrap_or("<none>");
        let names = c.names.unwrap_or_default();
        let state = c.state.unwrap();
        let status = c.status.as_deref().unwrap_or("");

        println!("{id} {names:?} {state} ({status})");
    }

    Ok(())
}
