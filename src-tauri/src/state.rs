use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub theme: String,
    pub keyboard_layout: String,
    pub audio_enabled: bool,
    pub audio_volume: f32,
    pub font_size: u32,
    pub shell: String,
    pub ai_provider: String,
    pub ai_api_key: String,
    pub ai_model: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: "tron".to_string(),
            keyboard_layout: "qwerty".to_string(),
            audio_enabled: true,
            audio_volume: 0.5,
            font_size: 14,
            shell: String::new(),
            ai_provider: "ollama".to_string(),
            ai_api_key: String::new(),
            ai_model: "llama3".to_string(),
        }
    }
}

#[allow(dead_code)]
pub struct PtySession {
    pub id: String,
    pub writer: Arc<Mutex<Box<dyn portable_pty::MasterPty + Send>>>,
    pub child: Arc<Mutex<Box<dyn portable_pty::Child + Send + Sync>>>,
}

#[allow(dead_code)]
pub struct AppState {
    pub settings: Arc<Mutex<AppSettings>>,
    pub pty_sessions: Arc<Mutex<HashMap<String, PtySession>>>,
}

impl AppState {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            settings: Arc::new(Mutex::new(AppSettings::default())),
            pty_sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
