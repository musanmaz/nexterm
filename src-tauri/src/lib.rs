mod pty;
mod system;
mod docker;
mod git;
mod ssh;
mod plugins;
mod ai;
mod kubernetes;
mod state;

use pty::PtyManager;
use system::cpu::SystemMonitor;
use system::network::NetworkMonitorState;
use docker::containers::DockerState;
use ssh::SSHState;
use plugins::loader::PluginState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .manage(PtyManager::new())
        .manage(SystemMonitor::new())
        .manage(NetworkMonitorState::new())
        .manage(DockerState::new())
        .manage(SSHState::new())
        .manage(PluginState::new())
        .invoke_handler(tauri::generate_handler![
            // PTY commands
            pty::pty_spawn,
            pty::pty_write,
            pty::pty_resize,
            pty::pty_kill,
            pty::pty_get_cwd,
            pty::get_default_shell_cmd,
            // System commands
            system::get_cpu_info,
            system::get_memory_info,
            system::get_network_info,
            system::get_processes,
            system::get_disk_info,
            system::get_hostname,
            system::get_os_info,
            system::get_uptime,
            system::list_directory,
            system::get_home_dir,
            // Docker commands
            docker::docker_list_containers,
            docker::docker_start_container,
            docker::docker_stop_container,
            docker::docker_restart_container,
            docker::docker_remove_container,
            docker::docker_is_available,
            docker::docker_list_images,
            docker::docker_remove_image,
            // Git commands
            git::git_get_branches,
            git::git_get_log,
            git::git_get_status,
            git::git_get_diff,
            // SSH commands
            ssh::ssh_save_profile,
            ssh::ssh_get_profiles,
            ssh::ssh_delete_profile,
            // Plugin commands
            plugins::plugin_list,
            plugins::plugin_toggle,
            plugins::plugin_scan,
            // AI commands
            ai::ai_query,
            ai::ai_translate_command,
            ai::ai_explain_command,
            ai::ai_analyze_error,
            // Kubernetes commands
            kubernetes::k8s_is_available,
            kubernetes::k8s_get_contexts,
            kubernetes::k8s_switch_context,
            kubernetes::k8s_get_namespaces,
            kubernetes::k8s_get_pods,
            kubernetes::k8s_get_deployments,
            kubernetes::k8s_get_services,
            kubernetes::k8s_scale_deployment,
            kubernetes::k8s_restart_deployment,
            kubernetes::k8s_delete_pod,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
