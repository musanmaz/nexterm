<script lang="ts">
  import Modal from '$lib/components/shared/Modal.svelte';
  import type { Theme } from '$lib/types';

  let { open = $bindable(false), theme }: {
    open: boolean;
    theme: Theme | null;
  } = $props();
</script>

<Modal title="THEME EDITOR" bind:open>
  {#if theme}
    <div class="space-y-3">
      <h3 class="text-[11px] tracking-widest text-[var(--color-primary)]">{theme.name}</h3>
      <div class="grid grid-cols-2 gap-2">
        {#each Object.entries(theme.colors) as [key, value]}
          <div class="flex items-center gap-2">
            <div class="w-4 h-4 rounded border border-[var(--color-border)]" style="background: {value};"></div>
            <span class="text-[9px] text-[var(--color-text)]">{key}</span>
          </div>
        {/each}
      </div>
    </div>
  {:else}
    <div class="text-center text-[10px] text-[var(--color-text)] opacity-50 py-4">No theme loaded</div>
  {/if}
</Modal>
