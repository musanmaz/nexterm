<script lang="ts">
  import Clock from './Clock.svelte';
  import Panel from '$lib/components/shared/Panel.svelte';
  import { getCurrentVersion } from '$lib/utils/updater';

  let {
    hostname = '',
    osInfo = '',
    uptime = 0,
    activePanel = 'system',
    rightCollapsed = false,
    onpanelchange,
    onsettings,
    ontoggleright,
  }: {
    hostname?: string;
    osInfo?: string;
    uptime?: number;
    activePanel?: string;
    rightCollapsed?: boolean;
    onpanelchange: (panel: string) => void;
    onsettings: () => void;
    ontoggleright: () => void;
  } = $props();

  function formatUptime(seconds: number): string {
    const d = Math.floor(seconds / 86400);
    const h = Math.floor((seconds % 86400) / 3600);
    const m = Math.floor((seconds % 3600) / 60);
    if (d > 0) return `${d}d ${h}h`;
    if (h > 0) return `${h}h ${m}m`;
    return `${m}m`;
  }

  const panels = [
    { id: 'system', icon: '◈', label: 'SYSTEM' },
    { id: 'docker', icon: '⬡', label: 'DOCKER' },
    { id: 'kubernetes', icon: '☸', label: 'K8S' },
    { id: 'git', icon: '⎇', label: 'GIT' },
    { id: 'ssh', icon: '⇋', label: 'SSH' },
    { id: 'plugins', icon: '⚡', label: 'PLUGINS' },
  ];
</script>

<div style="display:flex;flex-direction:column;height:100%;width:100%;gap:8px;">
  <Clock />

  <Panel title="HOST">
    <div style="font-size:10px;display:flex;flex-direction:column;gap:4px;">
      <div style="display:flex;justify-content:space-between;">
        <span style="color:var(--color-text);opacity:0.6;">HOST</span>
        <span class="glow-text">{hostname || '...'}</span>
      </div>
      <div style="display:flex;justify-content:space-between;">
        <span style="color:var(--color-text);opacity:0.6;">OS</span>
        <span style="color:var(--color-text);">{osInfo || '...'}</span>
      </div>
      <div style="display:flex;justify-content:space-between;">
        <span style="color:var(--color-text);opacity:0.6;">UPTIME</span>
        <span style="color:var(--color-text);">{formatUptime(uptime)}</span>
      </div>
    </div>
  </Panel>

  <div style="display:flex;flex-direction:column;gap:1px;margin-top:8px;">
    {#each panels as panel}
      <button
        type="button"
        style="
          display:flex;align-items:center;gap:8px;padding:8px 12px;
          font-size:10px;letter-spacing:2px;border:none;cursor:pointer;
          text-align:left;width:100%;transition:all 0.2s;
          background:{activePanel === panel.id ? 'var(--color-primary)' : 'transparent'};
          color:{activePanel === panel.id ? 'var(--color-bg-primary)' : 'var(--color-text)'};
        "
        onclick={() => onpanelchange(panel.id)}
      >
        <span style="font-size:14px;">{panel.icon}</span>
        <span>{panel.label}</span>
      </button>
    {/each}
  </div>

  <div style="flex:1;"></div>

  <button
    type="button"
    style="
      display:flex;align-items:center;gap:8px;padding:8px 12px;
      font-size:10px;letter-spacing:2px;border:none;cursor:pointer;
      text-align:left;width:100%;background:transparent;color:var(--color-text);
    "
    onclick={ontoggleright}
  >
    <span style="font-size:14px;">{rightCollapsed ? '»' : '«'}</span>
    <span>{rightCollapsed ? 'SHOW PANEL' : 'HIDE PANEL'}</span>
  </button>

  <button
    type="button"
    style="
      display:flex;align-items:center;gap:8px;padding:8px 12px;
      font-size:10px;letter-spacing:2px;border:none;cursor:pointer;
      text-align:left;width:100%;background:transparent;color:var(--color-text);
    "
    onclick={onsettings}
  >
    <span style="font-size:14px;">⚙</span>
    <span>SETTINGS</span>
  </button>

  <div style="text-align:center;font-size:8px;color:var(--color-text);opacity:0.3;padding-bottom:8px;letter-spacing:2px;">
    NEXTERM v{getCurrentVersion()}
  </div>
</div>
