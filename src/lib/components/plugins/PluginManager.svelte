<script lang="ts">
  import type { PluginInfo } from '$lib/types';
  import { pluginToggle, pluginScan, pluginList } from '$lib/utils/ipc';
  import { onMount } from 'svelte';

  let plugins = $state<PluginInfo[]>([]);
  let scanning = $state(false);
  let loading = $state(true);

  onMount(async () => {
    try {
      plugins = await pluginList();
    } catch {
      plugins = [];
    } finally {
      loading = false;
    }
  });

  async function scan() {
    scanning = true;
    try {
      plugins = await pluginScan();
    } catch {
      // scan failed
    } finally {
      scanning = false;
    }
  }

  async function toggle(id: string, enabled: boolean) {
    await pluginToggle(id, enabled);
    try {
      plugins = await pluginList();
    } catch {
      // refresh failed
    }
  }
</script>

<div style="height:100%;display:flex;flex-direction:column;">
  <div style="display:flex;align-items:center;justify-content:space-between;padding:8px;border-bottom:1px solid var(--color-border);">
    <span style="font-size:10px;letter-spacing:2px;color:var(--color-text);">PLUGINS</span>
    <button
      style="padding:4px 12px;font-size:10px;background:var(--color-primary);color:var(--color-bg-primary);border:none;cursor:pointer;letter-spacing:1px;opacity:{scanning ? '0.5' : '1'};"
      onclick={scan}
      disabled={scanning}
    >{scanning ? 'SCANNING...' : 'SCAN'}</button>
  </div>
  <div style="flex:1;overflow-y:auto;padding:4px;">
    {#if loading}
      <div style="text-align:center;font-size:10px;color:var(--color-text);opacity:0.5;padding:32px 0;">Loading plugins...</div>
    {:else}
      {#each plugins as plugin}
        <div style="display:flex;align-items:center;gap:8px;padding:8px;border-bottom:1px solid var(--color-border);">
          <div style="flex:1;min-width:0;">
            <div style="font-size:11px;color:var(--color-text-bright);">{plugin.manifest.name}</div>
            <div style="font-size:9px;color:var(--color-text);opacity:0.6;">{plugin.manifest.description}</div>
            <div style="font-size:8px;color:var(--color-text);opacity:0.4;">v{plugin.manifest.version} by {plugin.manifest.author}</div>
          </div>
          <button
            style="
              padding:4px 10px;font-size:9px;cursor:pointer;transition:all 0.2s;
              background:{plugin.enabled ? 'var(--color-success)' : 'transparent'};
              color:{plugin.enabled ? 'var(--color-bg-primary)' : 'var(--color-text)'};
              border:1px solid {plugin.enabled ? 'var(--color-success)' : 'var(--color-border)'};
            "
            onclick={() => toggle(plugin.manifest.id, !plugin.enabled)}
          >
            {plugin.enabled ? 'ON' : 'OFF'}
          </button>
        </div>
      {/each}
      {#if plugins.length === 0}
        <div style="text-align:center;font-size:10px;color:var(--color-text);opacity:0.5;padding:32px 0;">
          <div style="font-size:24px;margin-bottom:8px;opacity:0.3;">⚡</div>
          No plugins installed
        </div>
      {/if}
    {/if}
  </div>
</div>
