<script lang="ts">
  import type { K8sContext, K8sPod, K8sDeployment, K8sService, K8sNamespace } from '$lib/types';
  import { k8sScaleDeployment, k8sRestartDeployment, k8sDeletePod } from '$lib/utils/ipc';

  let {
    available = false,
    contexts = [],
    namespaces = [],
    currentNamespace = 'default',
    pods = [],
    deployments = [],
    services = [],
    loading = false,
    error = '',
    onrefresh,
    onswitchcontext,
    onsetnamespace,
    onexec,
    onlogs,
  }: {
    available?: boolean;
    contexts?: K8sContext[];
    namespaces?: K8sNamespace[];
    currentNamespace?: string;
    pods?: K8sPod[];
    deployments?: K8sDeployment[];
    services?: K8sService[];
    loading?: boolean;
    error?: string;
    onrefresh?: () => void;
    onswitchcontext?: (name: string) => void;
    onsetnamespace?: (ns: string) => void;
    onexec?: (podName: string, namespace: string, container?: string) => void;
    onlogs?: (podName: string, namespace: string, container?: string) => void;
  } = $props();

  let activeTab = $state<'pods' | 'deployments' | 'services'>('pods');
  let actionMsg = $state('');
  let scaleTarget = $state<{ name: string; ns: string; replicas: number } | null>(null);

  function podStatusColor(status: string): string {
    if (status === 'Running') return 'var(--color-success)';
    if (status === 'Succeeded') return 'var(--color-primary)';
    if (status === 'Pending') return 'var(--color-warning)';
    return 'var(--color-error)';
  }

  function deployHealthColor(ready: string): string {
    const [r, t] = ready.split('/').map(Number);
    if (r === t && t > 0) return 'var(--color-success)';
    if (r > 0) return 'var(--color-warning)';
    return 'var(--color-error)';
  }

  async function handleScale() {
    if (!scaleTarget) return;
    try {
      await k8sScaleDeployment(scaleTarget.ns, scaleTarget.name, scaleTarget.replicas);
      actionMsg = `Scaled ${scaleTarget.name} to ${scaleTarget.replicas}`;
      scaleTarget = null;
      setTimeout(() => onrefresh?.(), 1000);
    } catch (e) { actionMsg = `Scale error: ${e}`; }
    setTimeout(() => { actionMsg = ''; }, 3000);
  }

  async function handleRestart(ns: string, name: string) {
    try {
      await k8sRestartDeployment(ns, name);
      actionMsg = `Restarting ${name}...`;
      setTimeout(() => onrefresh?.(), 2000);
    } catch (e) { actionMsg = `Restart error: ${e}`; }
    setTimeout(() => { actionMsg = ''; }, 3000);
  }

  async function handleDeletePod(ns: string, name: string) {
    try {
      await k8sDeletePod(ns, name);
      actionMsg = `Deleted pod ${name}`;
      setTimeout(() => onrefresh?.(), 1000);
    } catch (e) { actionMsg = `Delete error: ${e}`; }
    setTimeout(() => { actionMsg = ''; }, 3000);
  }

  const currentCtx = $derived(contexts.find(c => c.is_current));
</script>

