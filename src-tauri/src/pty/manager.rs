use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PtyOutput {
    pub session_id: String,
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PtySessionInfo {
    pub id: String,
    pub pid: u32,
    pub shell: String,
}

#[allow(dead_code)]
pub struct PtyInstance {
    pub writer: Arc<Mutex<Box<dyn Write + Send>>>,
    pub pid: u32,
    pub shell: String,
}

pub struct PtyManager {
    pub sessions: Arc<Mutex<HashMap<String, PtyInstance>>>,
}

impl PtyManager {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

fn get_default_shell() -> String {
    #[cfg(target_os = "windows")]
    {
        std::env::var("COMSPEC").unwrap_or_else(|_| "cmd.exe".to_string())
    }
    #[cfg(not(target_os = "windows"))]
    {
        std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string())
    }
}

#[tauri::command]
pub async fn pty_spawn(
    app: AppHandle,
    state: tauri::State<'_, PtyManager>,
    rows: u16,
    cols: u16,
    shell: Option<String>,
) -> Result<PtySessionInfo, String> {
    let session_id = Uuid::new_v4().to_string();
    let shell_path = shell.unwrap_or_else(get_default_shell);

    let pty_system = native_pty_system();
    let pair = pty_system
        .openpty(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })
        .map_err(|e| format!("Failed to open PTY: {}", e))?;

    let mut cmd = CommandBuilder::new(&shell_path);
    cmd.cwd(dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("/")));

    cmd.env("TERM", "xterm-256color");
    cmd.env("COLORTERM", "truecolor");
    cmd.env("TERM_PROGRAM", "NEXTERM");
    cmd.env("TERM_PROGRAM_VERSION", "3.0.0");

    let child = pair
        .slave
        .spawn_command(cmd)
        .map_err(|e| format!("Failed to spawn: {}", e))?;

    let pid = child.process_id().unwrap_or(0);

    let reader = pair
        .master
        .try_clone_reader()
        .map_err(|e| format!("Failed to clone reader: {}", e))?;

    let writer = pair
        .master
        .take_writer()
        .map_err(|e| format!("Failed to take writer: {}", e))?;

    let instance = PtyInstance {
        writer: Arc::new(Mutex::new(writer)),
        pid,
        shell: shell_path.clone(),
    };

    state.sessions.lock().await.insert(session_id.clone(), instance);

    let sid = session_id.clone();
    let app_handle = app.clone();
    std::thread::spawn(move || {
        let mut reader = reader;
        let mut buf = [0u8; 4096];
        loop {
            match reader.read(&mut buf) {
                Ok(0) => {
                    let _ = app_handle.emit("pty-exit", &sid);
                    break;
                }
                Ok(n) => {
                    let data = String::from_utf8_lossy(&buf[..n]).to_string();
                    let _ = app_handle.emit(
                        "pty-output",
                        PtyOutput {
                            session_id: sid.clone(),
                            data,
                        },
                    );
                }
                Err(_) => {
                    let _ = app_handle.emit("pty-exit", &sid);
                    break;
                }
            }
        }
    });

    let sid2 = session_id.clone();
    let app_handle2 = app.clone();
    let child = Arc::new(Mutex::new(child));
    let child_clone = child.clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            let mut c = child_clone.lock().await;
            if let Ok(Some(_status)) = c.try_wait() {
                let _ = app_handle2.emit("pty-exit", &sid2);
                break;
            }
        }
    });

    Ok(PtySessionInfo {
        id: session_id,
        pid,
        shell: shell_path,
    })
}

#[tauri::command]
pub async fn pty_write(
    state: tauri::State<'_, PtyManager>,
    session_id: String,
    data: String,
) -> Result<(), String> {
    let sessions = state.sessions.lock().await;
    if let Some(instance) = sessions.get(&session_id) {
        let mut writer = instance.writer.lock().await;
        writer
            .write_all(data.as_bytes())
            .map_err(|e| format!("Write error: {}", e))?;
        writer.flush().map_err(|e| format!("Flush error: {}", e))?;
    } else {
        return Err("Session not found".to_string());
    }
    Ok(())
}

#[tauri::command]
pub async fn pty_resize(
    _state: tauri::State<'_, PtyManager>,
    _session_id: String,
    _rows: u16,
    _cols: u16,
) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub async fn pty_kill(
    state: tauri::State<'_, PtyManager>,
    session_id: String,
) -> Result<(), String> {
    let mut sessions = state.sessions.lock().await;
    sessions.remove(&session_id);
    Ok(())
}

#[tauri::command]
pub fn get_default_shell_cmd() -> String {
    get_default_shell()
}

#[tauri::command]
pub async fn pty_get_cwd(
    state: tauri::State<'_, PtyManager>,
    session_id: String,
) -> Result<String, String> {
    let sessions = state.sessions.lock().await;
    if let Some(instance) = sessions.get(&session_id) {
        let pid = instance.pid;
        #[cfg(target_os = "macos")]
        {
            let output = std::process::Command::new("lsof")
                .args(["-p", &pid.to_string(), "-Fn", "-d", "cwd"])
                .output()
                .map_err(|e| format!("lsof failed: {}", e))?;
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                if let Some(path) = line.strip_prefix('n') {
                    if path.starts_with('/') {
                        return Ok(path.to_string());
                    }
                }
            }
            Err("CWD not found".to_string())
        }
        #[cfg(target_os = "linux")]
        {
            let link = format!("/proc/{}/cwd", pid);
            std::fs::read_link(&link)
                .map(|p| p.to_string_lossy().to_string())
                .map_err(|e| format!("Failed to read CWD: {}", e))
        }
        #[cfg(target_os = "windows")]
        {
            Err("CWD detection not supported on Windows".to_string())
        }
    } else {
        Err("Session not found".to_string())
    }
}
