import type { ContainerInfo, ImageInfo } from '$lib/types';
import { dockerListContainers, dockerListImages, dockerIsAvailable } from '$lib/utils/ipc';

let containers = $state<ContainerInfo[]>([]);
let images = $state<ImageInfo[]>([]);
let available = $state(false);
let loading = $state(false);

export function getDockerStore() {
  async function checkAvailability() {
    try {
      available = await dockerIsAvailable();
    } catch {
      available = false;
    }
  }

  async function refreshContainers(all = true) {
    if (!available) return;
    loading = true;
    try {
      containers = await dockerListContainers(all);
    } catch (e) {
      console.error('Docker containers error:', e);
    } finally {
      loading = false;
    }
  }

  async function refreshImages() {
    if (!available) return;
    try {
      images = await dockerListImages();
    } catch (e) {
      console.error('Docker images error:', e);
    }
  }

  return {
    get containers() { return containers; },
    get images() { return images; },
    get available() { return available; },
    get loading() { return loading; },
    checkAvailability,
    refreshContainers,
    refreshImages,
  };
}
