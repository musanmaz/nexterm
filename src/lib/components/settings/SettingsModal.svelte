<script lang="ts">
  import Modal from '$lib/components/shared/Modal.svelte';
  import type { AppSettings, AIProvider, AIProviderType } from '$lib/types';
  import { discoverModels, testConnection } from '$lib/utils/model-discovery';

  let { open = $bindable(false), settings, onupdate }: {
    open: boolean;
    settings: AppSettings;
    onupdate: (settings: Partial<AppSettings>) => void;
  } = $props();

  const themes = ['tron', 'blade', 'matrix', 'nord', 'cyberpunk'];

  let activeSection = $state<'appearance' | 'audio' | 'providers'>('providers');
  let editingProvider = $state<AIProvider | null>(null);
  let isNewProvider = $state(false);
  let testResult = $state<{ ok: boolean; message: string } | null>(null);
  let testingConnection = $state(false);
  let discoveringModels = $state(false);
  let importText = $state('');
  let showImport = $state(false);
  let importMsg = $state('');

  const providerTypes: { id: AIProviderType; label: string; defaultUrl: string }[] = [
    { id: 'litellm', label: 'LiteLLM Proxy', defaultUrl: 'http://localhost:4000' },
    { id: 'openai-compatible', label: 'OpenAI Compatible', defaultUrl: 'http://localhost:8000' },
    { id: 'openai', label: 'OpenAI', defaultUrl: 'https://api.openai.com' },
    { id: 'anthropic', label: 'Anthropic', defaultUrl: 'https://api.anthropic.com' },
    { id: 'ollama', label: 'Ollama (Local)', defaultUrl: 'http://localhost:11434' },
  ];

  function startAddProvider() {
    editingProvider = {
      id: crypto.randomUUID?.() || Math.random().toString(36).slice(2),
      name: '',
      type: 'litellm',
      baseUrl: 'http://localhost:4000',
      apiKey: '',
      model: '',
      models: [],
      enabled: true,
    };
    isNewProvider = true;
    testResult = null;
  }

  function startEditProvider(p: AIProvider) {
    editingProvider = { ...p, models: [...p.models] };
    isNewProvider = false;
    testResult = null;
  }

  function saveProvider() {
    if (!editingProvider || !editingProvider.name.trim()) return;
    const providers = [...(settings.aiProviders || [])];
    const idx = providers.findIndex(p => p.id === editingProvider!.id);
    if (idx >= 0) {
      providers[idx] = editingProvider;
    } else {
      providers.push(editingProvider);
    }
    let activeId = settings.activeProviderId;
    if (providers.length === 1 || (!activeId && providers.length > 0)) {
      activeId = providers[0].id;
    }
    onupdate({ aiProviders: providers, activeProviderId: activeId });
    editingProvider = null;
  }

  function deleteProvider(id: string) {
    const providers = (settings.aiProviders || []).filter(p => p.id !== id);
    let activeId = settings.activeProviderId;
    if (activeId === id) {
      activeId = providers[0]?.id || '';
    }
    onupdate({ aiProviders: providers, activeProviderId: activeId });
  }

  function setActiveProvider(id: string) {
    onupdate({ activeProviderId: id });
  }

  function onTypeChange(type: AIProviderType) {
    if (!editingProvider) return;
    const pt = providerTypes.find(p => p.id === type);
    editingProvider = {
      ...editingProvider,
      type,
      baseUrl: pt?.defaultUrl || editingProvider.baseUrl,
      models: [],
    };
  }

  async function handleTest() {
    if (!editingProvider) return;
    testingConnection = true;
    testResult = null;
    try {
      const result = await testConnection(editingProvider);
      testResult = result;
      if (result.ok && result.models && result.models.length > 0) {
        editingProvider = { ...editingProvider, models: result.models };
        if (!editingProvider.model && result.models.length > 0) {
          editingProvider = { ...editingProvider, model: result.models[0] };
        }
      }
    } catch (e) {
      testResult = { ok: false, message: String(e) };
    } finally {
      testingConnection = false;
    }
  }

  async function handleDiscover() {
    if (!editingProvider) return;
    discoveringModels = true;
    try {
      const models = await discoverModels(editingProvider);
      editingProvider = { ...editingProvider, models };
      if (models.length > 0 && !editingProvider.model) {
        editingProvider = { ...editingProvider, model: models[0] };
      }
    } catch { /* ignore */ }
    finally { discoveringModels = false; }
  }

  function importOpencode() {
    try {
      const json = JSON.parse(importText.trim());
      const providerSection = json.provider || {};
      const providers = [...(settings.aiProviders || [])];
      let count = 0;

      for (const [key, val] of Object.entries(providerSection) as [string, any][]) {
        const baseUrl = val.options?.baseURL || val.options?.baseUrl || 'http://localhost:4000';
        const apiKeyVal = val.options?.apiKey || '';
        const modelEntries = val.models ? Object.keys(val.models) : [];
        const slash = '/';
        const rawModel = json.model || '';
        const defaultModel = rawModel.includes(slash) ? rawModel.split(slash).slice(1).join(slash) : (rawModel || modelEntries[0] || '');

        const existingIdx = providers.findIndex(p => p.name === (val.name || key));
        const newProv: AIProvider = {
          id: existingIdx >= 0 ? providers[existingIdx].id : (crypto.randomUUID?.() || Math.random().toString(36).slice(2)),
          name: val.name || key,
          type: key.includes('litellm') || val.npm?.includes('openai-compatible') ? 'litellm' : 'openai-compatible',
          baseUrl,
          apiKey: apiKeyVal,
          model: defaultModel,
          models: modelEntries,
          enabled: true,
        };

        if (existingIdx >= 0) {
          providers[existingIdx] = newProv;
        } else {
          providers.push(newProv);
        }
        count++;
      }

      let activeId = settings.activeProviderId;
      if (!activeId && providers.length > 0) activeId = providers[providers.length - 1].id;
      onupdate({ aiProviders: providers, activeProviderId: activeId });
      importMsg = `${count} provider(s) imported successfully!`;
      showImport = false;
      importText = '';
      setTimeout(() => { importMsg = ''; }, 3000);
    } catch (e) {
      importMsg = `Parse error: ${e}`;
      setTimeout(() => { importMsg = ''; }, 5000);
    }
  }

  const sectionTitle = 'font-size:11px;letter-spacing:2px;color:var(--color-primary);margin-bottom:8px;padding-bottom:4px;border-bottom:1px solid var(--color-border);';
  const labelStyle = 'display:block;font-size:10px;color:var(--color-text);letter-spacing:2px;margin-bottom:4px;';
  const inputStyle = 'width:100%;background:var(--color-bg-primary);border:1px solid var(--color-border);color:var(--color-text);font-size:12px;padding:6px 8px;font-family:inherit;outline:none;box-sizing:border-box;';

  const activeProvider = $derived((settings.aiProviders || []).find(p => p.id === settings.activeProviderId));
