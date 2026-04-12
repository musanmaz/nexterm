<script lang="ts">
  import type { KeyboardLayout, KeyDef } from '$lib/types';

  let { layout, onkeypress }: {
    layout: KeyboardLayout | null;
    onkeypress: (key: string) => void;
  } = $props();

  let shifted = $state(false);
  let ctrl = $state(false);
  let alt = $state(false);

  function handleKey(keyDef: KeyDef) {
    if (keyDef.action === 'shift') {
      shifted = !shifted;
      return;
    }
    if (keyDef.action === 'ctrl') {
      ctrl = !ctrl;
      return;
    }
    if (keyDef.action === 'alt') {
      alt = !alt;
      return;
    }

    let key = keyDef.key;
    if (shifted && key.length === 1) {
      key = key.toUpperCase();
    }
    onkeypress(key);

    if (shifted && !keyDef.action) shifted = false;
    if (ctrl) ctrl = false;
    if (alt) alt = false;
  }
</script>

<div class="w-full p-2 bg-[var(--color-bg-secondary)]">
  {#if layout}
    {#each layout.keys as row}
      <div class="flex gap-px mb-px justify-center">
        {#each row.row as keyDef}
          <button
            class="h-9 flex items-center justify-center text-[11px] font-mono border border-[var(--color-border)] cursor-pointer transition-all duration-100 active:scale-95"
            class:bg-[var(--color-primary)]={(keyDef.action === 'shift' && shifted) || (keyDef.action === 'ctrl' && ctrl) || (keyDef.action === 'alt' && alt)}
            class:text-[var(--color-bg-primary)]={(keyDef.action === 'shift' && shifted) || (keyDef.action === 'ctrl' && ctrl) || (keyDef.action === 'alt' && alt)}
            class:bg-[var(--color-bg-panel)]={!((keyDef.action === 'shift' && shifted) || (keyDef.action === 'ctrl' && ctrl) || (keyDef.action === 'alt' && alt))}
            class:text-[var(--color-text)]={!((keyDef.action === 'shift' && shifted) || (keyDef.action === 'ctrl' && ctrl) || (keyDef.action === 'alt' && alt))}
            class:hover:bg-[var(--color-border)]={true}
            style="flex: {keyDef.width || 1};"
            onclick={() => handleKey(keyDef)}
          >
            {#if shifted && keyDef.key.length === 1}
              {keyDef.key.toUpperCase()}
            {:else}
              {keyDef.label || keyDef.key}
            {/if}
          </button>
        {/each}
      </div>
    {/each}
  {:else}
    <div class="text-center text-[10px] text-[var(--color-text)] opacity-50 py-4">
      Loading keyboard layout...
    </div>
  {/if}
</div>
