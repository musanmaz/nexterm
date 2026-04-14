import { applyTheme, loadTheme } from '$lib/utils/theme';
import type { Theme } from '$lib/types';

let currentTheme = $state<Theme | null>(null);
let themeName = $state('tron');
let loading = $state(false);

export function getThemeStore() {
  async function setTheme(name: string) {
    loading = true;
    try {
      const theme = await loadTheme(name);
      currentTheme = theme;
      themeName = name;
      applyTheme(theme);
    } catch (e) {
      console.error('Failed to load theme:', e);
    } finally {
      loading = false;
    }
  }

  function applyCustomTheme(theme: Theme) {
    currentTheme = theme;
    themeName = theme.name;
    applyTheme(theme);
  }

  return {
    get current() { return currentTheme; },
    get name() { return themeName; },
    get loading() { return loading; },
    setTheme,
    applyTheme: applyCustomTheme,
  };
}