</script>

<Modal title="SETTINGS" bind:open>
  <div style="display:flex;flex-direction:column;gap:0;">
    <!-- Section Tabs -->
    <div style="display:flex;border-bottom:1px solid var(--color-border);margin-bottom:12px;">
      {#each [
        { id: 'providers' as const, label: '⚡ PROVIDERS' },
        { id: 'appearance' as const, label: '🎨 APPEARANCE' },
        { id: 'audio' as const, label: '🔊 AUDIO' },
      ] as sec}
        <button
          type="button"
          style="flex:1;padding:8px;font-size:9px;letter-spacing:1px;border:none;cursor:pointer;border-bottom:2px solid {activeSection === sec.id ? 'var(--color-primary)' : 'transparent'};background:{activeSection === sec.id ? 'var(--color-bg-panel)' : 'transparent'};color:{activeSection === sec.id ? 'var(--color-text-bright)' : 'var(--color-text)'};"
          onclick={() => activeSection = sec.id}
        >{sec.label}</button>
      {/each}
    </div>

    {#if activeSection === 'providers'}
      {#if editingProvider}
        <!-- Provider Edit Form -->
        <div style="display:flex;flex-direction:column;gap:10px;">
          <div style="display:flex;align-items:center;justify-content:space-between;">
            <h3 style={sectionTitle}>{isNewProvider ? '+ NEW PROVIDER' : '✎ EDIT PROVIDER'}</h3>
            <button type="button" style="padding:3px 8px;font-size:9px;background:transparent;color:var(--color-text);border:1px solid var(--color-border);cursor:pointer;" onclick={() => editingProvider = null}>← BACK</button>
          </div>

          <!-- Provider Type -->
          <div>
            <span style={labelStyle}>TYPE</span>
            <div style="display:flex;gap:4px;flex-wrap:wrap;">
              {#each providerTypes as pt}
                <button
                  type="button"
                  style="padding:4px 10px;font-size:9px;letter-spacing:1px;border:1px solid {editingProvider.type === pt.id ? 'var(--color-primary)' : 'var(--color-border)'};cursor:pointer;background:{editingProvider.type === pt.id ? 'var(--color-primary)' : 'transparent'};color:{editingProvider.type === pt.id ? 'var(--color-bg-primary)' : 'var(--color-text)'};"
                  onclick={() => onTypeChange(pt.id)}
                >{pt.label}</button>
              {/each}
            </div>
          </div>

          <!-- Name -->
          <div>
            <label for="prov-name" style={labelStyle}>NAME</label>
            <input id="prov-name" bind:value={editingProvider.name} placeholder="My LiteLLM Server" style={inputStyle} />
          </div>

          <!-- Base URL -->
          <div>
            <label for="prov-url" style={labelStyle}>BASE URL</label>
            <input id="prov-url" bind:value={editingProvider.baseUrl} placeholder="http://localhost:4000" style={inputStyle} />
            <div style="font-size:8px;color:var(--color-text);opacity:0.4;margin-top:2px;">
              {#if editingProvider.type === 'litellm'}LiteLLM proxy URL - works with or without the /v1 suffix
              {:else if editingProvider.type === 'ollama'}Ollama server URL
              {:else if editingProvider.type === 'openai'}OpenAI API (usually no need to change)
              {:else}OpenAI-compatible endpoint - works with or without the /v1 suffix
              {/if}
            </div>
          </div>

          <!-- API Key -->
          {#if editingProvider.type !== 'ollama'}
            <div>
              <label for="prov-key" style={labelStyle}>API KEY</label>
              <input id="prov-key" type="password" bind:value={editingProvider.apiKey} placeholder="sk-..." style={inputStyle} />
            </div>
          {/if}

          <!-- Test Connection -->
          <div style="display:flex;gap:6px;align-items:center;">
            <button
              type="button"
              style="padding:5px 12px;font-size:10px;background:var(--color-accent);color:#fff;border:none;cursor:pointer;letter-spacing:1px;{testingConnection ? 'opacity:0.5;' : ''}"
              onclick={handleTest}
              disabled={testingConnection}
            >{testingConnection ? 'TESTING...' : '⚡ TEST CONNECTION'}</button>
            <button
              type="button"
              style="padding:5px 12px;font-size:10px;background:var(--color-primary);color:var(--color-bg-primary);border:none;cursor:pointer;letter-spacing:1px;{discoveringModels ? 'opacity:0.5;' : ''}"
              onclick={handleDiscover}
              disabled={discoveringModels}
            >{discoveringModels ? 'LOADING...' : '↻ DISCOVER MODELS'}</button>
          </div>

          {#if testResult}
            <div style="padding:6px 10px;font-size:10px;border-radius:2px;background:{testResult.ok ? 'rgba(0,255,0,0.1)' : 'rgba(255,0,0,0.1)'};border:1px solid {testResult.ok ? 'var(--color-success)' : 'var(--color-error)'};color:{testResult.ok ? 'var(--color-success)' : 'var(--color-error)'};">
              {testResult.ok ? '✓' : '✕'} {testResult.message}
            </div>
          {/if}

          <!-- Model Selection -->
          <div>
            <label for="prov-model" style={labelStyle}>MODEL {editingProvider.models.length > 0 ? `(${editingProvider.models.length} discovered)` : ''}</label>
            {#if editingProvider.models.length > 0}
              <select
                id="prov-model"
                bind:value={editingProvider.model}
                style={inputStyle}
              >
                {#each editingProvider.models as m}
                  <option value={m}>{m}</option>
                {/each}
              </select>
            {:else}
              <input id="prov-model" bind:value={editingProvider.model} placeholder="gpt-4, llama3, claude-3..." style={inputStyle} />
              <div style="font-size:8px;color:var(--color-text);opacity:0.4;margin-top:2px;">
                Use "DISCOVER MODELS" to fetch the model list, or type one manually
              </div>
            {/if}
          </div>

          <!-- Save -->
          <button
            type="button"
            style="width:100%;padding:8px;font-size:12px;background:var(--color-success);color:#000;border:none;cursor:pointer;letter-spacing:2px;"
            onclick={saveProvider}
          >💾 SAVE PROVIDER</button>
        </div>
      {:else}
        <!-- Provider List -->
        <div style="display:flex;flex-direction:column;gap:10px;">
          <div style="display:flex;align-items:center;justify-content:space-between;">
            <h3 style={sectionTitle}>AI PROVIDERS</h3>
            <button
              type="button"
              style="padding:4px 10px;font-size:10px;background:var(--color-primary);color:var(--color-bg-primary);border:none;cursor:pointer;letter-spacing:1px;"
              onclick={startAddProvider}
            >+ ADD</button>
          </div>

          {#if !settings.aiProviders || settings.aiProviders.length === 0}
            <div style="text-align:center;padding:20px;opacity:0.4;">
              <div style="font-size:20px;margin-bottom:8px;">⚡</div>
              <div style="font-size:11px;color:var(--color-text);">No providers added yet</div>
              <div style="font-size:9px;color:var(--color-text);margin-top:4px;">
                Use "+ ADD" to add LiteLLM, OpenAI, Anthropic, or Ollama
              </div>
            </div>
          {:else}
            {#each settings.aiProviders as prov}
              <div style="
                display:flex;align-items:center;gap:8px;padding:8px 10px;
                border:1px solid {settings.activeProviderId === prov.id ? 'var(--color-primary)' : 'var(--color-border)'};
                background:{settings.activeProviderId === prov.id ? 'rgba(var(--color-primary-rgb, 0,255,255),0.05)' : 'var(--color-bg-primary)'};
                cursor:pointer;
              ">
                <!-- Radio / Active indicator -->
                <button
                  type="button"
                  style="width:16px;height:16px;border-radius:50%;border:2px solid {settings.activeProviderId === prov.id ? 'var(--color-primary)' : 'var(--color-border)'};background:{settings.activeProviderId === prov.id ? 'var(--color-primary)' : 'transparent'};cursor:pointer;flex-shrink:0;"
                  onclick={() => setActiveProvider(prov.id)}
                  title="Set as active"
                ></button>

                <!-- Info -->
                <button type="button" style="flex:1;min-width:0;background:none;border:none;cursor:pointer;text-align:left;padding:0;" onclick={() => setActiveProvider(prov.id)}>
                  <div style="font-size:11px;color:var(--color-text-bright);display:flex;align-items:center;gap:6px;">
                    <span>{prov.name}</span>
                    <span style="font-size:8px;padding:1px 6px;background:var(--color-bg-secondary);border:1px solid var(--color-border);color:var(--color-text);letter-spacing:1px;">{prov.type.toUpperCase()}</span>
                  </div>
                  <div style="font-size:9px;color:var(--color-text);opacity:0.5;margin-top:2px;">
                    {prov.model || 'No model selected'} · {prov.baseUrl}
                    {#if prov.models.length > 0}
                      · {prov.models.length} model(s)
                    {/if}
                  </div>
                </button>

                <!-- Actions -->
                <div style="display:flex;gap:3px;flex-shrink:0;">
                  <button type="button" style="padding:3px 8px;font-size:9px;background:var(--color-primary);color:#000;border:none;cursor:pointer;" onclick={() => startEditProvider(prov)} title="Edit">✎</button>
                  <button type="button" style="padding:3px 8px;font-size:9px;background:var(--color-error);color:#fff;border:none;cursor:pointer;" onclick={() => deleteProvider(prov.id)} title="Delete">✕</button>
                </div>
              </div>
            {/each}
          {/if}

          {#if activeProvider}
            <div style="padding:8px;border:1px solid var(--color-border);background:var(--color-bg-primary);font-size:9px;color:var(--color-text);">
              <span style="color:var(--color-success);">● ACTIVE:</span>
              <span style="color:var(--color-text-bright);">{activeProvider.name}</span>
              → <span style="color:var(--color-primary);">{activeProvider.model}</span>
            </div>
          {/if}

          {#if importMsg}
            <div style="padding:6px 10px;font-size:10px;border-radius:2px;background:{importMsg.includes('error') ? 'rgba(255,0,0,0.1)' : 'rgba(0,255,0,0.1)'};border:1px solid {importMsg.includes('error') ? 'var(--color-error)' : 'var(--color-success)'};color:{importMsg.includes('error') ? 'var(--color-error)' : 'var(--color-success)'};">
              {importMsg}
            </div>
          {/if}

          <!-- Import from opencode.json -->
          <div style="border-top:1px solid var(--color-border);padding-top:10px;">
            <button
              type="button"
              style="width:100%;padding:6px;font-size:9px;letter-spacing:1px;background:transparent;color:var(--color-text);border:1px dashed var(--color-border);cursor:pointer;opacity:0.6;"
              onclick={() => showImport = !showImport}
            >📋 IMPORT FROM OPENCODE.JSON</button>

            {#if showImport}
              <div style="margin-top:8px;display:flex;flex-direction:column;gap:6px;">
                <div style="font-size:8px;color:var(--color-text);opacity:0.5;">
                  Paste your opencode.json content - provider, model, and API key will be imported automatically
                </div>
                <textarea
                  bind:value={importText}
                  placeholder={'{"provider":{"litellm":{"name":"LiteLLM","options":{"baseURL":"..."}}}}'}
                  style="width:100%;height:100px;background:var(--color-bg-primary);border:1px solid var(--color-border);color:var(--color-text);font-size:10px;padding:6px;font-family:monospace;outline:none;resize:vertical;box-sizing:border-box;"
                ></textarea>
                <button
                  type="button"
                  style="width:100%;padding:6px;font-size:10px;background:var(--color-primary);color:var(--color-bg-primary);border:none;cursor:pointer;letter-spacing:1px;"
                  onclick={importOpencode}
                >IMPORT</button>
              </div>
            {/if}
          </div>
        </div>
      {/if}

    {:else if activeSection === 'appearance'}
      <div style="display:flex;flex-direction:column;gap:12px;">
        <div>
          <span style={labelStyle}>THEME</span>
          <div style="display:flex;gap:6px;flex-wrap:wrap;">
            {#each themes as theme}
              <button
                type="button"
                style="padding:4px 12px;font-size:10px;letter-spacing:1px;border:1px solid {settings.theme === theme ? 'var(--color-primary)' : 'var(--color-border)'};cursor:pointer;background:{settings.theme === theme ? 'var(--color-primary)' : 'transparent'};color:{settings.theme === theme ? 'var(--color-bg-primary)' : 'var(--color-text)'};"
                onclick={() => onupdate({ theme })}
              >{theme.toUpperCase()}</button>
            {/each}
          </div>
        </div>
        <div>
          <label for="settings-fontsize" style={labelStyle}>FONT SIZE: {settings.fontSize}px</label>
          <input
            id="settings-fontsize"
            type="range" min="10" max="24" step="1"
            value={settings.fontSize}
            oninput={(e) => onupdate({ fontSize: parseInt((e.target as HTMLInputElement).value) })}
            style="width:100%;accent-color:var(--color-primary);"
          />
        </div>
      </div>

    {:else if activeSection === 'audio'}
      <div style="display:flex;flex-direction:column;gap:12px;">
        <div style="display:flex;align-items:center;gap:12px;">
          <button
            type="button"
            style="padding:4px 12px;font-size:10px;border:1px solid {settings.audioEnabled ? 'var(--color-success)' : 'var(--color-border)'};cursor:pointer;background:{settings.audioEnabled ? 'var(--color-success)' : 'transparent'};color:{settings.audioEnabled ? '#000' : 'var(--color-text)'};"
            onclick={() => onupdate({ audioEnabled: !settings.audioEnabled })}
          >{settings.audioEnabled ? 'ON' : 'OFF'}</button>
          <input
            type="range" min="0" max="100" step="5"
            value={settings.audioVolume * 100}
            oninput={(e) => onupdate({ audioVolume: parseInt((e.target as HTMLInputElement).value) / 100 })}
            style="flex:1;accent-color:var(--color-primary);"
            disabled={!settings.audioEnabled}
          />
          <span style="font-size:10px;color:var(--color-text);width:32px;text-align:right;">{Math.round(settings.audioVolume * 100)}%</span>
        </div>
      </div>
    {/if}
  </div>
</Modal>
