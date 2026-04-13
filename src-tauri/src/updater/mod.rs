use std::path::PathBuf;
use std::process::Command;

fn downloads_dir() -> PathBuf {
    dirs::download_dir().unwrap_or_else(|| {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("/tmp"))
            .join("Downloads")
    })
}

#[tauri::command]
pub async fn download_update(url: String, filename: String) -> Result<String, String> {
    let dest = downloads_dir().join(&filename);
    let dest_str = dest.to_string_lossy().to_string();

    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .header("User-Agent", "NEXTERM-Updater")
        .send()
        .await
        .map_err(|e| format!("Download request failed: {}", e))?;

    if !resp.status().is_success() {
        return Err(format!("Download failed with status: {}", resp.status()));
    }

    let bytes = resp
        .bytes()
        .await
        .map_err(|e| format!("Failed to read download body: {}", e))?;

    tokio::fs::write(&dest, &bytes)
        .await
        .map_err(|e| format!("Failed to write file: {}", e))?;

    Ok(dest_str)
}

#[tauri::command]
pub fn install_and_restart(dmg_path: String, app: tauri::AppHandle) -> Result<(), String> {
    Command::new("open")
        .arg(&dmg_path)
        .spawn()
        .map_err(|e| format!("Failed to open DMG: {}", e))?;

    // Give macOS a moment to mount the DMG before we quit
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(2));
        app.exit(0);
    });

    Ok(())
}
