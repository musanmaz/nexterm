<script lang="ts">
  import CpuChart from './CpuChart.svelte';
  import MemoryChart from './MemoryChart.svelte';
  import Panel from '$lib/components/shared/Panel.svelte';
  import type { CpuInfo, MemoryInfo, DiskInfo } from '$lib/types';

  let { cpu, memory, disks = [] }: {
    cpu: CpuInfo | null;
    memory: MemoryInfo | null;
    disks?: DiskInfo[];
  } = $props();
</script>

<div style="display:flex;flex-direction:column;gap:8px;">
  <Panel title="CPU">
    <CpuChart info={cpu} />
  </Panel>
  <Panel title="MEMORY">
    <MemoryChart info={memory} />
  </Panel>
</div>
