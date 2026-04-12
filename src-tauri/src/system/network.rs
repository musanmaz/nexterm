use serde::{Deserialize, Serialize};
use sysinfo::Networks;
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterface {
    pub name: String,
    pub received: u64,
    pub transmitted: u64,
    pub mac: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInfo {
    pub interfaces: Vec<NetworkInterface>,
    pub total_received: u64,
    pub total_transmitted: u64,
}

pub struct NetworkMonitorState {
    networks: Mutex<Networks>,
}

impl NetworkMonitorState {
    pub fn new() -> Self {
        Self {
            networks: Mutex::new(Networks::new_with_refreshed_list()),
        }
    }
}

#[tauri::command]
pub fn get_network_info(state: tauri::State<'_, NetworkMonitorState>) -> NetworkInfo {
    let mut networks = state.networks.lock().unwrap();
    networks.refresh(true);

    let mut interfaces = Vec::new();
    let mut total_rx = 0u64;
    let mut total_tx = 0u64;

    for (name, data) in networks.iter() {
        let rx = data.total_received();
        let tx = data.total_transmitted();
        total_rx += rx;
        total_tx += tx;
        interfaces.push(NetworkInterface {
            name: name.to_string(),
            received: rx,
            transmitted: tx,
            mac: data.mac_address().to_string(),
        });
    }

    NetworkInfo {
        interfaces,
        total_received: total_rx,
        total_transmitted: total_tx,
    }
}
