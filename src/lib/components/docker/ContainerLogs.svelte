<script lang="ts">
  let { logs = '', containerName = '' }: {
    logs?: string;
    containerName?: string;
  } = $props();
</script>

<div class="h-full flex flex-col">
  <div class="panel-title text-[10px]">LOGS: {containerName}</div>
  <div class="flex-1 overflow-y-auto p-2 bg-[var(--color-bg-primary)] font-mono text-[10px] text-[var(--color-text)] leading-4">
    <pre class="whitespace-pre-wrap">{logs || 'No logs available'}</pre>
  </div>
</div>
