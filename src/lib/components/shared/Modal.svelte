<script lang="ts">
  let { title = '', open = $bindable(false), children, onclose }: {
    title?: string;
    open: boolean;
    children: any;
    onclose?: () => void;
  } = $props();

  function close() {
    open = false;
    onclose?.();
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') close();
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
  <div style="position:fixed;inset:0;z-index:10000;display:flex;align-items:center;justify-content:center;">
    <div
      style="position:absolute;inset:0;background:rgba(0,0,0,0.7);backdrop-filter:blur(4px);"
      onclick={close}
      role="presentation"
    ></div>
    <div class="sci-fi-panel" style="position:relative;width:600px;max-height:80vh;z-index:10001;box-shadow:var(--glow-primary);">
      <div class="panel-title" style="display:flex;align-items:center;justify-content:space-between;">
        <span>{title}</span>
        <button
          type="button"
          style="color:var(--color-text);font-size:14px;cursor:pointer;background:transparent;border:none;"
          onclick={close}
        >✕</button>
      </div>
      <div style="padding:16px;overflow-y:auto;max-height:calc(80vh - 40px);">
        {@render children()}
      </div>
    </div>
  </div>
{/if}
