use serde::{Deserialize, Serialize};
use sysinfo::System;
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuInfo {
    pub brand: String,
    pub cores: usize,
    pub usage: Vec<f32>,
    pub frequency: u64,
    pub global_usage: f32,
}

pub struct SystemMonitor {
    pub(crate) sys: Mutex<System>,
}

impl SystemMonitor {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        Self {
            sys: Mutex::new(sys),
        }
    }
}

#[tauri::command]
pub fn get_cpu_info(state: tauri::State<'_, SystemMonitor>) -> CpuInfo {
    let mut sys = state.sys.lock().unwrap();
    sys.refresh_cpu_all();

    let cpus = sys.cpus();
    CpuInfo {
        brand: cpus.first().map(|c| c.brand().to_string()).unwrap_or_default(),
        cores: cpus.len(),
        usage: cpus.iter().map(|c| c.cpu_usage()).collect(),
        frequency: cpus.first().map(|c| c.frequency()).unwrap_or(0),
        global_usage: sys.global_cpu_usage(),
    }
}
