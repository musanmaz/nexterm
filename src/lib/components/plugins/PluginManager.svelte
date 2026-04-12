<script lang="ts">
  import type { PluginInfo } from '$lib/types';
  import { pluginToggle, pluginScan } from '$lib/utils/ipc';

  let { plugins = [] }: { plugins?: PluginInfo[] } = $props();

  let scanning = $state(false);

  async function scan() {
    scanning = true;
    try {
      await pluginScan();
    } finally {
      scanning = false;
    }
  }

  async function toggle(id: string, enabled: boolean) {
    await pluginToggle(id, enabled);
  }
</script>

<div class="h-full flex flex-col">
  <div class="flex items-center justify-between p-2 border-b border-[var(--color-border)]">
    <span class="text-[10px] tracking-widest text-[var(--color-text)]">PLUGINS</span>
    <button
      class="px-2 py-1 text-[10px] bg-[var(--color-primary)] text-[var(--color-bg-primary)] border-none cursor-pointer disabled:opacity-50"
      onclick={scan}
      disabled={scanning}
    >{scanning ? 'SCANNING...' : 'SCAN'}</button>
  </div>
  <div class="flex-1 overflow-y-auto p-1">
    {#each plugins as plugin}
      <div class="flex items-center gap-2 p-2 border-b border-[var(--color-border)]">
        <div class="flex-1">
          <div class="text-[11px] text-[var(--color-text-bright)]">{plugin.manifest.name}</div>
          <div class="text-[9px] text-[var(--color-text)] opacity-60">{plugin.manifest.description}</div>
          <div class="text-[8px] text-[var(--color-text)] opacity-40">v{plugin.manifest.version} by {plugin.manifest.author}</div>
        </div>
        <button
          class="px-2 py-1 text-[9px] border cursor-pointer transition-all"
          class:bg-[var(--color-success)]={plugin.enabled}
          class:text-black={plugin.enabled}
          class:border-[var(--color-success)]={plugin.enabled}
          class:bg-transparent={!plugin.enabled}
          class:text-[var(--color-text)]={!plugin.enabled}
          class:border-[var(--color-border)]={!plugin.enabled}
          onclick={() => toggle(plugin.manifest.id, !plugin.enabled)}
        >
          {plugin.enabled ? 'ON' : 'OFF'}
        </button>
      </div>
    {/each}
    {#if plugins.length === 0}
      <div class="text-center text-[10px] text-[var(--color-text)] opacity-50 py-8">
        <div class="text-2xl mb-2 opacity-30">⚡</div>
        No plugins installed
      </div>
    {/if}
  </div>
</div>
