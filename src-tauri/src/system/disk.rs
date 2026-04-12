use serde::{Deserialize, Serialize};
use sysinfo::Disks;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskInfo {
    pub name: String,
    pub mount_point: String,
    pub total_space: u64,
    pub available_space: u64,
    pub file_system: String,
    pub usage_percent: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub name: String,
    pub is_dir: bool,
    pub size: u64,
    pub modified: String,
}

#[tauri::command]
pub fn get_disk_info() -> Vec<DiskInfo> {
    let disks = Disks::new_with_refreshed_list();

    disks
        .iter()
        .map(|d| {
            let total = d.total_space();
            let available = d.available_space();
            DiskInfo {
                name: d.name().to_string_lossy().to_string(),
                mount_point: d.mount_point().to_string_lossy().to_string(),
                total_space: total,
                available_space: available,
                file_system: d.file_system().to_string_lossy().to_string(),
                usage_percent: if total > 0 {
                    ((total - available) as f32 / total as f32) * 100.0
                } else {
                    0.0
                },
            }
        })
        .collect()
}

#[tauri::command]
pub fn get_hostname() -> String {
    sysinfo::System::host_name().unwrap_or_else(|| "unknown".to_string())
}

#[tauri::command]
pub fn get_os_info() -> String {
    format!(
        "{} {}",
        sysinfo::System::name().unwrap_or_default(),
        sysinfo::System::os_version().unwrap_or_default()
    )
}

#[tauri::command]
pub fn get_uptime() -> u64 {
    sysinfo::System::uptime()
}

#[tauri::command]
pub fn list_directory(path: String) -> Result<Vec<FileEntry>, String> {
    let resolved = if path == "~" || path.is_empty() {
        dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("/"))
    } else {
        let p = std::path::PathBuf::from(&path);
        if path.starts_with('~') {
            if let Some(home) = dirs::home_dir() {
                home.join(path.strip_prefix("~/").unwrap_or(&path))
            } else {
                p
            }
        } else {
            p
        }
    };

    let entries = std::fs::read_dir(&resolved)
        .map_err(|e| format!("Failed to read directory: {}", e))?;

    let mut files: Vec<FileEntry> = entries
        .filter_map(|e| e.ok())
        .map(|e| {
            let metadata = e.metadata().ok();
            FileEntry {
                name: e.file_name().to_string_lossy().to_string(),
                is_dir: metadata.as_ref().map(|m| m.is_dir()).unwrap_or(false),
                size: metadata.as_ref().map(|m| m.len()).unwrap_or(0),
                modified: metadata
                    .and_then(|m| m.modified().ok())
                    .map(|t| {
                        let duration = t
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default();
                        duration.as_secs().to_string()
                    })
                    .unwrap_or_default(),
            }
        })
        .collect();

    files.sort_by(|a, b| {
        b.is_dir
            .cmp(&a.is_dir)
            .then(a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });

    Ok(files)
}

#[tauri::command]
pub fn get_home_dir() -> String {
    dirs::home_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| "/".to_string())
}
