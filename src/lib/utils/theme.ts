import type { Theme } from '$lib/types';

export function applyTheme(theme: Theme): void {
  const root = document.documentElement;
  const c = theme.colors;
  root.style.setProperty('--color-primary', c.primary);
  root.style.setProperty('--color-secondary', c.secondary);
  root.style.setProperty('--color-accent', c.accent);
  root.style.setProperty('--color-bg-primary', c.bgPrimary);
  root.style.setProperty('--color-bg-secondary', c.bgSecondary);
  root.style.setProperty('--color-bg-panel', c.bgPanel);
  root.style.setProperty('--color-text', c.text);
  root.style.setProperty('--color-text-bright', c.textBright);
  root.style.setProperty('--color-border', c.border);
  root.style.setProperty('--color-border-bright', c.borderBright);
  root.style.setProperty('--color-success', c.success);
  root.style.setProperty('--color-warning', c.warning);
  root.style.setProperty('--color-error', c.error);
}

export async function loadTheme(name: string): Promise<Theme> {
  const response = await fetch(`/themes/${name}.json`);
  if (!response.ok) throw new Error(`Theme '${name}' not found`);
  return response.json();
}

export function getTerminalTheme(theme: Theme) {
  const t = theme.terminal;
  return {
    background: t.background,
    foreground: t.foreground,
    cursor: t.cursor,
    selectionBackground: t.selectionBackground,
    black: t.black,
    red: t.red,
    green: t.green,
    yellow: t.yellow,
    blue: t.blue,
    magenta: t.magenta,
    cyan: t.cyan,
    white: t.white,
    brightBlack: t.brightBlack,
    brightRed: t.brightRed,
    brightGreen: t.brightGreen,
    brightYellow: t.brightYellow,
    brightBlue: t.brightBlue,
    brightMagenta: t.brightMagenta,
    brightCyan: t.brightCyan,
    brightWhite: t.brightWhite,
  };
}
