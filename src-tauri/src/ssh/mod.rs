use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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

pub struct SSHState {
    pub profiles: Arc<Mutex<HashMap<String, SSHProfile>>>,
}

impl SSHState {
    pub fn new() -> Self {
        Self {
            profiles: Arc::new(Mutex::new(HashMap::new())),
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
    Ok(())
}
