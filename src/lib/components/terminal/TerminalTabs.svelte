<script lang="ts">
  import type { TerminalTab } from '$lib/types';

  let { tabs = [], activeTabId = '', onselect, onclose, onadd, onrename }: {
    tabs: TerminalTab[];
    activeTabId: string;
    onselect: (id: string) => void;
    onclose: (id: string) => void;
    onadd: () => void;
    onrename?: (id: string, title: string) => void;
  } = $props();

  let editingId = $state<string | null>(null);
  let editValue = $state('');

  function startEdit(tab: TerminalTab) {
    editingId = tab.id;
    editValue = tab.title;
  }

  function finishEdit() {
    if (editingId && editValue.trim()) {
      onrename?.(editingId, editValue.trim());
    }
    editingId = null;
  }

  function handleEditKey(e: KeyboardEvent) {
    if (e.key === 'Enter') finishEdit();
    if (e.key === 'Escape') { editingId = null; }
  }
</script>

<div style="display:flex;align-items:center;height:32px;background:var(--color-bg-secondary);border-bottom:1px solid var(--color-border);">
  {#each tabs as tab}
    <button
      type="button"
      style="
        display:flex;align-items:center;gap:8px;padding:0 16px;height:100%;
        font-size:12px;letter-spacing:1px;border:none;border-right:1px solid var(--color-border);
        cursor:pointer;transition:all 0.2s;
        background:{tab.id === activeTabId ? 'var(--color-bg-panel)' : 'transparent'};
        color:{tab.id === activeTabId ? 'var(--color-text-bright)' : 'var(--color-text)'};
        {tab.id === activeTabId ? 'border-top:2px solid var(--color-primary);' : ''}
      "
      onclick={() => onselect(tab.id)}
      ondblclick={(e) => { e.preventDefault(); startEdit(tab); }}
    >
      <span style="font-size:10px;opacity:0.5;">⬡</span>
      {#if editingId === tab.id}
        <!-- svelte-ignore a11y_autofocus -->
        <input
          type="text"
          bind:value={editValue}
          onblur={finishEdit}
          onkeydown={handleEditKey}
          autofocus
          style="
            width:80px;background:var(--color-bg-primary);border:1px solid var(--color-primary);
            color:var(--color-text-bright);font-size:11px;padding:1px 4px;outline:none;
            font-family:inherit;letter-spacing:1px;
          "
          onclick={(e) => e.stopPropagation()}
        />
      {:else}
        <span>{tab.title}</span>
      {/if}
      {#if tabs.length > 1}
        <span
          style="margin-left:8px;opacity:0.4;font-size:10px;cursor:pointer;"
          onclick={(e) => { e.stopPropagation(); onclose(tab.id); }}
          onkeydown={(e) => { if (e.key === 'Enter' || e.key === ' ') { e.stopPropagation(); onclose(tab.id); } }}
          role="button"
          tabindex="-1"
        >✕</span>
      {/if}
    </button>
  {/each}

  {#if tabs.length < 5}
    <button
      type="button"
      style="padding:0 12px;height:100%;color:var(--color-text);font-size:14px;cursor:pointer;background:transparent;border:none;"
      onclick={onadd}
      title="New Tab (Ctrl+Shift+T)"
    >+</button>
  {/if}

  <div style="flex:1;"></div>
</div>
