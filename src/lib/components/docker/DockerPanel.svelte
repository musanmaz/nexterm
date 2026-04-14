<script lang="ts">
  import ContainerList from './ContainerList.svelte';
  import type { ContainerInfo, ImageInfo } from '$lib/types';
  import { dockerStartContainer, dockerStopContainer, dockerRestartContainer, dockerRemoveContainer, dockerRemoveImage, dockerContainerLogs } from '$lib/utils/ipc';

  let { containers = [], images = [], available = false, onrefresh, onexec }: {
    containers?: ContainerInfo[];
    images?: ImageInfo[];
    available?: boolean;
    onrefresh?: () => void;
    onexec?: (name: string, containerId: string) => void;
  } = $props();

  let activeTab = $state<'containers' | 'images' | 'logs'>('containers');
  let actionMsg = $state('');
  let logsContent = $state('');
  let logsContainer = $state('');
  let logsLoading = $state(false);

  async function handleAction(id: string, action: string) {
    actionMsg = '';
    try {
      if (action === 'start') await dockerStartContainer(id);
      else if (action === 'stop') await dockerStopContainer(id);
      else if (action === 'restart') await dockerRestartContainer(id);
      else if (action === 'remove') await dockerRemoveContainer(id, false);
      onrefresh?.();
    } catch (e) {
      actionMsg = `Error: ${e}`;
      setTimeout(() => { actionMsg = ''; }, 3000);
    }
  }

  async function removeImage(id: string) {
    actionMsg = '';
    try {
      await dockerRemoveImage(id, false);
      onrefresh?.();
    } catch (e) {
      actionMsg = `Error: ${e}`;
      setTimeout(() => { actionMsg = ''; }, 3000);
    }
  }

  async function viewLogs(id: string, name: string) {
    logsContainer = name;
    logsContent = '';
    logsLoading = true;
    activeTab = 'logs';
    try {
      logsContent = await dockerContainerLogs(id, '300');
    } catch (e) {
      logsContent = `Error fetching logs: ${e}`;
    } finally {
      logsLoading = false;
    }
  }

  function formatSize(bytes: number): string {
    if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(0) + ' KB';
    if (bytes < 1024 * 1024 * 1024) return (bytes / (1024 * 1024)).toFixed(0) + ' MB';
    return (bytes / (1024 * 1024 * 1024)).toFixed(1) + ' GB';
  }
</script>

<div style="height:100%;display:flex;flex-direction:column;">
  {#if !available}
    <div style="flex:1;display:flex;align-items:center;justify-content:center;">
      <div style="text-align:center;">
        <span style="font-size:24px;display:block;margin-bottom:8px;opacity:0.3;">⬡</span>
        <span style="font-size:11px;color:var(--color-text);opacity:0.5;">Docker not available</span>
      </div>
    </div>
  {:else}
    <div style="display:flex;gap:4px;padding:8px;border-bottom:1px solid var(--color-border);align-items:center;">
      {#each [['containers', `CONTAINERS (${containers.length})`], ['images', `IMAGES (${images.length})`], ['logs', 'LOGS']] as [tab, label]}
        <button
          type="button"
          style="
            padding:4px 10px;font-size:10px;letter-spacing:1px;cursor:pointer;transition:all 0.2s;
            background:{activeTab === tab ? 'var(--color-primary)' : 'transparent'};
            color:{activeTab === tab ? 'var(--color-bg-primary)' : 'var(--color-text)'};
            border:1px solid {activeTab === tab ? 'var(--color-primary)' : 'var(--color-border)'};
          "
          onclick={() => activeTab = tab as typeof activeTab}
        >{label}</button>
      {/each}
      <div style="flex:1;"></div>
      <button
        type="button"
        style="padding:4px 8px;font-size:10px;color:var(--color-text);background:transparent;border:1px solid var(--color-border);cursor:pointer;"
        onclick={onrefresh}
      >↻</button>
    </div>

    {#if actionMsg}
      <div style="padding:4px 8px;font-size:9px;color:var(--color-error);background:rgba(255,0,60,0.1);border-bottom:1px solid var(--color-border);">
        {actionMsg}
      </div>
    {/if}

    <div style="flex:1;overflow-y:auto;">
      {#if activeTab === 'containers'}
        <ContainerList {containers} onaction={handleAction} {onexec} onlogs={viewLogs} />
      {:else if activeTab === 'images'}
        <div style="padding:4px;">
          {#each images as image}
            <div style="display:flex;align-items:center;gap:6px;padding:6px;border-bottom:1px solid var(--color-border);">
              <div style="flex:1;min-width:0;overflow:hidden;">
                <div style="font-size:11px;color:var(--color-text-bright);overflow:hidden;text-overflow:ellipsis;white-space:nowrap;">
                  {image.tags[0] || image.id}
                </div>
                <div style="font-size:9px;color:var(--color-text);opacity:0.5;">
                  {formatSize(image.size)} · {image.id.slice(0, 12)}
                </div>
              </div>
              <button
                type="button"
                style="padding:3px 8px;font-size:9px;background:var(--color-error);color:#fff;border:none;cursor:pointer;flex-shrink:0;"
                onclick={() => removeImage(image.id)}
              >✕ DEL</button>
            </div>
          {/each}
          {#if images.length === 0}
            <div style="text-align:center;font-size:10px;color:var(--color-text);opacity:0.5;padding:16px;">No images</div>
          {/if}
        </div>
      {:else if activeTab === 'logs'}
        <div style="height:100%;display:flex;flex-direction:column;">
          {#if !logsContainer}
            <div style="padding:8px;">
              <div style="font-size:9px;color:var(--color-text);opacity:0.5;margin-bottom:8px;letter-spacing:1px;">SELECT CONTAINER</div>
              {#each containers.filter(c => c.state === 'running') as c}
                <button
                  type="button"
                  style="display:block;width:100%;text-align:left;padding:6px 8px;margin-bottom:4px;font-size:10px;color:var(--color-text-bright);background:var(--color-bg-primary);border:1px solid var(--color-border);cursor:pointer;"
                  onclick={() => viewLogs(c.id, c.name)}
                >{c.name} ({c.image})</button>
              {/each}
              {#if containers.filter(c => c.state === 'running').length === 0}
                <div style="font-size:10px;color:var(--color-text);opacity:0.5;text-align:center;padding:16px;">No running containers</div>
              {/if}
            </div>
          {:else}
            <div style="display:flex;align-items:center;gap:8px;padding:6px 8px;border-bottom:1px solid var(--color-border);">
              <button
                type="button"
                style="padding:2px 8px;font-size:10px;color:var(--color-text);background:transparent;border:1px solid var(--color-border);cursor:pointer;"
                onclick={() => { logsContainer = ''; logsContent = ''; }}
              >← BACK</button>
              <span style="font-size:10px;color:var(--color-primary);letter-spacing:1px;">{logsContainer}</span>
            </div>
            <div style="flex:1;overflow-y:auto;padding:8px;background:var(--color-bg-primary);font-family:var(--font-mono);font-size:10px;line-height:1.5;color:var(--color-text);">
              {#if logsLoading}
                <span style="opacity:0.5;">Loading logs...</span>
              {:else}
                <pre style="white-space:pre-wrap;margin:0;">{logsContent || 'No logs available'}</pre>
              {/if}
            </div>
          {/if}
        </div>
      {/if}
    </div>
  {/if}
</div>
