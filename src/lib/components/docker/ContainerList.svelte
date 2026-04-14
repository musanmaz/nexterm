<script lang="ts">
  import type { ContainerInfo } from '$lib/types';

  let { containers = [], onaction, onexec, onlogs }: {
    containers?: ContainerInfo[];
    onaction: (id: string, action: string) => void;
    onexec?: (name: string, containerId: string) => void;
    onlogs?: (containerId: string, name: string) => void;
  } = $props();

  let actionLoading = $state<string | null>(null);

  function stateColor(state: string): string {
    if (state === 'running') return 'var(--color-success)';
    if (state === 'exited') return 'var(--color-error)';
    return 'var(--color-warning)';
  }

  async function doAction(id: string, action: string) {
    actionLoading = id + action;
    try {
      onaction(id, action);
    } finally {
      setTimeout(() => { actionLoading = null; }, 2000);
    }
  }
</script>

<div style="padding:4px;">
  {#each containers as container}
    <div style="display:flex;align-items:center;gap:6px;padding:6px;border-bottom:1px solid var(--color-border);">
      <!-- Status dot -->
      <div style="width:8px;height:8px;border-radius:50%;flex-shrink:0;background:{stateColor(container.state)};"></div>

      <!-- Info -->
      <div style="flex:1;min-width:0;overflow:hidden;">
        <div style="font-size:11px;color:var(--color-text-bright);overflow:hidden;text-overflow:ellipsis;white-space:nowrap;">{container.name}</div>
        <div style="font-size:9px;color:var(--color-text);opacity:0.5;overflow:hidden;text-overflow:ellipsis;white-space:nowrap;">{container.image}</div>
      </div>

      <!-- Actions -->
      <div style="display:flex;gap:3px;flex-shrink:0;">
        {#if container.state !== 'running'}
          <button
            type="button"
            style="padding:3px 8px;font-size:9px;background:var(--color-success);color:#000;border:none;cursor:pointer;letter-spacing:1px;"
            onclick={() => doAction(container.id, 'start')}
            title="Start"
          >▶</button>
          <button
            type="button"
            style="padding:3px 8px;font-size:9px;background:var(--color-error);color:#fff;border:none;cursor:pointer;letter-spacing:1px;"
            onclick={() => doAction(container.id, 'remove')}
            title="Remove"
          >✕</button>
        {:else}
          <button
            type="button"
            style="padding:3px 8px;font-size:9px;background:var(--color-accent);color:#fff;border:none;cursor:pointer;letter-spacing:1px;"
            onclick={() => onexec?.(container.name, container.id)}
            title="Open shell in container"
          >SH</button>
          <button
            type="button"
            style="padding:3px 8px;font-size:9px;background:var(--color-secondary);color:#fff;border:none;cursor:pointer;letter-spacing:1px;"
            onclick={() => onlogs?.(container.id, container.name)}
            title="View logs"
          >LOG</button>
          <button
            type="button"
            style="padding:3px 8px;font-size:9px;background:var(--color-warning);color:#000;border:none;cursor:pointer;letter-spacing:1px;"
            onclick={() => doAction(container.id, 'stop')}
            title="Stop"
          >■</button>
          <button
            type="button"
            style="padding:3px 8px;font-size:9px;background:var(--color-primary);color:#000;border:none;cursor:pointer;letter-spacing:1px;"
            onclick={() => doAction(container.id, 'restart')}
            title="Restart"
          >↻</button>
        {/if}
      </div>
    </div>
  {/each}
  {#if containers.length === 0}
    <div style="text-align:center;font-size:10px;color:var(--color-text);opacity:0.5;padding:16px;">No containers</div>
  {/if}
</div>
