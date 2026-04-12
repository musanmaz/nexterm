<script lang="ts">
  import { onMount } from 'svelte';
  import type { FileEntry } from '$lib/types';
  import { listDirectory, getHomeDir } from '$lib/utils/ipc';

  let { onfileclick }: {
    onfileclick?: (path: string) => void;
  } = $props();

  let cwd = $state('~');
  let files = $state<FileEntry[]>([]);
  let loading = $state(false);
  let error = $state('');

  const fileIcons: Record<string, string> = {
    ts: '{}', js: '{}', json: '{}', py: '#', rs: '>>',
    md: 'M', html: '<>', css: '#', svelte: 'S', toml: 'T',
    yaml: 'Y', yml: 'Y', sh: '$', lock: 'L',
    jpg: 'I', jpeg: 'I', png: 'I', svg: 'I', gif: 'I',
    default: '-',
    folder: 'D',
  };

  function getIcon(name: string, isDir: boolean): string {
    if (isDir) return fileIcons.folder;
    const ext = name.split('.').pop()?.toLowerCase() || '';
    return fileIcons[ext] || fileIcons.default;
  }

  function formatSize(bytes: number): string {
    if (bytes < 1024) return bytes + ' B';
    if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(0) + ' K';
    return (bytes / (1024 * 1024)).toFixed(1) + ' M';
  }

  async function loadFiles(path: string) {
    loading = true;
    error = '';
    try {
      files = await listDirectory(path);
      cwd = path;
    } catch (e) {
      error = String(e);
      files = [];
    } finally {
      loading = false;
    }
  }

  function navigateTo(name: string, isDir: boolean) {
    if (isDir) {
      const newPath = cwd === '/' ? `/${name}` : `${cwd}/${name}`;
      loadFiles(newPath);
    } else {
      onfileclick?.(name);
    }
  }

  function goUp() {
    if (cwd === '/') return;
    const parent = cwd.substring(0, cwd.lastIndexOf('/')) || '/';
    loadFiles(parent);
  }

  onMount(async () => {
    try {
      const home = await getHomeDir();
      await loadFiles(home);
    } catch {
      await loadFiles('/');
    }
  });
</script>

<div class="h-full flex flex-col">
  <div class="flex items-center gap-1 px-2 py-1 border-b border-[var(--color-border)]">
    <button
      class="text-[10px] text-[var(--color-text)] hover:text-[var(--color-text-bright)] bg-transparent border-none cursor-pointer px-1"
      onclick={goUp}
      title="Go up"
    >..</button>
    <span class="text-[10px] text-[var(--color-primary)] truncate flex-1 tracking-wider">
      {cwd}
    </span>
    <button
      class="text-[10px] text-[var(--color-text)] hover:text-[var(--color-text-bright)] bg-transparent border-none cursor-pointer px-1"
      onclick={() => loadFiles(cwd)}
      title="Refresh"
    >↻</button>
  </div>
  <div class="flex-1 overflow-y-auto">
    {#if loading}
      <div class="text-center text-[10px] text-[var(--color-text)] opacity-50 py-4 animate-pulse">
        Scanning directory...
      </div>
    {:else if error}
      <div class="text-center text-[10px] text-[var(--color-error)] opacity-70 py-4">
        {error}
      </div>
    {:else if files.length === 0}
      <div class="text-center text-[10px] text-[var(--color-text)] opacity-50 py-4">
        Empty directory
      </div>
    {:else}
      {#each files as file}
        <button
          class="flex items-center gap-2 w-full px-2 py-0.5 text-[10px] border-none bg-transparent cursor-pointer hover:bg-[var(--color-border)] transition-colors text-left"
          onclick={() => navigateTo(file.name, file.is_dir)}
        >
          <span class="w-4 text-center opacity-70 font-mono text-[8px]">{getIcon(file.name, file.is_dir)}</span>
          <span class="flex-1 truncate" class:text-[var(--color-primary)]={file.is_dir} class:text-[var(--color-text)]={!file.is_dir}>
            {file.name}
          </span>
          <span class="text-[var(--color-text)] opacity-40 w-12 text-right">
            {file.is_dir ? '' : formatSize(file.size)}
          </span>
        </button>
      {/each}
    {/if}
  </div>
</div>
