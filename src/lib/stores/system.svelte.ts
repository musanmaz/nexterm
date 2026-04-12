import type { CpuInfo, MemoryInfo, NetworkInfo, ProcessInfo, DiskInfo } from '$lib/types';
import { getCpuInfo, getMemoryInfo, getNetworkInfo, getProcesses, getDiskInfo } from '$lib/utils/ipc';

let cpu = $state<CpuInfo | null>(null);
let memory = $state<MemoryInfo | null>(null);
let network = $state<NetworkInfo | null>(null);
let processes = $state<ProcessInfo[]>([]);
let disks = $state<DiskInfo[]>([]);
let hostname = $state('');
let osInfo = $state('');
let uptime = $state(0);

let intervalId: ReturnType<typeof setInterval> | null = null;

export function getSystemStore() {
  async function refresh() {
    try {
      const [cpuData, memData, netData, procData, diskData] = await Promise.all([
        getCpuInfo(),
        getMemoryInfo(),
        getNetworkInfo(),
        getProcesses(),
        getDiskInfo(),
      ]);
      cpu = cpuData;
      memory = memData;
      network = netData;
      processes = procData;
      disks = diskData;
    } catch (e) {
      console.error('System refresh error:', e);
    }
  }

  function startPolling(intervalMs = 2000) {
    if (intervalId) return;
    refresh();
    intervalId = setInterval(refresh, intervalMs);
  }

  function stopPolling() {
    if (intervalId) {
      clearInterval(intervalId);
      intervalId = null;
    }
  }

  return {
    get cpu() { return cpu; },
    get memory() { return memory; },
    get network() { return network; },
    get processes() { return processes; },
    get disks() { return disks; },
    get hostname() { return hostname; },
    get osInfo() { return osInfo; },
    get uptime() { return uptime; },
    refresh,
    startPolling,
    stopPolling,
    setHostname: (h: string) => { hostname = h; },
    setOsInfo: (o: string) => { osInfo = o; },
    setUptime: (u: number) => { uptime = u; },
  };
}
