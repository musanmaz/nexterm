<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import Sidebar from '$lib/components/sidebar/Sidebar.svelte';
  import Globe from '$lib/components/sidebar/Globe.svelte';
  import SystemMonitor from '$lib/components/system/SystemMonitor.svelte';
  import ProcessList from '$lib/components/system/ProcessList.svelte';
  import NetworkMonitor from '$lib/components/system/NetworkMonitor.svelte';
  import Terminal from '$lib/components/terminal/Terminal.svelte';
  import TerminalTabs from '$lib/components/terminal/TerminalTabs.svelte';
  import AIAssistant from '$lib/components/terminal/AIAssistant.svelte';
  import FileExplorer from '$lib/components/filesystem/FileExplorer.svelte';
  import CommandHistory from '$lib/components/terminal/CommandHistory.svelte';
  import DockerPanel from '$lib/components/docker/DockerPanel.svelte';
  import GitPanel from '$lib/components/git/GitPanel.svelte';
  import SSHManager from '$lib/components/ssh/SSHManager.svelte';
  import PluginManager from '$lib/components/plugins/PluginManager.svelte';
  import KubernetesPanel from '$lib/components/kubernetes/KubernetesPanel.svelte';
  import SettingsModal from '$lib/components/settings/SettingsModal.svelte';
  import UpdateBanner from '$lib/components/shared/UpdateBanner.svelte';
  import Panel from '$lib/components/shared/Panel.svelte';
  import FuzzyFinder from '$lib/components/filesystem/FuzzyFinder.svelte';
  import ThemeEditor from '$lib/components/settings/ThemeEditor.svelte';

  import { getThemeStore } from '$lib/stores/theme.svelte';
  import { getSettingsStore } from '$lib/stores/settings.svelte';
  import { getTerminalStore } from '$lib/stores/terminal.svelte';
  import { getSystemStore } from '$lib/stores/system.svelte';
  import { getDockerStore } from '$lib/stores/docker.svelte';
  import { getGitStore } from '$lib/stores/git.svelte';
  import { getSSHStore } from '$lib/stores/ssh.svelte';
  import { getKubernetesStore } from '$lib/stores/kubernetes.svelte';
  import { keybindings } from '$lib/utils/keybindings';
  import { audio } from '$lib/utils/audio';
  import { getHostname, getOsInfo, getUptime, getHomeDir, ptyWrite } from '$lib/utils/ipc';

  import type { TerminalTab } from '$lib/types';

  const themeStore = getThemeStore();
  const settingsStore = getSettingsStore();
  const terminalStore = getTerminalStore();
  const systemStore = getSystemStore();
  const dockerStore = getDockerStore();
  const gitStore = getGitStore();
  const sshStore = getSSHStore();
  const k8sStore = getKubernetesStore();

  let activePanel = $state('system');
  let settingsOpen = $state(false);
  let rightCollapsed = $state(false);
  let tabSessions = $state<Record<string, string>>({});
  let pendingCommands = $state<Record<string, string>>({});
  let commandFeed = $state<string[]>([]);
  let initError = $state('');
  let cmdHistoryRef: CommandHistory;
  let fuzzyOpen = $state(false);
  let themeEditorOpen = $state(false);

  let activeSessionId = $derived(tabSessions[terminalStore.activeTabId] || '');
  let uptimeInterval: ReturnType<typeof setInterval> | null = null;

  onMount(() => {
    settingsStore.load();
    initApp().catch(e => {
      initError = String(e);
      if ((window as any).__dbgLog) (window as any).__dbgLog('initApp FAILED: ' + e, '#f00');
    });
  });

  onDestroy(() => {
    systemStore.stopPolling();
    if (uptimeInterval) clearInterval(uptimeInterval);
  });

  async function initApp() {
    try {
      await themeStore.setTheme(settingsStore.current.theme);
    } catch (e) {
      console.error('Theme load error:', e);
    }

    const tabId = crypto.randomUUID?.() || Math.random().toString(36).slice(2);
    const tab: TerminalTab = {
      id: tabId,
      title: 'Terminal 1',
      sessionId: null,
      isActive: true,
    };
    terminalStore.addTab(tab);

    systemStore.startPolling(2000);

    try { systemStore.setHostname(await getHostname()); } catch {}
    try { systemStore.setOsInfo(await getOsInfo()); } catch {}
    try { systemStore.setUptime(await getUptime()); } catch {}

    try {
      await dockerStore.checkAvailability();
      if (dockerStore.available) {
        dockerStore.refreshContainers();
        dockerStore.refreshImages();
      }
    } catch {}

    try {
      const home = await getHomeDir();
      gitStore.setPath(home);
    } catch {}

    try { sshStore.load(); } catch {}

    k8sStore.init().catch(() => {});

    try {
      audio.init();
      audio.setEnabled(settingsStore.current.audioEnabled);
      audio.setVolume(settingsStore.current.audioVolume);
    } catch {}

    keybindings.init();
    keybindings.register({
      key: 't', ctrl: true, shift: true,
      handler: addNewTab,
      description: 'New terminal tab',
    });
    keybindings.register({
      key: ',', ctrl: true,
      handler: () => { settingsOpen = true; },
      description: 'Open settings',
    });
    keybindings.register({
      key: 'p', ctrl: true,
      handler: () => { fuzzyOpen = true; },
      description: 'Fuzzy file finder',
    });
    keybindings.register({
      key: 'e', ctrl: true, shift: true,
      handler: () => { themeEditorOpen = true; },
      description: 'Open theme editor',
    });

    uptimeInterval = setInterval(async () => {
      try { systemStore.setUptime(await getUptime()); } catch {}
    }, 60000);
  }

  function addNewTab() {
    const idx = terminalStore.tabs.length + 1;
    const tabId = crypto.randomUUID?.() || Math.random().toString(36).slice(2);
    const tab: TerminalTab = {
      id: tabId,
      title: `Terminal ${idx}`,
      sessionId: null,
      isActive: true,
    };
    terminalStore.addTab(tab);
  }

  function addTabWithCommand(title: string, command: string) {
    const tabId = crypto.randomUUID?.() || Math.random().toString(36).slice(2);
    const tab: TerminalTab = {
      id: tabId,
      title,
      sessionId: null,
      isActive: true,
    };
    pendingCommands[tabId] = command;
    terminalStore.addTab(tab);
  }

  function handleSession(tabId: string, sessionId: string) {
    tabSessions[tabId] = sessionId;
    const cmd = pendingCommands[tabId];
    if (cmd) {
      setTimeout(() => {
        ptyWrite(sessionId, cmd + '\n');
      }, 300);
      delete pendingCommands[tabId];
    }
  }

  function closeTab(id: string) {
    terminalStore.removeTab(id);
    delete tabSessions[id];
    delete pendingCommands[id];
  }

  let terminalCwd = $state('');

  function handleCwd(path: string) {
    terminalCwd = path;
    gitStore.setPath(path);
  }

  function runCommand(cmd: string) {
    if (activeSessionId) {
      ptyWrite(activeSessionId, cmd + '\n');
      cmdHistoryRef?.addToHistory(cmd);
      recordCommand(cmd);
    }
  }

  function recordCommand(cmd: string) {
    const clean = cmd.trim();
    if (!clean) return;
    commandFeed = [clean, ...commandFeed.filter(c => c !== clean)].slice(0, 120);
  }

  async function handleSettingsUpdate(partial: Record<string, any>) {
    if (partial._openThemeEditor) {
      themeEditorOpen = true;
      return;
    }
    settingsStore.update(partial);
    if (partial.theme) await themeStore.setTheme(partial.theme);
    if (partial.audioEnabled !== undefined) audio.setEnabled(partial.audioEnabled);
    if (partial.audioVolume !== undefined) audio.setVolume(partial.audioVolume);
  }

  function handlePanelChange(panel: string) {
    activePanel = panel;
    if (panel === 'kubernetes' && k8sStore.available) {
      k8sStore.refreshContexts();
      k8sStore.refreshAll();
    }
  }

  function handleSettingsClick() {
    settingsOpen = true;
  }

  function handleToggleRight() {
    rightCollapsed = !rightCollapsed;
  }

  let gridCols = $derived(`220px 1fr ${rightCollapsed ? '0px' : '280px'}`);
