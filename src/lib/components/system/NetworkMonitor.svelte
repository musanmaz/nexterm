<script lang="ts">
  import { untrack } from 'svelte';
  import Chart from '$lib/components/shared/Chart.svelte';
  import type { NetworkInfo } from '$lib/types';

  let { network }: { network: NetworkInfo | null } = $props();

  let rxHistory = $state<number[]>([]);
  let txHistory = $state<number[]>([]);
  let lastRx = $state(0);
  let lastTx = $state(0);

  function formatBytes(bytes: number): string {
    if (bytes < 1024) return bytes + ' B';
    if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
    if (bytes < 1024 * 1024 * 1024) return (bytes / (1024 * 1024)).toFixed(1) + ' MB';
    return (bytes / (1024 * 1024 * 1024)).toFixed(2) + ' GB';
  }

  $effect(() => {
    if (network) {
      const prevRx = untrack(() => lastRx);
      const prevTx = untrack(() => lastTx);
      const rxRate = prevRx > 0 ? network.total_received - prevRx : 0;
      const txRate = prevTx > 0 ? network.total_transmitted - prevTx : 0;
      lastRx = network.total_received;
      lastTx = network.total_transmitted;
      const prevRxH = untrack(() => rxHistory);
      const prevTxH = untrack(() => txHistory);
      rxHistory = [...prevRxH.slice(-59), Math.max(0, rxRate)];
      txHistory = [...prevTxH.slice(-59), Math.max(0, txRate)];
    }
  });
</script>

<div>
  {#if network}
    <div class="space-y-2">
      <div>
        <div class="flex justify-between text-[10px] mb-0.5">
          <span class="text-[var(--color-success)]">▼ RX</span>
          <span class="text-[var(--color-text)]">{formatBytes(network.total_received)}</span>
        </div>
        <Chart data={rxHistory} color="#00ff9f" height={30} />
      </div>
      <div>
        <div class="flex justify-between text-[10px] mb-0.5">
          <span class="text-[var(--color-accent)]">▲ TX</span>
          <span class="text-[var(--color-text)]">{formatBytes(network.total_transmitted)}</span>
        </div>
        <Chart data={txHistory} color="#ea00d9" height={30} />
      </div>
      <div class="border-t border-[var(--color-border)] pt-1">
        <div class="text-[9px] text-[var(--color-text)] opacity-60 mb-1">INTERFACES</div>
        {#each network.interfaces.filter(i => i.received > 0 || i.transmitted > 0).slice(0, 4) as iface}
          <div class="flex justify-between text-[9px] text-[var(--color-text)]">
            <span class="truncate max-w-[80px]">{iface.name}</span>
            <span>{formatBytes(iface.received)} / {formatBytes(iface.transmitted)}</span>
          </div>
        {/each}
      </div>
    </div>
  {:else}
    <div class="text-[10px] text-[var(--color-text)] opacity-50 animate-pulse">Loading network data...</div>
  {/if}
</div>
