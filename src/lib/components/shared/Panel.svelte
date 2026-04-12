<script lang="ts">
  let { title = '', children, collapsible = false, class: className = '' }: {
    title?: string;
    children: any;
    collapsible?: boolean;
    class?: string;
  } = $props();

  let collapsed = $state(false);
</script>

<div class="sci-fi-panel {className}" class:collapsed>
  {#if title}
    <div class="panel-title flex items-center justify-between">
      <span>{title}</span>
      {#if collapsible}
        <button
          class="text-[var(--color-text)] hover:text-[var(--color-text-bright)] text-xs cursor-pointer bg-transparent border-none"
          onclick={() => collapsed = !collapsed}
        >
          {collapsed ? '▶' : '▼'}
        </button>
      {/if}
    </div>
  {/if}
  {#if !collapsed}
    <div class="panel-content p-2">
      {@render children()}
    </div>
  {/if}
</div>

<style>
  .collapsed {
    max-height: 28px;
    overflow: hidden;
  }
</style>