</script>

<UpdateBanner />

{#if initError}
  <div style="position:fixed;top:0;left:0;right:0;background:#f00;color:#fff;padding:8px;z-index:99999;font:12px monospace;">
    INIT ERROR: {initError}
  </div>
{/if}

<div
  style="width:100%;height:100%;display:grid;grid-template-columns:{gridCols};grid-template-rows:1fr 180px;"
>
  <!-- Left Sidebar -->
  <div style="grid-row:1/3;grid-column:1;border-right:1px solid var(--color-border);overflow:hidden;display:flex;flex-direction:column;">
    <Sidebar
      hostname={systemStore.hostname}
      osInfo={systemStore.osInfo}
      uptime={systemStore.uptime}
      {activePanel}
      {rightCollapsed}
      onpanelchange={handlePanelChange}
      onsettings={handleSettingsClick}
      ontoggleright={handleToggleRight}
    />
  </div>

  <!-- Main Terminal Area -->
  <div style="grid-row:1;grid-column:2;display:flex;flex-direction:column;overflow:hidden;">
    <TerminalTabs
      tabs={terminalStore.tabs}
      activeTabId={terminalStore.activeTabId}
      onselect={(id) => terminalStore.setActive(id)}
      onclose={closeTab}
      onadd={addNewTab}
      onrename={(id, title) => terminalStore.updateTitle(id, title)}
    />
    <div style="flex:1;position:relative;overflow:hidden;">
      {#each terminalStore.tabs as tab (tab.id)}
        <div
          style="position:absolute;inset:0;{tab.id !== terminalStore.activeTabId ? 'display:none;' : ''}"
        >
          <Terminal
            theme={themeStore.current}
            fontSize={settingsStore.current.fontSize}
            onsession={(sid) => handleSession(tab.id, sid)}
            oncwd={handleCwd}
            oncommand={recordCommand}
          />
        </div>
      {/each}
    </div>
    <AIAssistant
      provider={settingsStore.current.aiProvider}
      model={settingsStore.current.aiModel}
      apiKey={settingsStore.current.aiApiKey}
      litellmBaseUrl={settingsStore.current.litellmBaseUrl}
      litellmApiKey={settingsStore.current.litellmApiKey}
      litellmModel={settingsStore.current.litellmModel}
      providers={settingsStore.current.aiProviders || []}
      activeProviderId={settingsStore.current.activeProviderId || ''}
      cwd={terminalCwd}
      osInfo={systemStore.osInfo}
      recentCommands={commandFeed.slice(0, 15)}
      oncommand={runCommand}
      onchangeprovider={(id) => settingsStore.update({ activeProviderId: id })}
      onchangemodel={(providerId, newModel) => {
        const provs = [...(settingsStore.current.aiProviders || [])];
        const idx = provs.findIndex(p => p.id === providerId);
        if (idx >= 0) {
          provs[idx] = { ...provs[idx], model: newModel };
          settingsStore.update({ aiProviders: provs });
        }
      }}
    />
  </div>

  <!-- Right Panel -->
  {#if !rightCollapsed}
    <div style="grid-row:1/3;grid-column:3;border-left:1px solid var(--color-border);overflow:hidden;display:flex;flex-direction:column;">
      {#if activePanel === 'system'}
        <div style="flex:1;display:flex;flex-direction:column;gap:8px;padding:8px;overflow-y:auto;">
          <SystemMonitor
            cpu={systemStore.cpu}
            memory={systemStore.memory}
            disks={systemStore.disks}
          />
          <Panel title="NETWORK" collapsible>
            <NetworkMonitor network={systemStore.network} />
          </Panel>
          <Panel title="TOP PROCESSES" collapsible>
            <ProcessList processes={systemStore.processes} />
          </Panel>
          <Panel title="INTEL HUB" collapsible>
            <div style="height:260px;">
              <Globe
                baseColor={themeStore.current?.globe?.baseColor}
                markerColor={themeStore.current?.globe?.markerColor}
                commands={commandFeed}
                network={systemStore.network}
                processes={systemStore.processes}
              />
            </div>
          </Panel>
          <Panel title="DISKS" collapsible>
            <div style="display:flex;flex-direction:column;gap:4px;">
              {#if systemStore.disks && systemStore.disks.length > 0}
                {#each systemStore.disks as disk}
                  <div style="display:flex;align-items:center;justify-content:space-between;font-size:10px;">
                    <span style="color:var(--color-text);max-width:80px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap;">{disk.mount_point}</span>
                    <div style="flex:1;margin:0 8px;height:4px;background:var(--color-bg-primary);border-radius:2px;overflow:hidden;">
                      <div
                        style="height:100%;border-radius:2px;transition:all 0.5s;width:{disk.usage_percent}%;background:{disk.usage_percent >= 90 ? 'var(--color-error)' : disk.usage_percent >= 70 ? 'var(--color-warning)' : 'var(--color-success)'};"
                      ></div>
                    </div>
                    <span style="color:var(--color-text-bright);width:40px;text-align:right;">{disk.usage_percent.toFixed(0)}%</span>
                  </div>
                {/each}
              {:else}
                <span style="font-size:10px;color:var(--color-text);opacity:0.5;">No disk data</span>
              {/if}
            </div>
          </Panel>
        </div>
      {:else if activePanel === 'docker'}
        <DockerPanel
          containers={dockerStore.containers}
          images={dockerStore.images}
          available={dockerStore.available}
          onrefresh={() => {
            dockerStore.refreshContainers();
            dockerStore.refreshImages();
          }}
          onexec={(name, id) => addTabWithCommand(`🐳 ${name}`, `docker exec -it ${id} /bin/sh`)}
        />
      {:else if activePanel === 'kubernetes'}
        <KubernetesPanel
          available={k8sStore.available}
          ocAvailable={k8sStore.ocAvailable}
          contexts={k8sStore.contexts}
          namespaces={k8sStore.namespaces}
          currentNamespace={k8sStore.currentNamespace}
          pods={k8sStore.pods}
          deployments={k8sStore.deployments}
          services={k8sStore.services}
          loading={k8sStore.loading}
          error={k8sStore.error}
          clusterProfiles={k8sStore.clusterProfiles}
          activeClusterId={k8sStore.activeClusterId}
          kubeconfig={k8sStore.kubeconfig}
          onrefresh={() => k8sStore.refreshAll()}
          onswitchcontext={(name) => k8sStore.switchContext(name)}
          onsetnamespace={(ns) => k8sStore.setNamespace(ns)}
          onexec={(pod, ns, container) => {
            const kc = k8sStore.kubeconfig ? `--kubeconfig=${k8sStore.kubeconfig} ` : '';
            const c = container ? `-c ${container} ` : '';
            addTabWithCommand(`☸ ${pod}`, `kubectl ${kc}exec -it ${pod} -n ${ns} ${c}-- /bin/sh`);
          }}
          onlogs={(pod, ns, container) => {
            const kc = k8sStore.kubeconfig ? `--kubeconfig=${k8sStore.kubeconfig} ` : '';
            const c = container ? `-c ${container} ` : '';
            addTabWithCommand(`📋 ${pod}`, `kubectl ${kc}logs -f ${pod} -n ${ns} ${c}`);
          }}
          onaddcluster={(profile) => { k8sStore.addCluster(profile); k8sStore.activateCluster(profile.id); }}
          onremovecluster={(id) => k8sStore.removeCluster(id)}
          onactivatecluster={(id) => k8sStore.activateCluster(id)}
        />
      {:else if activePanel === 'git'}
        <GitPanel
          branches={gitStore.branches}
          commits={gitStore.commits}
          status={gitStore.status}
          diffs={gitStore.diffs}
          isRepo={gitStore.isRepo}
          currentPath={gitStore.currentPath}
          onrefresh={() => gitStore.refresh()}
          onpathchange={(path) => gitStore.setPath(path)}
          onloaddiff={(staged) => gitStore.loadDiff(staged)}
        />
      {:else if activePanel === 'ssh'}
        <SSHManager
          profiles={sshStore.profiles}
          onsave={(p) => sshStore.save(p)}
          ondelete={(id) => sshStore.remove(id)}
          onconnect={(p) => {
            let cmd = `ssh ${p.username}@${p.host} -p ${p.port}`;
            if (p.auth_method === 'PrivateKey' && p.private_key_path) {
              cmd = `ssh -i ${p.private_key_path} ${p.username}@${p.host} -p ${p.port}`;
            }
            addTabWithCommand(`⇋ ${p.name}`, cmd);
          }}
        />
      {:else if activePanel === 'plugins'}
        <PluginManager />
      {/if}
    </div>
  {/if}

  <!-- Bottom: File Explorer + Command History -->
  <div style="grid-row:2;grid-column:2{rightCollapsed ? '/4' : ''};border-top:1px solid var(--color-border);display:flex;overflow:hidden;">
    <div style="width:280px;border-right:1px solid var(--color-border);overflow:hidden;">
      <FileExplorer onfileclick={(name) => {
        if (activeSessionId) ptyWrite(activeSessionId, name + ' ');
      }} />
    </div>
    <div style="flex:1;overflow:hidden;">
      <CommandHistory bind:this={cmdHistoryRef} oncommand={runCommand} />
    </div>
  </div>
</div>

<SettingsModal
  bind:open={settingsOpen}
  settings={settingsStore.current}
  onupdate={handleSettingsUpdate}
/>

<FuzzyFinder
  bind:open={fuzzyOpen}
  basePath={terminalCwd || '/'}
  onselect={(path) => {
    if (activeSessionId) ptyWrite(activeSessionId, path + ' ');
  }}
/>

<ThemeEditor
  bind:open={themeEditorOpen}
  theme={themeStore.current}
  onsave={async (customTheme) => {
    try {
      localStorage.setItem('nexterm-custom-theme', JSON.stringify(customTheme));
      themeStore.applyTheme(customTheme);
    } catch {}
  }}
/>
