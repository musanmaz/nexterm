<script lang="ts">
  import { untrack } from 'svelte';
  import Chart from '$lib/components/shared/Chart.svelte';
  import type { CpuInfo } from '$lib/types';

  let { info }: { info: CpuInfo | null } = $props();

  let history = $state<number[]>([]);

  $effect(() => {
    if (info) {
      const prev = untrack(() => history);
      history = [...prev.slice(-59), info.global_usage];
    }
  });
</script>

<div>
  {#if info}
    <div class="flex items-center justify-between text-[10px] mb-1">
      <span class="text-[var(--color-text)] truncate">{info.brand}</span>
      <span class="glow-text font-bold">{info.global_usage.toFixed(1)}%</span>
    </div>
    <Chart data={history} height={40} label="USAGE" />
    <div class="grid gap-px mt-1" style="grid-template-columns: repeat({Math.min(info.cores, 16)}, 1fr);">
      {#each info.usage as coreUsage, i}
        <div class="h-2 bg-[var(--color-bg-primary)] rounded-sm overflow-hidden" title="Core {i}: {coreUsage.toFixed(1)}%">
          <div
            class="h-full bg-[var(--color-primary)] transition-all duration-300"
            style="width: {coreUsage}%; opacity: {0.4 + (coreUsage / 100) * 0.6};"
          ></div>
        </div>
      {/each}
    </div>
    <div class="text-[9px] text-[var(--color-text)] opacity-50 mt-1">{info.cores} cores @ {info.frequency} MHz</div>
  {:else}
    <div class="text-[10px] text-[var(--color-text)] opacity-50 animate-pulse">Loading CPU data...</div>
  {/if}
</div>
