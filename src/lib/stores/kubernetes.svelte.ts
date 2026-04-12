import type { K8sContext, K8sPod, K8sDeployment, K8sService, K8sNamespace } from '$lib/types';
import {
  k8sIsAvailable, k8sGetContexts, k8sSwitchContext,
  k8sGetNamespaces, k8sGetPods, k8sGetDeployments, k8sGetServices,
} from '$lib/utils/ipc';

let available = $state(false);
let contexts = $state<K8sContext[]>([]);
let namespaces = $state<K8sNamespace[]>([]);
let currentNamespace = $state('default');
let pods = $state<K8sPod[]>([]);
let deployments = $state<K8sDeployment[]>([]);
let services = $state<K8sService[]>([]);
let loading = $state(false);
let error = $state('');

export function getKubernetesStore() {
  async function checkAvailable() {
    try { available = await k8sIsAvailable(); } catch { available = false; }
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

  async function init() {
    await checkAvailable();
    if (available) {
      await refreshContexts();
      await refreshNamespaces();
      await refreshAll();
    }
  }

  return {
    get available() { return available; },
    get contexts() { return contexts; },
    get namespaces() { return namespaces; },
    get currentNamespace() { return currentNamespace; },
    get pods() { return pods; },
    get deployments() { return deployments; },
    get services() { return services; },
    get loading() { return loading; },
    get error() { return error; },
    init,
    checkAvailable,
    refreshContexts,
    switchContext,
    refreshNamespaces,
    setNamespace,
    refreshAll,
  };
}
