use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSHProfile {
    pub id: String,
    pub name: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub auth_method: SSHAuthMethod,
    pub private_key_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SSHAuthMethod {
    Password,
    PrivateKey,
    Agent,
}

fn profiles_path() -> PathBuf {
    let base = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
    let dir = base.join("com.nexterm.app");
    std::fs::create_dir_all(&dir).ok();
    dir.join("ssh_profiles.json")
}

fn load_profiles_from_disk() -> HashMap<String, SSHProfile> {
    let path = profiles_path();
    match std::fs::read_to_string(&path) {
        Ok(data) => serde_json::from_str(&data).unwrap_or_default(),
        Err(_) => HashMap::new(),
    }
}

fn save_profiles_to_disk(profiles: &HashMap<String, SSHProfile>) {
    let path = profiles_path();
    if let Ok(data) = serde_json::to_string_pretty(profiles) {
        std::fs::write(&path, data).ok();
    }
}

pub struct SSHState {
    pub profiles: Arc<Mutex<HashMap<String, SSHProfile>>>,
}

impl SSHState {
    pub fn new() -> Self {
        let loaded = load_profiles_from_disk();
        Self {
            profiles: Arc::new(Mutex::new(loaded)),
        }
    }
}

#[tauri::command]
pub async fn ssh_save_profile(
    state: tauri::State<'_, SSHState>,
    profile: SSHProfile,
) -> Result<(), String> {
    let mut profiles = state.profiles.lock().await;
    profiles.insert(profile.id.clone(), profile);
    save_profiles_to_disk(&profiles);
    Ok(())
}

#[tauri::command]
pub async fn ssh_get_profiles(
    state: tauri::State<'_, SSHState>,
) -> Result<Vec<SSHProfile>, String> {
    let profiles = state.profiles.lock().await;
    Ok(profiles.values().cloned().collect())
}

#[tauri::command]
pub async fn ssh_delete_profile(
    state: tauri::State<'_, SSHState>,
    profile_id: String,
) -> Result<(), String> {
    let mut profiles = state.profiles.lock().await;
    profiles.remove(&profile_id);
    save_profiles_to_disk(&profiles);
    Ok(())
}
