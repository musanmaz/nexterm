<script lang="ts">
  import type { ProcessInfo } from '$lib/types';

  let { processes = [] }: { processes: ProcessInfo[] } = $props();

  function formatMemory(bytes: number): string {
    if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(0) + 'K';
    if (bytes < 1024 * 1024 * 1024) return (bytes / (1024 * 1024)).toFixed(1) + 'M';
    return (bytes / (1024 * 1024 * 1024)).toFixed(1) + 'G';
  }
</script>

<div class="h-full overflow-hidden">
  <div class="grid grid-cols-[1fr_50px_60px] gap-1 text-[9px] text-[var(--color-text)] opacity-60 px-1 pb-1 border-b border-[var(--color-border)]">
    <span>PROCESS</span>
    <span class="text-right">CPU%</span>
    <span class="text-right">MEM</span>
  </div>
  <div class="overflow-y-auto" style="max-height: calc(100% - 20px);">
    {#each processes.slice(0, 20) as proc}
      <div class="grid grid-cols-[1fr_50px_60px] gap-1 text-[10px] px-1 py-px hover:bg-[var(--color-border)] transition-colors">
        <span class="text-[var(--color-text)] truncate">{proc.name}</span>
        <span class="text-right" class:text-[var(--color-error)]={proc.cpu_usage > 80} class:text-[var(--color-warning)]={proc.cpu_usage > 30 && proc.cpu_usage <= 80} class:text-[var(--color-text-bright)]={proc.cpu_usage <= 30}>
          {proc.cpu_usage.toFixed(1)}
        </span>
        <span class="text-right text-[var(--color-text)]">{formatMemory(proc.memory)}</span>
      </div>
    {/each}
  </div>
</div>
