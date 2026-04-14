import type { AppSettings } from '$lib/types';

const defaultSettings: AppSettings = {
  theme: 'tron',
  keyboardLayout: 'qwerty',
  audioEnabled: true,
  audioVolume: 0.5,
  fontSize: 14,
  shell: '',
  aiProvider: '',
  aiApiKey: '',
  aiModel: '',
  litellmBaseUrl: '',
  litellmApiKey: '',
  litellmModel: '',
  aiProviders: [],
  activeProviderId: '',
  terminalProfiles: [],
};

let settings = $state<AppSettings>({ ...defaultSettings });

export function getSettingsStore() {
  function update(partial: Partial<AppSettings>) {
    settings = { ...settings, ...partial };
    save();
  }

  function save() {
    try {
      localStorage.setItem('nexterm-settings', JSON.stringify(settings));
    } catch { /* noop */ }
  }

  function load() {
    try {
      const stored = localStorage.getItem('nexterm-settings');
      if (stored) {
        settings = { ...defaultSettings, ...JSON.parse(stored) };
      }
    } catch { /* noop */ }
  }

  function reset() {
    settings = { ...defaultSettings };
    save();
  }

  return {
    get current() { return settings; },
    update,
    load,
    save,
    reset,
  };
}
