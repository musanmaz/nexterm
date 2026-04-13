import type { K8sContext, K8sPod, K8sDeployment, K8sService, K8sNamespace, ClusterProfile } from '$lib/types';
import {
  k8sIsAvailable, k8sGetContexts, k8sSwitchContext,
  k8sGetNamespaces, k8sGetPods, k8sGetDeployments, k8sGetServices,
  k8sOcLogin, k8sOcIsAvailable, k8sGetContextsForKubeconfig,
} from '$lib/utils/ipc';

const STORAGE_KEY = 'nexterm-cluster-profiles';

let available = $state(false);
let ocAvailable = $state(false);
let contexts = $state<K8sContext[]>([]);
let namespaces = $state<K8sNamespace[]>([]);
let currentNamespace = $state('default');
let pods = $state<K8sPod[]>([]);
let deployments = $state<K8sDeployment[]>([]);
let services = $state<K8sService[]>([]);
let loading = $state(false);
let error = $state('');
let clusterProfiles = $state<ClusterProfile[]>([]);
let activeClusterId = $state('default');

function loadProfiles() {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (raw) {
      const parsed = JSON.parse(raw) as { profiles: ClusterProfile[]; activeId: string };
      clusterProfiles = parsed.profiles || [];
      activeClusterId = parsed.activeId || 'default';
    }
  } catch {}
  if (!clusterProfiles.some(p => p.id === 'default')) {
    clusterProfiles = [{ id: 'default', name: 'Default (~/.kube/config)', type: 'default' }, ...clusterProfiles];
  }
}

function saveProfiles() {
  localStorage.setItem(STORAGE_KEY, JSON.stringify({
    profiles: clusterProfiles,
    activeId: activeClusterId,
  }));
}

export function getKubernetesStore() {
  async function checkAvailable() {
    try { available = await k8sIsAvailable(); } catch { available = false; }
    try { ocAvailable = await k8sOcIsAvailable(); } catch { ocAvailable = false; }
  }

  async function refreshContexts() {
    try {
      contexts = await k8sGetContexts();
      error = '';
    } catch (e) { error = String(e); }
  }

  async function switchContext(name: string) {
    try {
      await k8sSwitchContext(name);
      await refreshContexts();
      await refreshNamespaces();
      currentNamespace = 'default';
      await refreshAll();
    } catch (e) { error = String(e); }
  }

  async function refreshNamespaces() {
    try {
      namespaces = await k8sGetNamespaces();
    } catch { namespaces = []; }
  }

  function setNamespace(ns: string) {
    currentNamespace = ns;
    refreshAll();
  }

  async function refreshAll() {
    loading = true;
    error = '';
    try {
      const [p, d, s] = await Promise.allSettled([
        k8sGetPods(currentNamespace),
        k8sGetDeployments(currentNamespace),
        k8sGetServices(currentNamespace),
      ]);
      pods = p.status === 'fulfilled' ? p.value : [];
      deployments = d.status === 'fulfilled' ? d.value : [];
      services = s.status === 'fulfilled' ? s.value : [];
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  function addCluster(profile: ClusterProfile) {
    clusterProfiles = [...clusterProfiles, profile];
    saveProfiles();
  }

  function removeCluster(id: string) {
    if (id === 'default') return;
    clusterProfiles = clusterProfiles.filter(p => p.id !== id);
    if (activeClusterId === id) activeClusterId = 'default';
    saveProfiles();
  }

  function updateCluster(profile: ClusterProfile) {
    clusterProfiles = clusterProfiles.map(p => p.id === profile.id ? profile : p);
    saveProfiles();
  }

  async function activateCluster(id: string) {
    activeClusterId = id;
    saveProfiles();
    const profile = clusterProfiles.find(p => p.id === id);
    if (!profile) return;

    error = '';
    loading = true;
    try {
      if (profile.type === 'openshift' && profile.apiUrl && profile.username && profile.password) {
        await k8sOcLogin(
          profile.apiUrl,
          profile.username,
          profile.password,
          profile.insecureSkipTls ?? true,
        );
      }
      await refreshContexts();
      await refreshNamespaces();
      currentNamespace = 'default';
      await refreshAll();
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  async function init() {
    loadProfiles();
    await checkAvailable();
    if (available) {
      await refreshContexts();
      await refreshNamespaces();
      await refreshAll();
    }
  }

  return {
    get available() { return available; },
    get ocAvailable() { return ocAvailable; },
    get contexts() { return contexts; },
    get namespaces() { return namespaces; },
    get currentNamespace() { return currentNamespace; },
    get pods() { return pods; },
    get deployments() { return deployments; },
    get services() { return services; },
    get loading() { return loading; },
    get error() { return error; },
    get clusterProfiles() { return clusterProfiles; },
    get activeClusterId() { return activeClusterId; },
    init,
    checkAvailable,
    refreshContexts,
    switchContext,
    refreshNamespaces,
    setNamespace,
    refreshAll,
    addCluster,
    removeCluster,
    updateCluster,
    activateCluster,
  };
}
