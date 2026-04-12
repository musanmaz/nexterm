use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryInfo {
    pub total: u64,
    pub used: u64,
    pub available: u64,
    pub swap_total: u64,
    pub swap_used: u64,
    pub usage_percent: f32,
}

#[tauri::command]
pub fn get_memory_info(state: tauri::State<'_, super::cpu::SystemMonitor>) -> MemoryInfo {
    let mut sys = state.sys.lock().unwrap();
    sys.refresh_memory();

    let total = sys.total_memory();
    let used = sys.used_memory();
    MemoryInfo {
        total,
        used,
        available: sys.available_memory(),
        swap_total: sys.total_swap(),
        swap_used: sys.used_swap(),
        usage_percent: if total > 0 {
            (used as f32 / total as f32) * 100.0
        } else {
            0.0
        },
    }
}
