use bollard::Docker;
use bollard::container::{ListContainersOptions, StopContainerOptions, StartContainerOptions, RemoveContainerOptions};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerInfo {
    pub id: String,
    pub name: String,
    pub image: String,
    pub state: String,
    pub status: String,
    pub ports: Vec<String>,
    pub created: i64,
}

pub struct DockerState {
    pub(crate) client: Arc<Mutex<Option<Docker>>>,
}

impl DockerState {
    pub fn new() -> Self {
        let client = Docker::connect_with_local_defaults().ok();
        Self {
            client: Arc::new(Mutex::new(client)),
        }
    }
}

#[tauri::command]
pub async fn docker_list_containers(
    state: tauri::State<'_, DockerState>,
    all: bool,
) -> Result<Vec<ContainerInfo>, String> {
    let guard = state.client.lock().await;
    let docker = guard.as_ref().ok_or("Docker not available")?;

    let mut filters = HashMap::new();
    if !all {
        filters.insert("status".to_string(), vec!["running".to_string()]);
    }

    let options = Some(ListContainersOptions {
        all,
        filters,
        ..Default::default()
    });

    let containers = docker
        .list_containers(options)
        .await
        .map_err(|e| format!("Docker error: {}", e))?;

    Ok(containers
        .into_iter()
        .map(|c| {
            let ports = c.ports.unwrap_or_default().iter().map(|p| {
                format!(
                    "{}:{} -> {}",
                    p.ip.as_deref().unwrap_or(""),
                    p.public_port.unwrap_or(0),
                    p.private_port
                )
            }).collect();

            ContainerInfo {
                id: c.id.unwrap_or_default().chars().take(12).collect(),
                name: c.names.unwrap_or_default().first().cloned().unwrap_or_default().trim_start_matches('/').to_string(),
                image: c.image.unwrap_or_default(),
                state: c.state.unwrap_or_default(),
                status: c.status.unwrap_or_default(),
                ports,
                created: c.created.unwrap_or(0),
            }
        })
        .collect())
}

#[tauri::command]
pub async fn docker_start_container(
    state: tauri::State<'_, DockerState>,
    container_id: String,
) -> Result<(), String> {
    let guard = state.client.lock().await;
    let docker = guard.as_ref().ok_or("Docker not available")?;
    docker
        .start_container(&container_id, None::<StartContainerOptions<String>>)
        .await
        .map_err(|e| format!("Error: {}", e))
}

#[tauri::command]
pub async fn docker_stop_container(
    state: tauri::State<'_, DockerState>,
    container_id: String,
) -> Result<(), String> {
    let guard = state.client.lock().await;
    let docker = guard.as_ref().ok_or("Docker not available")?;
    docker
        .stop_container(&container_id, Some(StopContainerOptions { t: 10 }))
        .await
        .map_err(|e| format!("Error: {}", e))
}

#[tauri::command]
pub async fn docker_restart_container(
    state: tauri::State<'_, DockerState>,
    container_id: String,
) -> Result<(), String> {
    let guard = state.client.lock().await;
    let docker = guard.as_ref().ok_or("Docker not available")?;
    docker
        .restart_container(&container_id, Some(bollard::container::RestartContainerOptions { t: 10 }))
        .await
        .map_err(|e| format!("Error: {}", e))
}

#[tauri::command]
pub async fn docker_remove_container(
    state: tauri::State<'_, DockerState>,
    container_id: String,
    force: bool,
) -> Result<(), String> {
    let guard = state.client.lock().await;
    let docker = guard.as_ref().ok_or("Docker not available")?;
    docker
        .remove_container(
            &container_id,
            Some(RemoveContainerOptions { force, ..Default::default() }),
        )
        .await
        .map_err(|e| format!("Error: {}", e))
}

#[tauri::command]
pub async fn docker_is_available(state: tauri::State<'_, DockerState>) -> Result<bool, String> {
    let guard = state.client.lock().await;
    match guard.as_ref() {
        Some(docker) => match docker.ping().await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        },
        None => Ok(false),
    }
}
