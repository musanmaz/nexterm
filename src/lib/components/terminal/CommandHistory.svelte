<script lang="ts">
  import { untrack } from 'svelte';

  let { oncommand }: {
    oncommand?: (cmd: string) => void;
  } = $props();

  let history = $state<string[]>([]);
  let filter = $state('');
  let activeTab = $state<'history' | 'quick' | 'snippets'>('quick');

  const quickActions = [
    { label: 'CLEAR', cmd: 'clear', icon: '⌫', color: 'var(--color-text)' },
    { label: 'TOP', cmd: 'top', icon: '◈', color: 'var(--color-chart-1)' },
    { label: 'HTOP', cmd: 'htop', icon: '◈', color: 'var(--color-chart-2)' },
    { label: 'LS -LA', cmd: 'ls -la', icon: 'D', color: 'var(--color-success)' },
    { label: 'DISK', cmd: 'df -h', icon: '⬡', color: 'var(--color-warning)' },
    { label: 'PORTS', cmd: 'lsof -i -P | head -20', icon: '⇋', color: 'var(--color-accent)' },
    { label: 'GIT LOG', cmd: 'git log --oneline -10', icon: '⎇', color: 'var(--color-chart-3)' },
    { label: 'GIT STATUS', cmd: 'git status -s', icon: '⎇', color: 'var(--color-success)' },
    { label: 'DOCKER PS', cmd: 'docker ps --format "table {{.Names}}\t{{.Status}}"', icon: '⬡', color: 'var(--color-chart-1)' },
    { label: 'MEM', cmd: 'vm_stat | head -10', icon: '▣', color: 'var(--color-chart-4)' },
    { label: 'TREE', cmd: 'find . -maxdepth 2 -type f | head -30', icon: 'D', color: 'var(--color-text)' },
    { label: 'WHOAMI', cmd: 'whoami && hostname', icon: '◉', color: 'var(--color-primary)' },
  ];

  let defaultSnippets = $state([
    { name: 'Git commit all', cmd: 'git add -A && git commit -m ""' },
    { name: 'Find large files', cmd: 'find . -size +100M -type f' },
    { name: 'Kill port', cmd: 'lsof -ti:3000 | xargs kill -9' },
    { name: 'NPM clean install', cmd: 'rm -rf node_modules && npm install' },
    { name: 'Docker prune', cmd: 'docker system prune -af' },
    { name: 'Tar compress', cmd: 'tar -czf archive.tar.gz .' },
  ]);

  export function addToHistory(cmd: string) {
    if (!cmd.trim()) return;
    const prev = untrack(() => history);
    const filtered = prev.filter(h => h !== cmd);
    history = [cmd, ...filtered].slice(0, 50);
  }

  let filteredHistory = $derived(
    filter
      ? history.filter(h => h.toLowerCase().includes(filter.toLowerCase()))
      : history
  );

  function run(cmd: string) {
    oncommand?.(cmd);
  }
</script>

<div style="height:100%;display:flex;flex-direction:column;background:var(--color-bg-secondary);">
  <!-- Tabs -->
  <div style="display:flex;align-items:center;border-bottom:1px solid var(--color-border);padding:0 8px;height:28px;gap:4px;">
    {#each [['quick', '⚡ QUICK'], ['history', '↻ HISTORY'], ['snippets', '✂ SNIPPETS']] as [id, label]}
      <button
        type="button"
        style="
          padding:2px 10px;font-size:9px;letter-spacing:1px;cursor:pointer;border:none;
          transition:all 0.15s;
          background:{activeTab === id ? 'var(--color-primary)' : 'transparent'};
          color:{activeTab === id ? 'var(--color-bg-primary)' : 'var(--color-text)'};
        "
        onclick={() => activeTab = id as typeof activeTab}
      >{label}</button>
    {/each}
  </div>

  <!-- Content -->
  <div style="flex:1;overflow:hidden;padding:6px;">
    {#if activeTab === 'quick'}
      <div style="display:flex;flex-wrap:wrap;gap:4px;overflow-y:auto;height:100%;">
        {#each quickActions as action}
          <button
            type="button"
            style="
              display:flex;align-items:center;gap:6px;padding:4px 10px;
              font-size:9px;letter-spacing:1px;cursor:pointer;
              background:var(--color-bg-panel);border:1px solid var(--color-border);
              color:var(--color-text);transition:all 0.15s;
              white-space:nowrap;
            "
            onclick={() => run(action.cmd)}
            title={action.cmd}
          >
            <span style="color:{action.color};font-size:11px;">{action.icon}</span>
            <span>{action.label}</span>
          </button>
        {/each}
      </div>

    {:else if activeTab === 'history'}
      <div style="display:flex;flex-direction:column;gap:4px;height:100%;">
        <input
          type="text"
          bind:value={filter}
          placeholder="Filter commands..."
          style="
            width:100%;background:var(--color-bg-primary);border:1px solid var(--color-border);
            color:var(--color-text);font-size:10px;padding:3px 8px;font-family:var(--font-mono);outline:none;
          "
        />
        <div style="flex:1;overflow-y:auto;">
          {#if filteredHistory.length === 0}
            <div style="text-align:center;font-size:10px;color:var(--color-text);opacity:0.4;padding:16px;">
              {history.length === 0 ? 'Commands will appear here as you type' : 'No matching commands'}
            </div>
          {:else}
            {#each filteredHistory as cmd, i}
              <button
                type="button"
                style="
                  display:flex;align-items:center;gap:8px;width:100%;text-align:left;
                  padding:3px 8px;font-size:10px;font-family:var(--font-mono);
                  cursor:pointer;border:none;
                  background:{i % 2 === 0 ? 'transparent' : 'rgba(0,212,255,0.03)'};
                  color:var(--color-text);transition:background 0.1s;
                "
                onclick={() => run(cmd)}
                title="Click to run"
              >
                <span style="color:var(--color-primary);opacity:0.4;font-size:9px;">$</span>
                <span style="overflow:hidden;text-overflow:ellipsis;white-space:nowrap;">{cmd}</span>
              </button>
            {/each}
          {/if}
        </div>
      </div>

    {:else if activeTab === 'snippets'}
      <div style="display:flex;flex-direction:column;gap:4px;overflow-y:auto;height:100%;">
        {#each defaultSnippets as snippet}
          <button
            type="button"
            style="
              display:flex;align-items:center;justify-content:space-between;
              padding:4px 8px;cursor:pointer;border:none;
              background:var(--color-bg-panel);border:1px solid var(--color-border);
              color:var(--color-text);transition:all 0.15s;width:100%;text-align:left;
            "
            onclick={() => run(snippet.cmd)}
            title={snippet.cmd}
          >
            <span style="font-size:10px;color:var(--color-text-bright);">{snippet.name}</span>
            <span style="font-size:9px;color:var(--color-text);opacity:0.4;font-family:var(--font-mono);overflow:hidden;text-overflow:ellipsis;white-space:nowrap;max-width:60%;">{snippet.cmd}</span>
          </button>
        {/each}
      </div>
    {/if}
  </div>
</div>
