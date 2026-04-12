<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { Terminal as XTerm } from '@xterm/xterm';
  import { FitAddon } from '@xterm/addon-fit';
  import { WebglAddon } from '@xterm/addon-webgl';
  import { WebLinksAddon } from '@xterm/addon-web-links';
  import { ptySpawn, ptyWrite, ptyResize, ptyKill, ptyGetCwd, onPtyOutput, onPtyExit } from '$lib/utils/ipc';
  import type { PtySessionInfo, Theme } from '$lib/types';

  let { theme = null, fontSize = 14, onsession, oncwd }: {
    theme?: Theme | null;
    fontSize?: number;
    onsession?: (sessionId: string) => void;
    oncwd?: (path: string) => void;
  } = $props();

  let container: HTMLDivElement;
  let xterm: XTerm;
  let fitAddon: FitAddon;
  let session: PtySessionInfo | null = null;
  let unlistenOutput: (() => void) | null = null;
  let unlistenExit: (() => void) | null = null;
  let resizeObserver: ResizeObserver | null = null;

  onMount(async () => {
    const termTheme = theme?.terminal ? {
      background: theme.terminal.background,
      foreground: theme.terminal.foreground,
      cursor: theme.terminal.cursor,
      selectionBackground: theme.terminal.selectionBackground,
      black: theme.terminal.black,
      red: theme.terminal.red,
      green: theme.terminal.green,
      yellow: theme.terminal.yellow,
      blue: theme.terminal.blue,
      magenta: theme.terminal.magenta,
      cyan: theme.terminal.cyan,
      white: theme.terminal.white,
      brightBlack: theme.terminal.brightBlack,
      brightRed: theme.terminal.brightRed,
      brightGreen: theme.terminal.brightGreen,
      brightYellow: theme.terminal.brightYellow,
      brightBlue: theme.terminal.brightBlue,
      brightMagenta: theme.terminal.brightMagenta,
      brightCyan: theme.terminal.brightCyan,
      brightWhite: theme.terminal.brightWhite,
    } : {
      background: '#091833',
      foreground: '#00d4ff',
      cursor: '#00ffff',
      selectionBackground: 'rgba(0, 212, 255, 0.3)',
    };

    xterm = new XTerm({
      theme: termTheme,
      fontSize,
      fontFamily: "'Fira Code', 'Cascadia Code', 'JetBrains Mono', monospace",
      cursorBlink: true,
      cursorStyle: 'block',
      scrollback: 10000,
      allowTransparency: true,
      drawBoldTextInBrightColors: true,
      macOptionIsMeta: true,
    });

    fitAddon = new FitAddon();
    xterm.loadAddon(fitAddon);
    xterm.loadAddon(new WebLinksAddon());

    xterm.open(container);

    // OSC 7: shell reports CWD on directory change (zsh/bash on macOS)
    xterm.parser.registerOscHandler(7, (data) => {
      try {
        const url = new URL(data);
        oncwd?.(decodeURIComponent(url.pathname));
      } catch {
        const match = data.match(/file:\/\/[^/]*(\/.*)/);
        if (match) oncwd?.(decodeURIComponent(match[1]));
      }
      return false;
    });

    try {
      xterm.loadAddon(new WebglAddon());
    } catch {
      // WebGL not available, fall back to canvas
    }

    fitAddon.fit();

    try {
      session = await ptySpawn(xterm.rows, xterm.cols);
      onsession?.(session.id);

      unlistenOutput = await onPtyOutput((data) => {
        if (data.session_id === session?.id) {
          xterm.write(data.data);
        }
      });

      unlistenExit = await onPtyExit((sid) => {
        if (sid === session?.id) {
          xterm.write('\r\n\x1b[31m[Process exited]\x1b[0m\r\n');
        }
      });

      let cwdTimer: ReturnType<typeof setTimeout> | null = null;
      xterm.onData((data) => {
        if (session) {
          ptyWrite(session.id, data);
          // After Enter key, check CWD after a short delay (fallback for no OSC 7)
          if (data === '\r' || data === '\n') {
            if (cwdTimer) clearTimeout(cwdTimer);
            cwdTimer = setTimeout(async () => {
              try {
                if (session) {
                  const cwd = await ptyGetCwd(session.id);
                  if (cwd) oncwd?.(cwd);
                }
              } catch {}
            }, 500);
          }
        }
      });

      xterm.onResize(({ rows, cols }) => {
        if (session) {
          ptyResize(session.id, rows, cols);
        }
      });
    } catch (e) {
      xterm.write(`\x1b[31mFailed to spawn terminal: ${e}\x1b[0m\r\n`);
    }

    resizeObserver = new ResizeObserver(() => {
      fitAddon.fit();
    });
    resizeObserver.observe(container);
  });

  onDestroy(() => {
    if (session) ptyKill(session.id);
    unlistenOutput?.();
    unlistenExit?.();
    resizeObserver?.disconnect();
    xterm?.dispose();
  });
</script>

<div bind:this={container} class="w-full h-full"></div>

<style>
  :global(.xterm) {
    padding: 4px;
    height: 100%;
  }
  :global(.xterm-viewport) {
    overflow-y: auto !important;
  }
  :global(.xterm-viewport::-webkit-scrollbar) {
    width: 4px;
  }
  :global(.xterm-viewport::-webkit-scrollbar-thumb) {
    background: var(--color-border);
    border-radius: 2px;
  }
</style>
