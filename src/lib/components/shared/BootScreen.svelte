<script lang="ts">
  const bootLog = [
    'NEXTERM v1.0.0 -- Initializing system...',
    '[OK] Loading Tauri runtime v2.0',
    '[OK] Rust backend initialized',
    '[OK] PTY subsystem ready',
    '[OK] System monitor started',
    '[OK] Network interfaces detected',
    '[OK] Loading theme: tron',
    '[OK] CSS variables applied',
    '[OK] WebGL renderer initialized',
    '[OK] Font subsystem loaded',
    '[OK] Audio engine ready',
    '[OK] Keybinding manager initialized',
    '[OK] Docker API connection check...',
    '[OK] Git integration loaded',
    '[OK] SSH module initialized',
    '[OK] Plugin loader scanning...',
    '[OK] AI provider configured',
    '[OK] Terminal emulator ready',
    '[OK] xterm.js v5 loaded with WebGL addon',
    '[OK] Virtual keyboard layout: QWERTY',
    '[OK] File explorer initialized',
    '[OK] System info: sysinfo crate active',
    '[OK] IPC channels established',
    '[OK] Security sandbox active',
    '[OK] All modules loaded successfully',
    '',
    '========================================',
    '',
    'NEXTERM -- Ready.',
    'Welcome to the future.',
  ];

  const lineDelay = 60;
  const totalAnimMs = bootLog.length * lineDelay;
  const fadeStartMs = totalAnimMs + 800;
  const totalMs = fadeStartMs + 600;
</script>

<div class="boot-overlay" style="pointer-events: none; --total: {totalMs}ms; --fade-start: {fadeStartMs}ms">
  <div class="boot-inner">
    <div class="boot-header">
      <h1 class="boot-title glow-text">NEXTERM</h1>
      <p class="boot-version">v3.0.0</p>
    </div>

    <div class="boot-log sci-fi-panel">
      {#each bootLog as line, i}
        <div
          class="boot-line"
          class:boot-ok={line.startsWith('[OK]')}
          class:boot-center={line.includes('====') || line.includes('Ready') || line.includes('Welcome')}
          class:boot-glow={line.includes('Ready')}
          style="animation-delay: {i * lineDelay}ms"
        >{line || '\u00A0'}</div>
      {/each}
      <span class="boot-cursor"></span>
    </div>

    <div class="boot-progress-track">
      <div class="boot-progress-bar" style="animation-duration: {totalAnimMs}ms"></div>
    </div>
    <p class="boot-progress-text">LOADING...</p>
  </div>
</div>

<style>
  .boot-overlay {
    position: fixed;
    inset: 0;
    z-index: 100;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    background: var(--color-bg-primary);
    pointer-events: none;
    animation: bootFade 500ms var(--fade-start) forwards;
  }
  .boot-inner {
    width: 700px;
    max-width: 90vw;
  }
  .boot-header {
    text-align: center;
    margin-bottom: 2rem;
  }
  .boot-title {
    font-size: 1.875rem;
    font-weight: bold;
    letter-spacing: 8px;
  }
  .boot-version {
    font-size: 0.75rem;
    color: var(--color-text);
    margin-top: 0.5rem;
    letter-spacing: 4px;
  }
  .boot-log {
    height: 400px;
    overflow-y: auto;
    font-family: var(--font-mono);
    font-size: 0.75rem;
    line-height: 1.25rem;
    margin-bottom: 1.5rem;
    padding: 1rem;
  }
  .boot-line {
    color: var(--color-text);
    opacity: 0;
    animation: bootReveal 0.05s forwards;
  }
  .boot-ok { color: var(--color-success); }
  .boot-center { text-align: center; }
  .boot-glow {
    color: var(--color-text-bright);
    text-shadow: var(--glow-primary);
  }
  .boot-cursor {
    display: inline-block;
    width: 0.5rem;
    height: 1rem;
    background: var(--color-text-bright);
    animation: blink 1s step-end infinite;
  }
  .boot-progress-track {
    width: 100%;
    height: 4px;
    background: var(--color-bg-secondary);
    border-radius: 2px;
    overflow: hidden;
  }
  .boot-progress-bar {
    height: 100%;
    background: var(--color-primary);
    box-shadow: var(--glow-primary);
    width: 0%;
    animation: progressFill forwards linear;
  }
  .boot-progress-text {
    text-align: center;
    font-size: 10px;
    color: var(--color-text);
    margin-top: 0.5rem;
    letter-spacing: 0.1em;
  }

  @keyframes bootReveal {
    to { opacity: 1; }
  }
  @keyframes bootFade {
    to { opacity: 0; visibility: hidden; pointer-events: none; }
  }
  @keyframes blink {
    50% { opacity: 0; }
  }
  @keyframes progressFill {
    to { width: 100%; }
  }
</style>
