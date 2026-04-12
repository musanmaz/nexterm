<script lang="ts">
  import ConnectionList from './ConnectionList.svelte';
  import Modal from '$lib/components/shared/Modal.svelte';
  import type { SSHProfile } from '$lib/types';

  let { profiles = [], onsave, ondelete, onconnect }: {
    profiles?: SSHProfile[];
    onsave?: (profile: SSHProfile) => void;
    ondelete?: (id: string) => void;
    onconnect?: (profile: SSHProfile) => void;
  } = $props();

  let showForm = $state(false);
  let editId = $state('');
  let name = $state('');
  let host = $state('');
  let port = $state(22);
  let username = $state('');
  let authMethod = $state<'Password' | 'PrivateKey' | 'Agent'>('PrivateKey');
  let privateKeyPath = $state('');

  function openNew() {
    editId = '';
    name = '';
    host = '';
    port = 22;
    username = '';
    authMethod = 'PrivateKey';
    privateKeyPath = '~/.ssh/id_rsa';
    showForm = true;
  }

  function edit(profile: SSHProfile) {
    editId = profile.id;
    name = profile.name;
    host = profile.host;
    port = profile.port;
    username = profile.username;
    authMethod = profile.auth_method;
    privateKeyPath = profile.private_key_path || '';
    showForm = true;
  }

  function save() {
    const profile: SSHProfile = {
      id: editId || crypto.randomUUID(),
      name, host, port, username,
      auth_method: authMethod,
      private_key_path: authMethod === 'PrivateKey' ? privateKeyPath : undefined,
    };
    onsave?.(profile);
    showForm = false;
  }

  const inputStyle = 'width:100%;background:var(--color-bg-primary);border:1px solid var(--color-border);color:var(--color-text);font-size:12px;padding:6px 8px;font-family:inherit;outline:none;box-sizing:border-box;';
  const labelStyle = 'display:block;font-size:10px;color:var(--color-text);letter-spacing:2px;margin-bottom:4px;';
</script>

<div style="height:100%;display:flex;flex-direction:column;">
  <div style="display:flex;align-items:center;justify-content:space-between;padding:8px;border-bottom:1px solid var(--color-border);">
    <span style="font-size:10px;letter-spacing:2px;color:var(--color-text);">SSH CONNECTIONS</span>
    <button
      type="button"
      style="padding:4px 10px;font-size:10px;background:var(--color-primary);color:var(--color-bg-primary);border:none;cursor:pointer;letter-spacing:1px;"
      onclick={openNew}
    >+ NEW</button>
  </div>
  <div style="flex:1;overflow-y:auto;">
    <ConnectionList {profiles} {onconnect} {ondelete} onedit={edit} />
  </div>
</div>

<Modal title={editId ? 'EDIT SSH PROFILE' : 'NEW SSH PROFILE'} bind:open={showForm}>
  <div style="display:flex;flex-direction:column;gap:12px;">
    <div>
      <label for="ssh-name" style={labelStyle}>NAME</label>
      <input id="ssh-name" bind:value={name} placeholder="My Server" style={inputStyle} />
    </div>
    <div style="display:grid;grid-template-columns:1fr 80px;gap:8px;">
      <div>
        <label for="ssh-host" style={labelStyle}>HOST</label>
        <input id="ssh-host" bind:value={host} placeholder="192.168.1.100" style={inputStyle} />
      </div>
      <div>
        <label for="ssh-port" style={labelStyle}>PORT</label>
        <input id="ssh-port" type="number" bind:value={port} style={inputStyle} />
      </div>
    </div>
    <div>
      <label for="ssh-username" style={labelStyle}>USERNAME</label>
      <input id="ssh-username" bind:value={username} placeholder="root" style={inputStyle} />
    </div>
    <div>
      <label for="ssh-auth" style={labelStyle}>AUTH METHOD</label>
      <select id="ssh-auth" bind:value={authMethod} style={inputStyle}>
        <option value="PrivateKey">Private Key</option>
        <option value="Password">Password</option>
        <option value="Agent">SSH Agent</option>
      </select>
    </div>
    {#if authMethod === 'PrivateKey'}
      <div>
        <label for="ssh-keypath" style={labelStyle}>PRIVATE KEY PATH</label>
        <input id="ssh-keypath" bind:value={privateKeyPath} placeholder="~/.ssh/id_rsa" style={inputStyle} />
        <div style="font-size:9px;color:var(--color-text);opacity:0.5;margin-top:4px;">
          Enter full path, e.g.: ~/.ssh/id_rsa, ~/.ssh/id_ed25519
        </div>
      </div>
    {/if}
    <button
      type="button"
      style="width:100%;padding:8px;font-size:12px;background:var(--color-primary);color:var(--color-bg-primary);border:none;cursor:pointer;letter-spacing:2px;"
      onclick={save}
    >SAVE</button>
  </div>
</Modal>
