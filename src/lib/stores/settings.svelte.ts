import type { AppSettings } from '$lib/types';

const defaultSettings: AppSettings = {
  theme: 'tron',
  keyboardLayout: 'qwerty',
  audioEnabled: true,
  audioVolume: 0.5,
  fontSize: 14,
  shell: '',
  aiProvider: 'ollama',
  aiApiKey: '',
  aiModel: 'llama3',
  litellmBaseUrl: 'http://localhost:4000',
  litellmApiKey: '',
  litellmModel: 'gpt-4',
  aiProviders: [
    { id: 'ollama-default', name: 'Ollama (Local)', type: 'ollama', baseUrl: 'http://localhost:11434', apiKey: '', model: 'llama3', models: [], enabled: true },
  ],
  activeProviderId: 'ollama-default',
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
