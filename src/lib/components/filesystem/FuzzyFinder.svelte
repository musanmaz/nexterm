<script lang="ts">
  import { listDirectory } from '$lib/utils/ipc';

  let { open = $bindable(false), basePath = '', onselect }: {
    open: boolean;
    basePath?: string;
    onselect?: (path: string) => void;
  } = $props();

  let query = $state('');
  let results = $state<string[]>([]);
  let allFiles = $state<string[]>([]);
  let selectedIndex = $state(0);
  let loading = $state(false);
  let inputEl: HTMLInputElement;

  $effect(() => {
    if (open && basePath) {
      loading = true;
      scanDirectory(basePath, 3).then(files => {
        allFiles = files;
        loading = false;
      });
      setTimeout(() => inputEl?.focus(), 50);
    } else {
      query = '';
      results = [];
      allFiles = [];
      selectedIndex = 0;
    }
  });

  $effect(() => {
    if (!query.trim()) {
      results = allFiles.slice(0, 50);
    } else {
      const q = query.toLowerCase();
      results = allFiles.filter(f => {
        const name = f.split('/').pop()?.toLowerCase() || '';
        return name.includes(q) || f.toLowerCase().includes(q);
      }).slice(0, 50);
    }
    selectedIndex = 0;
  });

  async function scanDirectory(path: string, depth: number): Promise<string[]> {
    if (depth <= 0) return [];
    try {
      const entries = await listDirectory(path);
      const files: string[] = [];
      for (const entry of entries) {
        const fullPath = path.endsWith('/') ? path + entry.name : path + '/' + entry.name;
        if (entry.name.startsWith('.') || entry.name === 'node_modules' || entry.name === 'target' || entry.name === '.git') continue;
        if (entry.is_dir) {
          const sub = await scanDirectory(fullPath, depth - 1);
          files.push(...sub);
        } else {
          files.push(fullPath);
        }
      }
      return files;
    } catch {
      return [];
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'ArrowDown') {
      e.preventDefault();
      selectedIndex = Math.min(selectedIndex + 1, results.length - 1);
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      selectedIndex = Math.max(selectedIndex - 1, 0);
    } else if (e.key === 'Enter' && results[selectedIndex]) {
      onselect?.(results[selectedIndex]);
      open = false;
    } else if (e.key === 'Escape') {
      open = false;
    }
  }
</script>

{#if open}
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    style="position:fixed;inset:0;background:rgba(0,0,0,0.7);z-index:9999;display:flex;align-items:flex-start;justify-content:center;padding-top:80px;"
    onclick={() => open = false}
    onkeydown={handleKeydown}
  >
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      style="width:500px;max-height:400px;background:var(--color-bg-panel);border:1px solid var(--color-border);display:flex;flex-direction:column;"
      onclick={(e) => e.stopPropagation()}
    >
      <div style="padding:8px;border-bottom:1px solid var(--color-border);">
        <input
          bind:this={inputEl}
          bind:value={query}
          placeholder="Search files... (type to filter)"
          onkeydown={handleKeydown}
          style="width:100%;background:var(--color-bg-primary);border:1px solid var(--color-border);color:var(--color-text-bright);font-size:12px;padding:6px 10px;font-family:var(--font-mono);outline:none;"
        />
      </div>
      <div style="flex:1;overflow-y:auto;max-height:320px;">
        {#if loading}
          <div style="text-align:center;font-size:10px;color:var(--color-text);opacity:0.5;padding:16px;">Scanning files...</div>
        {:else}
          {#each results as result, i}
            <button
              type="button"
              style="
                display:block;width:100%;text-align:left;padding:4px 10px;font-size:11px;font-family:var(--font-mono);
                border:none;cursor:pointer;transition:background 0.1s;
                background:{i === selectedIndex ? 'var(--color-primary)' : 'transparent'};
                color:{i === selectedIndex ? 'var(--color-bg-primary)' : 'var(--color-text)'};
              "
              onclick={() => { onselect?.(result); open = false; }}
              onmouseenter={() => selectedIndex = i}
            >
              {result.replace(basePath + '/', '')}
            </button>
          {/each}
          {#if query && results.length === 0}
            <div style="text-align:center;font-size:10px;color:var(--color-text);opacity:0.5;padding:16px;">No matches found</div>
          {/if}
        {/if}
      </div>
    </div>
  </div>
{/if}
