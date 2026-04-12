import type { SSHProfile } from '$lib/types';
import { sshGetProfiles, sshSaveProfile, sshDeleteProfile } from '$lib/utils/ipc';

let profiles = $state<SSHProfile[]>([]);

export function getSSHStore() {
  async function load() {
    try {
      profiles = await sshGetProfiles();
    } catch (e) {
      console.error('SSH profiles error:', e);
    }
  }

  async function save(profile: SSHProfile) {
    await sshSaveProfile(profile);
    await load();
  }

  async function remove(id: string) {
    await sshDeleteProfile(id);
    await load();
  }

  return {
    get profiles() { return profiles; },
    load,
    save,
    remove,
  };
}
