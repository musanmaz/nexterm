import type {
  PtySessionInfo,
  PtyOutput,
  CpuInfo,
  MemoryInfo,
  NetworkInfo,
  ProcessInfo,
  DiskInfo,
  FileEntry,
  ContainerInfo,
  ImageInfo,
  GitBranch,
  GitCommit,
  GitStatus,
  FileDiff,
  SSHProfile,
  PluginInfo,
  AIRequest,
  AIResponse,
  K8sContext,
  K8sPod,
  K8sDeployment,
  K8sService,
  K8sNamespace,
} from '$lib/types';
import { invoke as tauriInvoke } from '@tauri-apps/api/core';
import { listen as tauriListen } from '@tauri-apps/api/event';

function hasTauri(): boolean {
  return typeof window !== 'undefined' &&
    typeof (window as any).__TAURI_INTERNALS__ !== 'undefined';
}

async function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  if (!hasTauri()) throw new Error(`Not running in Tauri`);
  return tauriInvoke<T>(cmd, args);
}

async function listen<T>(event: string, handler: (ev: { payload: T }) => void): Promise<() => void> {
  if (!hasTauri()) return () => {};
  return tauriListen<T>(event, handler);
}

// PTY Commands
export const ptySpawn = (rows: number, cols: number, shell?: string) =>
  invoke<PtySessionInfo>('pty_spawn', { rows, cols, shell });

export const ptyWrite = (sessionId: string, data: string) =>
  invoke<void>('pty_write', { sessionId, data });

export const ptyResize = (sessionId: string, rows: number, cols: number) =>
  invoke<void>('pty_resize', { sessionId, rows, cols });

export const ptyKill = (sessionId: string) =>
  invoke<void>('pty_kill', { sessionId });

export const getDefaultShell = () =>
  invoke<string>('get_default_shell_cmd');

export const ptyGetCwd = (sessionId: string) =>
  invoke<string>('pty_get_cwd', { sessionId });

export const onPtyOutput = (callback: (data: PtyOutput) => void) =>
  listen<PtyOutput>('pty-output', (event) => callback(event.payload));

export const onPtyExit = (callback: (sessionId: string) => void) =>
  listen<string>('pty-exit', (event) => callback(event.payload));

// System Commands
export const getCpuInfo = () => invoke<CpuInfo>('get_cpu_info');
export const getMemoryInfo = () => invoke<MemoryInfo>('get_memory_info');
export const getNetworkInfo = () => invoke<NetworkInfo>('get_network_info');
export const getProcesses = () => invoke<ProcessInfo[]>('get_processes');
export const getDiskInfo = () => invoke<DiskInfo[]>('get_disk_info');
export const getHostname = () => invoke<string>('get_hostname');
export const getOsInfo = () => invoke<string>('get_os_info');
export const getUptime = () => invoke<number>('get_uptime');
export const listDirectory = (path: string) => invoke<FileEntry[]>('list_directory', { path });
export const getHomeDir = () => invoke<string>('get_home_dir');

// Docker Commands
export const dockerListContainers = (all: boolean) =>
  invoke<ContainerInfo[]>('docker_list_containers', { all });
export const dockerStartContainer = (containerId: string) =>
  invoke<void>('docker_start_container', { containerId });
export const dockerStopContainer = (containerId: string) =>
  invoke<void>('docker_stop_container', { containerId });
export const dockerRestartContainer = (containerId: string) =>
  invoke<void>('docker_restart_container', { containerId });
export const dockerRemoveContainer = (containerId: string, force: boolean) =>
  invoke<void>('docker_remove_container', { containerId, force });
export const dockerIsAvailable = () => invoke<boolean>('docker_is_available');
export const dockerListImages = () => invoke<ImageInfo[]>('docker_list_images');
export const dockerRemoveImage = (imageId: string, force: boolean) =>
  invoke<void>('docker_remove_image', { imageId, force });

// Git Commands
export const gitGetBranches = (path: string) =>
  invoke<GitBranch[]>('git_get_branches', { path });
export const gitGetLog = (path: string, maxCount: number) =>
  invoke<GitCommit[]>('git_get_log', { path, maxCount });
export const gitGetStatus = (path: string) =>
  invoke<GitStatus>('git_get_status', { path });
export const gitGetDiff = (path: string, staged: boolean) =>
  invoke<FileDiff[]>('git_get_diff', { path, staged });

// SSH Commands
export const sshSaveProfile = (profile: SSHProfile) =>
  invoke<void>('ssh_save_profile', { profile });
export const sshGetProfiles = () => invoke<SSHProfile[]>('ssh_get_profiles');
export const sshDeleteProfile = (profileId: string) =>
  invoke<void>('ssh_delete_profile', { profileId });

// Plugin Commands
export const pluginList = () => invoke<PluginInfo[]>('plugin_list');
export const pluginToggle = (pluginId: string, enabled: boolean) =>
  invoke<void>('plugin_toggle', { pluginId, enabled });
export const pluginScan = () => invoke<PluginInfo[]>('plugin_scan');

// AI Commands
export const aiQuery = (request: AIRequest) =>
  invoke<AIResponse>('ai_query', { request });
export const aiTranslateCommand = (
  prompt: string, provider: string, model: string, apiKey: string, shell: string
) => invoke<string>('ai_translate_command', { prompt, provider, model, apiKey, shell });
export const aiExplainCommand = (
  command: string, provider: string, model: string, apiKey: string
) => invoke<string>('ai_explain_command', { command, provider, model, apiKey });
export const aiAnalyzeError = (
  errorOutput: string, command: string, provider: string, model: string, apiKey: string
) => invoke<string>('ai_analyze_error', { errorOutput, command, provider, model, apiKey });

// Kubernetes Commands
export const k8sIsAvailable = () => invoke<boolean>('k8s_is_available');
export const k8sGetContexts = () => invoke<K8sContext[]>('k8s_get_contexts');
export const k8sSwitchContext = (contextName: string) =>
  invoke<void>('k8s_switch_context', { contextName });
export const k8sGetNamespaces = () => invoke<K8sNamespace[]>('k8s_get_namespaces');
export const k8sGetPods = (namespace: string) =>
  invoke<K8sPod[]>('k8s_get_pods', { namespace });
export const k8sGetDeployments = (namespace: string) =>
  invoke<K8sDeployment[]>('k8s_get_deployments', { namespace });
export const k8sGetServices = (namespace: string) =>
  invoke<K8sService[]>('k8s_get_services', { namespace });
export const k8sScaleDeployment = (namespace: string, name: string, replicas: number) =>
  invoke<void>('k8s_scale_deployment', { namespace, name, replicas });
export const k8sRestartDeployment = (namespace: string, name: string) =>
  invoke<void>('k8s_restart_deployment', { namespace, name });
export const k8sDeletePod = (namespace: string, name: string) =>
  invoke<void>('k8s_delete_pod', { namespace, name });
export const k8sOcLogin = (apiUrl: string, username: string, password: string, insecureSkipTls: boolean) =>
  invoke<string>('k8s_oc_login', { apiUrl, username, password, insecureSkipTls });
export const k8sOcIsAvailable = () => invoke<boolean>('k8s_oc_is_available');
export const k8sGetContextsForKubeconfig = (kubeconfigPath: string) =>
  invoke<import('$lib/types').K8sContext[]>('k8s_get_contexts_for_kubeconfig', { kubeconfigPath });

// Updater Commands
export const downloadUpdate = (url: string, filename: string) =>
  invoke<string>('download_update', { url, filename });
export const installAndRestart = (dmgPath: string) =>
  invoke<void>('install_and_restart', { dmgPath });
