use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub entry: String,
    pub permissions: Vec<String>,
    pub hooks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    pub manifest: PluginManifest,
    pub enabled: bool,
    pub path: String,
}

pub struct PluginState {
    pub plugins: Arc<Mutex<HashMap<String, PluginInfo>>>,
    pub plugin_dir: PathBuf,
}

impl PluginState {
    pub fn new() -> Self {
        let plugin_dir = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("nexterm")
            .join("plugins");
        std::fs::create_dir_all(&plugin_dir).ok();
        Self {
            plugins: Arc::new(Mutex::new(HashMap::new())),
            plugin_dir,
        }
    }
}

#[tauri::command]
pub async fn plugin_list(
    state: tauri::State<'_, PluginState>,
) -> Result<Vec<PluginInfo>, String> {
    let plugins = state.plugins.lock().await;
    Ok(plugins.values().cloned().collect())
}

#[tauri::command]
pub async fn plugin_toggle(
    state: tauri::State<'_, PluginState>,
    plugin_id: String,
    enabled: bool,
) -> Result<(), String> {
    let mut plugins = state.plugins.lock().await;
    if let Some(plugin) = plugins.get_mut(&plugin_id) {
        plugin.enabled = enabled;
    }
    Ok(())
}

#[tauri::command]
pub async fn plugin_scan(
    state: tauri::State<'_, PluginState>,
) -> Result<Vec<PluginInfo>, String> {
    let plugin_dir = state.plugin_dir.clone();
    let mut found = Vec::new();

    if let Ok(entries) = std::fs::read_dir(&plugin_dir) {
        for entry in entries.flatten() {
            let manifest_path = entry.path().join("plugin.json");
            if manifest_path.exists() {
                if let Ok(content) = std::fs::read_to_string(&manifest_path) {
                    if let Ok(manifest) = serde_json::from_str::<PluginManifest>(&content) {
                        let info = PluginInfo {
                            manifest: manifest.clone(),
                            enabled: true,
                            path: entry.path().to_string_lossy().to_string(),
                        };
                        found.push(info.clone());
                        let mut plugins = state.plugins.lock().await;
                        plugins.insert(manifest.id, info);
                    }
                }
            }
        }
    }

    Ok(found)
}
