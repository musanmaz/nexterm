export interface Theme {
  name: string;
  colors: {
    primary: string;
    secondary: string;
    accent: string;
    bgPrimary: string;
    bgSecondary: string;
    bgPanel: string;
    text: string;
    textBright: string;
    border: string;
    borderBright: string;
    success: string;
    warning: string;
    error: string;
  };
  terminal: {
    background: string;
    foreground: string;
    cursor: string;
    selectionBackground: string;
    black: string;
    red: string;
    green: string;
    yellow: string;
    blue: string;
    magenta: string;
    cyan: string;
    white: string;
    brightBlack: string;
    brightRed: string;
    brightGreen: string;
    brightYellow: string;
    brightBlue: string;
    brightMagenta: string;
    brightCyan: string;
    brightWhite: string;
  };
  globe?: {
    baseColor: string;
    markerColor: string;
    pinColor: string;
    satelliteColor: string;
  };
}

export interface AppSettings {
  theme: string;
  keyboardLayout: string;
  audioEnabled: boolean;
  audioVolume: number;
  fontSize: number;
  shell: string;
  aiProvider: string;
  aiApiKey: string;
  aiModel: string;
  litellmBaseUrl: string;
  litellmApiKey: string;
  litellmModel: string;
  aiProviders: AIProvider[];
  activeProviderId: string;
}

export type AIProviderType = 'openai' | 'anthropic' | 'ollama' | 'litellm' | 'openai-compatible';

export interface AIProvider {
  id: string;
  name: string;
  type: AIProviderType;
  baseUrl: string;
  apiKey: string;
  model: string;
  models: string[];
  enabled: boolean;
}

export interface ChatMessage {
  role: 'user' | 'assistant' | 'system';
  content: string;
  timestamp: number;
}

export type ChatMode = 'chat' | 'plan' | 'agent' | 'ask';

export interface ChatSession {
  id: string;
  title: string;
  mode: ChatMode;
  messages: ChatMessage[];
  createdAt: number;
}

export interface PtySessionInfo {
  id: string;
  pid: number;
  shell: string;
}

export interface PtyOutput {
  session_id: string;
  data: string;
}

export interface TerminalTab {
  id: string;
  title: string;
  sessionId: string | null;
  isActive: boolean;
}

export interface SplitPaneConfig {
  direction: 'horizontal' | 'vertical';
  ratio: number;
  children: (SplitPaneConfig | string)[];
}

export interface CpuInfo {
  brand: string;
  cores: number;
  usage: number[];
  frequency: number;
  global_usage: number;
}

export interface MemoryInfo {
  total: number;
  used: number;
  available: number;
  swap_total: number;
  swap_used: number;
  usage_percent: number;
}

export interface NetworkInfo {
  interfaces: NetworkInterface[];
  total_received: number;
  total_transmitted: number;
}

export interface NetworkInterface {
  name: string;
  received: number;
  transmitted: number;
  mac: string;
}

export interface ProcessInfo {
  pid: number;
  name: string;
  cpu_usage: number;
  memory: number;
  status: string;
}

export interface DiskInfo {
  name: string;
  mount_point: string;
  total_space: number;
  available_space: number;
  file_system: string;
  usage_percent: number;
}

export interface FileEntry {
  name: string;
  is_dir: boolean;
  size: number;
  modified: string;
}

export interface ContainerInfo {
  id: string;
  name: string;
  image: string;
  state: string;
  status: string;
  ports: string[];
  created: number;
}

export interface ImageInfo {
  id: string;
  tags: string[];
  size: number;
  created: number;
}

export interface GitBranch {
  name: string;
  is_head: boolean;
  is_remote: boolean;
  commit_id: string;
}

export interface GitCommit {
  id: string;
  short_id: string;
  message: string;
  author: string;
  email: string;
  time: number;
  parents: string[];
}

export interface GitStatus {
  staged: string[];
  unstaged: string[];
  untracked: string[];
  branch: string;
  ahead: number;
  behind: number;
}

export interface FileDiff {
  path: string;
  additions: number;
  deletions: number;
  hunks: DiffHunk[];
}

export interface DiffHunk {
  header: string;
  lines: DiffLine[];
}

export interface DiffLine {
  content: string;
  origin: string;
  old_lineno: number | null;
  new_lineno: number | null;
}

export interface SSHProfile {
  id: string;
  name: string;
  host: string;
  port: number;
  username: string;
  auth_method: 'Password' | 'PrivateKey' | 'Agent';
  private_key_path?: string;
}

export interface PluginManifest {
  id: string;
  name: string;
  version: string;
  description: string;
  author: string;
  entry: string;
  permissions: string[];
  hooks: string[];
}

export interface PluginInfo {
  manifest: PluginManifest;
  enabled: boolean;
  path: string;
}

export interface AIRequest {
  prompt: string;
  provider: string;
  model: string;
  api_key: string;
  context?: string;
}

export interface AIResponse {
  content: string;
  provider: string;
  model: string;
}

export interface KeyboardLayout {
  name: string;
  keys: KeyRow[];
}

export interface KeyRow {
  row: KeyDef[];
}

export interface KeyDef {
  key: string;
  label?: string;
  width?: number;
  action?: string;
}

export type PanelPosition = 'left' | 'right' | 'bottom' | 'center';

// ─── Kubernetes ──────────────────────────────────

export interface K8sContext {
  name: string;
  cluster: string;
  user: string;
  namespace: string;
  is_current: boolean;
}

export interface K8sPod {
  name: string;
  namespace: string;
  status: string;
  ready: string;
  restarts: number;
  age: string;
  node: string;
  containers: string[];
}

export interface K8sDeployment {
  name: string;
  namespace: string;
  ready: string;
  up_to_date: number;
  available: number;
  age: string;
  replicas: number;
}

export interface K8sService {
  name: string;
  namespace: string;
  svc_type: string;
  cluster_ip: string;
  external_ip: string;
  ports: string;
  age: string;
}

export interface K8sNamespace {
  name: string;
  status: string;
  age: string;
}
