# NEXTERM

NEXTERM is an AI-powered terminal emulator and DevOps desktop workspace built with Tauri + Svelte.

It is a modern successor-inspired project based on the spirit of [eDEX-UI](https://github.com/GitSquared/edex-ui), redesigned for real developer workflows: terminal, Docker, Kubernetes, Git, SSH, and AI chat in one screen.

## Screenshots

![NEXTERM Main UI](docs/screenshots/main-ui.jpg)
![NEXTERM AI Chat](docs/screenshots/ai-chat.jpg)

## Features

- Multi-tab terminal with inline rename
- AI Chat with modes: Chat, Plan, Agent, Ask
- Multi-provider AI support: LiteLLM, OpenAI, Anthropic, Ollama, OpenAI-compatible
- Docker container/image management (including exec)
- Kubernetes panel (contexts, namespaces, pods, deployments, services)
- Git status/log/branch panel synced with terminal CWD
- SSH profile manager (private key path support)
- Sci-fi dashboard with system monitoring

## Tech Stack

- Tauri v2 (Rust backend)
- Svelte 5 + TypeScript (frontend)
- Vite 6
- xterm.js
- Three.js

## Development

```bash
npm install
npm run tauri dev
```

## Build

```bash
npm run tauri build
```

## Install with Homebrew

This repository also acts as a Homebrew tap.

```bash
brew tap musanmaz/nexterm
brew install nexterm
```

Then run:

```bash
nexterm
```

## Release

Releases are published on GitHub and include a prebuilt macOS binary archive used by the Homebrew formula.

Latest release page:

- https://github.com/musanmaz/nexterm/releases

## Project Structure

```text
.
├── src/                 # Svelte frontend
├── src-tauri/           # Rust backend
├── Formula/             # Homebrew formula
├── docs/screenshots/    # README screenshots
└── static/
```

## Credits

- Original inspiration: [eDEX-UI](https://github.com/GitSquared/edex-ui) by GitSquared
- Built with [Tauri](https://tauri.app/), [Svelte](https://svelte.dev/), and [xterm.js](https://xtermjs.org/)

## License

MIT
# NEXTERM

> A next-generation, AI-powered terminal emulator and DevOps command center with a sci-fi interface.

**NEXTERM** is a fullscreen, cross-platform desktop application that brings together terminal emulation, system monitoring, Docker & Kubernetes management, Git visualization, SSH connections, and AI-assisted workflows — all in a single, visually striking interface inspired by science fiction.

Built from the ground up as a spiritual successor to [eDEX-UI](https://github.com/GitSquared/edex-ui), NEXTERM reimagines the concept with modern technologies and a focus on real-world DevOps productivity.

---

## Why NEXTERM?

The original [eDEX-UI](https://github.com/GitSquared/edex-ui) by [@GitSquared](https://github.com/GitSquared) was a beautiful sci-fi terminal emulator inspired by TRON Legacy. It captured the imagination of developers worldwide but was archived in 2023. NEXTERM picks up where it left off:

| | eDEX-UI | NEXTERM |
|---|---------|---------|
| **Framework** | Electron | Tauri v2 (Rust) |
| **Frontend** | Vanilla JS | Svelte 5 + TypeScript |
| **Memory** | ~500MB+ | ~80MB |
| **AI** | — | Multi-provider chat (LiteLLM, OpenAI, Anthropic, Ollama) |
| **Docker** | — | Full container & image management |
| **Kubernetes** | — | Context switching, pods, deployments, services |
| **Git** | — | Branch graph, status, diff viewer |
| **SSH** | — | Profile manager with private key support |
| **Terminal** | Single | Multi-tab with rename, exec into containers/pods |

---

## Features

### Terminal
- Multi-tab terminal emulator (up to 5 tabs) powered by **xterm.js** with WebGL rendering
- **Tab renaming** — double-click any tab to rename it
- **Command History** panel with quick actions and snippet library
- Automatic CWD tracking via OSC 7 + fallback process inspection
- Full keyboard shortcut support

### AI Chat
- **Multi-provider support** — LiteLLM Proxy, OpenAI, Anthropic, Ollama, or any OpenAI-compatible endpoint
- **Model auto-discovery** — automatically fetches available models from `/model/info` and `/v1/models`
- **4 Chat Modes:**
  - 💬 **Chat** — General conversation and command help
  - 📋 **Plan** — Architecture planning and step-by-step workflows
  - 🤖 **Agent** — Direct command execution, action-oriented
  - ❓ **Ask** — Technical Q&A and concept explanations
- **Multi-session** — Create, switch between, and manage multiple conversations
- **Model switching** — Change models on-the-fly from the chat header
- **Command extraction** — Code blocks in responses get ▶ RUN buttons to execute directly in terminal
- **opencode.json import** — Import provider configs from [OpenCode](https://opencode.ai)
- Sessions persisted in localStorage

### System Monitoring
- Real-time **CPU** usage chart (per-core)
- **Memory** usage with swap tracking
- **Disk** usage with color-coded capacity bars
- **Network** throughput (RX/TX) live chart
- **Top Processes** table
- Interactive **3D Globe** (Three.js)
- All panels are independently collapsible

### Docker
- Container list with state indicators (running/stopped/paused)
- **Start**, **Stop**, **Restart**, **Remove** actions per container
- **Shell into container** — opens a new terminal tab with `docker exec -it`
- Image list with size info and **Delete** capability
- Real-time availability detection

### Kubernetes
- **Context switcher** — list and switch between kubeconfig contexts
- **Namespace selector** — filter resources by namespace
- **Pods** — status, ready count, restarts, age, node info
  - **SH** — exec into running pods in a new terminal tab
  - **LOG** — tail pod logs in a new terminal tab
  - Delete pods
- **Deployments** — ready replicas, inline **scale** control, **rollout restart**
- **Services** — type, cluster IP, external IP, ports

### Git
- **Branch list** with current branch indicator
- **Commit log** with hash, author, date
- **Status** — staged, modified, untracked files
- **Auto-sync with terminal CWD** — navigating in the terminal automatically updates the Git panel
- Manual repository path input

### SSH
- Save connection profiles with **host, port, username, auth method**
- **Private key path** support (`~/.ssh/id_rsa`, `~/.ssh/id_ed25519`, etc.)
- **Connect** button opens a new terminal tab and runs `ssh` with the correct flags
- Edit and delete profiles

### File Explorer
- CWD-tracking file browser in the bottom panel
- Click filenames to insert them into the active terminal

### Appearance
- 5 built-in themes: **Tron** (default), **Blade**, **Matrix**, **Nord**, **Cyberpunk**
- Adjustable font size
- Sound effects with volume control
- Full sci-fi aesthetic with CSS custom properties

---

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Desktop Shell | **Tauri v2** (Rust backend) |
| Frontend | **Svelte 5** + TypeScript |
| Build | **Vite 6** |
| Terminal | **xterm.js v5** + WebGL renderer |
| Styling | Tailwind CSS v4 + inline styles + CSS custom properties |
| System Info | `sysinfo` (Rust) |
| Terminal PTY | `portable-pty` (Rust) |
| Docker | `bollard` (Rust) |
| Git | `git2` (Rust) |
| Kubernetes | `kubectl` CLI (JSON parsing) |
| AI Chat | Direct HTTP (`fetch`) to OpenAI-compatible APIs |
| 3D Globe | **Three.js** |
| Audio | **Howler.js** |
| Date/Time | `chrono` (Rust) |

---

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (1.70+)
- [Node.js](https://nodejs.org/) (18+)
- Platform-specific dependencies for Tauri: see [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/)
- (Optional) Docker for container management
- (Optional) `kubectl` for Kubernetes features

### Install & Run

```bash
# Clone the repository
git clone https://github.com/your-username/nexterm.git
cd nexterm

# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

### AI Setup

1. Open **Settings** (⚙ in sidebar or `Ctrl+,`)
2. Go to **PROVIDERS** tab
3. Click **+ ADD**
4. Select provider type (LiteLLM, OpenAI, Anthropic, Ollama, etc.)
5. Enter Base URL and API Key
6. Click **⚡ TEST CONNECTION** to verify
7. Click **↻ DISCOVER MODELS** to auto-fetch available models
8. Select a model and **SAVE**

Or import directly from OpenCode:
1. Click **📋 IMPORT FROM OPENCODE.JSON** at the bottom
2. Paste your `opencode.json` content
3. Click **IMPORT**

---

## Project Structure

```
nexterm/
├── src-tauri/                 # Rust backend
│   └── src/
│       ├── lib.rs             # Tauri app setup + command registration
│       ├── pty/               # PTY session management
│       ├── system/            # System monitoring (CPU, RAM, disk, network)
│       ├── docker/            # Docker container & image management
│       ├── kubernetes/        # Kubernetes operations via kubectl
│       ├── git/               # Git operations (libgit2)
│       ├── ssh/               # SSH profile storage
│       ├── plugins/           # Plugin system
│       └── ai/                # AI provider integration
├── src/                       # Svelte frontend
│   ├── lib/
│   │   ├── components/
│   │   │   ├── terminal/      # Terminal, Tabs, AIAssistant, CommandHistory
│   │   │   ├── system/        # CPU, Memory, Network charts
│   │   │   ├── docker/        # Docker panel & container list
│   │   │   ├── kubernetes/    # Kubernetes panel
│   │   │   ├── git/           # Git panel
│   │   │   ├── ssh/           # SSH manager
│   │   │   ├── sidebar/       # Navigation sidebar, clock, globe
│   │   │   ├── filesystem/    # File explorer
│   │   │   ├── settings/      # Settings modal
│   │   │   └── shared/        # Panel, Modal, Chart, BootScreen
│   │   ├── stores/            # Svelte 5 reactive stores (.svelte.ts)
│   │   ├── utils/             # IPC, AI chat, model discovery, audio, keybindings
│   │   └── types/             # TypeScript definitions
│   └── routes/                # SvelteKit pages
├── static/
│   ├── themes/                # Theme JSON files
│   └── sounds/                # Sound effect files
└── package.json
```

---

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+Shift+T` | New terminal tab |
| `Ctrl+,` | Open settings |
| Double-click tab | Rename terminal tab |

---

## Themes

Built-in: **Tron** (default), **Blade**, **Matrix**, **Nord**, **Cyberpunk**

Themes are JSON files in `static/themes/` defining colors for the UI, terminal emulator, and 3D globe.

---

## Credits

- [eDEX-UI](https://github.com/GitSquared/edex-ui) by [@GitSquared](https://github.com/GitSquared) — The original sci-fi terminal emulator that started it all
- [Tauri](https://tauri.app) — Lightweight desktop app framework
- [Svelte](https://svelte.dev) — Reactive UI framework
- [xterm.js](https://xtermjs.org) — Terminal emulator component
- [Three.js](https://threejs.org) — 3D graphics
- [LiteLLM](https://litellm.ai) — LLM proxy for unified model access
- [OpenCode](https://opencode.ai) — AI coding assistant (config format inspiration)

---

## License

GPLv3.0 — See [LICENSE](LICENSE)

---

<p align="center">
  <strong>NEXTERM</strong> — Where science fiction meets DevOps.
</p>
