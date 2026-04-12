use bollard::image::{ListImagesOptions, RemoveImageOptions};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageInfo {
    pub id: String,
    pub tags: Vec<String>,
    pub size: i64,
    pub created: i64,
}

#[tauri::command]
pub async fn docker_list_images(
    state: tauri::State<'_, super::containers::DockerState>,
) -> Result<Vec<ImageInfo>, String> {
    let guard = state.client.lock().await;
    let docker = guard.as_ref().ok_or("Docker not available")?;

    let options = Some(ListImagesOptions::<String> {
        all: false,
        filters: HashMap::new(),
        ..Default::default()
    });

    let images = docker
        .list_images(options)
        .await
        .map_err(|e| format!("Docker error: {}", e))?;

    Ok(images
        .into_iter()
        .map(|i| ImageInfo {
            id: i.id.chars().take(19).collect(),
            tags: i.repo_tags,
            size: i.size,
            created: i.created,
        })
        .collect())
}

#[tauri::command]
pub async fn docker_remove_image(
    state: tauri::State<'_, super::containers::DockerState>,
    image_id: String,
    force: bool,
) -> Result<(), String> {
    let guard = state.client.lock().await;
    let docker = guard.as_ref().ok_or("Docker not available")?;
    let options = Some(RemoveImageOptions { force, noprune: false });
    docker
        .remove_image(&image_id, options, None)
        .await
        .map_err(|e| format!("Error: {}", e))?;
    Ok(())
}
