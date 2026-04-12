<script lang="ts">
  import Modal from '$lib/components/shared/Modal.svelte';

  let { open = $bindable(false), onselect }: {
    open: boolean;
    onselect?: (path: string) => void;
  } = $props();

  let query = $state('');
  let results = $state<string[]>([]);
  let selectedIndex = $state(0);

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
    }
  }
</script>

<Modal title="FUZZY FINDER" bind:open>
  <div onkeydown={handleKeydown} role="search">
    <input
      bind:value={query}
      class="w-full bg-[var(--color-bg-primary)] border border-[var(--color-border)] text-[var(--color-text-bright)] text-sm px-3 py-2 font-mono outline-none focus:border-[var(--color-primary)]"
      placeholder="Search files..."
      autofocus
    />
    <div class="mt-2 max-h-[400px] overflow-y-auto">
      {#each results as result, i}
        <button
          class="w-full text-left px-3 py-1.5 text-xs font-mono border-none cursor-pointer transition-colors"
          class:bg-[var(--color-primary)]={i === selectedIndex}
          class:text-[var(--color-bg-primary)]={i === selectedIndex}
          class:bg-transparent={i !== selectedIndex}
          class:text-[var(--color-text)]={i !== selectedIndex}
          onclick={() => { onselect?.(result); open = false; }}
        >
          {result}
        </button>
      {/each}
      {#if query && results.length === 0}
        <div class="text-center text-[10px] text-[var(--color-text)] opacity-50 py-4">
          No matches found
        </div>
      {/if}
    </div>
  </div>
</Modal>
