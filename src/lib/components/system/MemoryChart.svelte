<script lang="ts">
  import { untrack } from 'svelte';
  import Chart from '$lib/components/shared/Chart.svelte';
  import type { MemoryInfo } from '$lib/types';

  let { info }: { info: MemoryInfo | null } = $props();

  let history = $state<number[]>([]);

  function formatBytes(bytes: number): string {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return (bytes / Math.pow(k, i)).toFixed(1) + ' ' + sizes[i];
  }

  $effect(() => {
    if (info) {
      const prev = untrack(() => history);
      history = [...prev.slice(-59), info.usage_percent];
    }
  });
</script>

<div>
  {#if info}
    <div class="flex items-center justify-between text-[10px] mb-1">
      <span class="text-[var(--color-text)]">RAM</span>
      <span class="glow-text font-bold">{info.usage_percent.toFixed(1)}%</span>
    </div>
    <Chart data={history} color="var(--color-chart-2)" height={35} />
    <div class="flex justify-between text-[9px] text-[var(--color-text)] mt-1">
      <span>{formatBytes(info.used)} / {formatBytes(info.total)}</span>
      <span>Free: {formatBytes(info.available)}</span>
    </div>
    {#if info.swap_total > 0}
      <div class="mt-1">
        <div class="flex items-center justify-between text-[9px] text-[var(--color-text)]">
          <span>SWAP</span>
          <span>{formatBytes(info.swap_used)} / {formatBytes(info.swap_total)}</span>
        </div>
        <div class="h-1 bg-[var(--color-bg-primary)] rounded overflow-hidden mt-0.5">
          <div
            class="h-full bg-[var(--color-accent)] transition-all duration-500"
            style="width: {info.swap_total > 0 ? (info.swap_used / info.swap_total) * 100 : 0}%;"
          ></div>
        </div>
      </div>
    {/if}
  {:else}
    <div class="text-[10px] text-[var(--color-text)] opacity-50 animate-pulse">Loading memory data...</div>
  {/if}
</div>