<div style="height:100%;display:flex;flex-direction:column;overflow:hidden;">
  <!-- Header -->
  <div style="display:flex;align-items:center;justify-content:space-between;padding:8px;border-bottom:1px solid var(--color-border);">
    <span style="font-size:10px;letter-spacing:2px;color:var(--color-text);">☸ KUBERNETES</span>
    <div style="display:flex;gap:4px;">
      {#if loading}
        <span style="font-size:9px;color:var(--color-warning);letter-spacing:1px;">LOADING...</span>
      {/if}
      <button
        type="button"
        style="padding:3px 8px;font-size:9px;background:var(--color-primary);color:var(--color-bg-primary);border:none;cursor:pointer;letter-spacing:1px;"
        onclick={() => onrefresh?.()}
      >↻ REFRESH</button>
    </div>
  </div>

  {#if !available}
    <div style="flex:1;display:flex;align-items:center;justify-content:center;flex-direction:column;gap:8px;padding:16px;">
      <div style="font-size:24px;opacity:0.3;">☸</div>
      <div style="font-size:11px;color:var(--color-text);opacity:0.5;">kubectl not found</div>
      <div style="font-size:9px;color:var(--color-text);opacity:0.3;">Make sure kubectl is installed and available in PATH</div>
    </div>
  {:else}
    <!-- Context Switcher -->
    <div style="padding:8px;border-bottom:1px solid var(--color-border);display:flex;flex-direction:column;gap:6px;">
      <div style="display:flex;align-items:center;gap:8px;">
        <span style="font-size:9px;color:var(--color-text);opacity:0.6;letter-spacing:1px;flex-shrink:0;">CONTEXT</span>
        <select
          value={currentCtx?.name || ''}
          onchange={(e) => onswitchcontext?.((e.target as HTMLSelectElement).value)}
          style="flex:1;background:var(--color-bg-primary);border:1px solid var(--color-border);color:var(--color-text-bright);font-size:11px;padding:4px 6px;font-family:inherit;outline:none;"
        >
          {#each contexts as ctx}
            <option value={ctx.name}>{ctx.name} ({ctx.cluster})</option>
          {/each}
        </select>
      </div>
      <div style="display:flex;align-items:center;gap:8px;">
        <span style="font-size:9px;color:var(--color-text);opacity:0.6;letter-spacing:1px;flex-shrink:0;">NAMESPACE</span>
        <select
          value={currentNamespace}
          onchange={(e) => onsetnamespace?.((e.target as HTMLSelectElement).value)}
          style="flex:1;background:var(--color-bg-primary);border:1px solid var(--color-border);color:var(--color-text-bright);font-size:11px;padding:4px 6px;font-family:inherit;outline:none;"
        >
          {#each namespaces as ns}
            <option value={ns.name}>{ns.name}</option>
          {/each}
        </select>
      </div>
    </div>

    <!-- Action Message -->
    {#if actionMsg}
      <div style="padding:4px 8px;font-size:9px;background:{actionMsg.includes('error') ? 'var(--color-error)' : 'var(--color-success)'};color:#fff;text-align:center;">
        {actionMsg}
      </div>
    {/if}

    {#if error}
      <div style="padding:4px 8px;font-size:9px;background:var(--color-error);color:#fff;text-align:center;">{error}</div>
    {/if}

    <!-- Tabs -->
    <div style="display:flex;border-bottom:1px solid var(--color-border);">
      {#each [
        { id: 'pods' as const, label: 'PODS', count: pods.length },
        { id: 'deployments' as const, label: 'DEPLOYMENTS', count: deployments.length },
        { id: 'services' as const, label: 'SERVICES', count: services.length },
      ] as tab}
        <button
          type="button"
          style="flex:1;padding:6px;font-size:9px;letter-spacing:1px;border:none;cursor:pointer;border-bottom:2px solid {activeTab === tab.id ? 'var(--color-primary)' : 'transparent'};background:{activeTab === tab.id ? 'var(--color-bg-panel)' : 'transparent'};color:{activeTab === tab.id ? 'var(--color-text-bright)' : 'var(--color-text)'};"
          onclick={() => activeTab = tab.id}
        >{tab.label} <span style="opacity:0.5;">({tab.count})</span></button>
      {/each}
    </div>

    <!-- Content -->
    <div style="flex:1;overflow-y:auto;">
      {#if activeTab === 'pods'}
        <!-- PODS -->
        {#each pods as pod}
          <div style="display:flex;align-items:center;gap:6px;padding:6px 8px;border-bottom:1px solid var(--color-border);">
            <div style="width:8px;height:8px;border-radius:50%;flex-shrink:0;background:{podStatusColor(pod.status)};"></div>
            <div style="flex:1;min-width:0;overflow:hidden;">
              <div style="font-size:11px;color:var(--color-text-bright);overflow:hidden;text-overflow:ellipsis;white-space:nowrap;" title={pod.name}>{pod.name}</div>
              <div style="font-size:8px;color:var(--color-text);opacity:0.5;">
                {pod.status} · Ready: {pod.ready} · Restarts: {pod.restarts} · {pod.age}
              </div>
            </div>
            <div style="display:flex;gap:3px;flex-shrink:0;">
              {#if pod.status === 'Running'}
                <button type="button" style="padding:2px 6px;font-size:8px;background:var(--color-accent);color:#fff;border:none;cursor:pointer;" onclick={() => onexec?.(pod.name, pod.namespace, pod.containers[0])} title="Shell into pod">SH</button>
                <button type="button" style="padding:2px 6px;font-size:8px;background:var(--color-primary);color:#000;border:none;cursor:pointer;" onclick={() => onlogs?.(pod.name, pod.namespace, pod.containers[0])} title="View logs">LOG</button>
              {/if}
              <button type="button" style="padding:2px 6px;font-size:8px;background:var(--color-error);color:#fff;border:none;cursor:pointer;" onclick={() => handleDeletePod(pod.namespace, pod.name)} title="Delete pod">✕</button>
            </div>
          </div>
        {/each}
        {#if pods.length === 0}
          <div style="text-align:center;padding:24px;font-size:10px;color:var(--color-text);opacity:0.5;">No pods found in this namespace</div>
        {/if}

      {:else if activeTab === 'deployments'}
        <!-- DEPLOYMENTS -->
        {#each deployments as dep}
          <div style="display:flex;align-items:center;gap:6px;padding:6px 8px;border-bottom:1px solid var(--color-border);">
            <div style="width:8px;height:8px;border-radius:50%;flex-shrink:0;background:{deployHealthColor(dep.ready)};"></div>
            <div style="flex:1;min-width:0;overflow:hidden;">
              <div style="font-size:11px;color:var(--color-text-bright);overflow:hidden;text-overflow:ellipsis;white-space:nowrap;" title={dep.name}>{dep.name}</div>
              <div style="font-size:8px;color:var(--color-text);opacity:0.5;">
                Ready: {dep.ready} · Up-to-date: {dep.up_to_date} · {dep.age}
              </div>
            </div>
            <div style="display:flex;gap:3px;flex-shrink:0;align-items:center;">
              {#if scaleTarget?.name === dep.name}
                <input
                  type="number"
                  min="0"
                  max="100"
                  bind:value={scaleTarget.replicas}
                  style="width:40px;background:var(--color-bg-primary);border:1px solid var(--color-primary);color:var(--color-text-bright);font-size:10px;padding:2px 4px;text-align:center;outline:none;"
                />
                <button type="button" style="padding:2px 6px;font-size:8px;background:var(--color-success);color:#000;border:none;cursor:pointer;" onclick={handleScale}>OK</button>
                <button type="button" style="padding:2px 6px;font-size:8px;background:var(--color-border);color:var(--color-text);border:none;cursor:pointer;" onclick={() => scaleTarget = null}>✕</button>
              {:else}
                <button type="button" style="padding:2px 6px;font-size:8px;background:var(--color-primary);color:#000;border:none;cursor:pointer;" onclick={() => scaleTarget = { name: dep.name, ns: dep.namespace, replicas: dep.replicas }} title="Scale">⇅ {dep.replicas}</button>
                <button type="button" style="padding:2px 6px;font-size:8px;background:var(--color-warning);color:#000;border:none;cursor:pointer;" onclick={() => handleRestart(dep.namespace, dep.name)} title="Rollout restart">↻</button>
              {/if}
            </div>
          </div>
        {/each}
        {#if deployments.length === 0}
          <div style="text-align:center;padding:24px;font-size:10px;color:var(--color-text);opacity:0.5;">No deployments found in this namespace</div>
        {/if}

      {:else if activeTab === 'services'}
        <!-- SERVICES -->
        {#each services as svc}
          <div style="padding:6px 8px;border-bottom:1px solid var(--color-border);">
            <div style="display:flex;align-items:center;gap:6px;">
              <span style="font-size:9px;padding:1px 6px;background:var(--color-bg-primary);border:1px solid var(--color-border);color:var(--color-text);letter-spacing:1px;">{svc.svc_type}</span>
              <span style="font-size:11px;color:var(--color-text-bright);flex:1;overflow:hidden;text-overflow:ellipsis;white-space:nowrap;" title={svc.name}>{svc.name}</span>
              <span style="font-size:9px;color:var(--color-text);opacity:0.5;">{svc.age}</span>
            </div>
            <div style="font-size:8px;color:var(--color-text);opacity:0.5;margin-top:2px;">
              ClusterIP: {svc.cluster_ip} · External: {svc.external_ip} · Ports: {svc.ports}
            </div>
          </div>
        {/each}
        {#if services.length === 0}
          <div style="text-align:center;padding:24px;font-size:10px;color:var(--color-text);opacity:0.5;">No services found in this namespace</div>
        {/if}
      {/if}
    </div>

    <!-- Scale Dialog -->
    {#if scaleTarget && activeTab !== 'deployments'}
      <!-- hidden when not on deployments tab -->
    {/if}
  {/if}
</div>
