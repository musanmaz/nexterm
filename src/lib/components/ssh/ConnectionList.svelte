<script lang="ts">
  import type { SSHProfile } from '$lib/types';

  let { profiles = [], onconnect, ondelete, onedit }: {
    profiles?: SSHProfile[];
    onconnect?: (profile: SSHProfile) => void;
    ondelete?: (id: string) => void;
    onedit?: (profile: SSHProfile) => void;
  } = $props();
</script>

<div style="padding:4px;">
  {#each profiles as profile}
    <div style="display:flex;align-items:center;gap:8px;padding:8px;border-bottom:1px solid var(--color-border);">
      <div style="font-size:16px;opacity:0.4;">⇋</div>
      <div style="flex:1;min-width:0;">
        <div style="font-size:11px;color:var(--color-text-bright);overflow:hidden;text-overflow:ellipsis;white-space:nowrap;">{profile.name}</div>
        <div style="font-size:9px;color:var(--color-text);opacity:0.6;">{profile.username}@{profile.host}:{profile.port}</div>
        {#if profile.auth_method === 'PrivateKey' && profile.private_key_path}
          <div style="font-size:8px;color:var(--color-text);opacity:0.4;">🔑 {profile.private_key_path}</div>
        {/if}
      </div>
      <div style="display:flex;gap:3px;flex-shrink:0;">
        <button
          type="button"
          style="padding:3px 8px;font-size:9px;background:var(--color-success);color:#000;border:none;cursor:pointer;letter-spacing:1px;"
          onclick={() => onconnect?.(profile)}
          title="Connect via SSH"
        >CONNECT</button>
        <button
          type="button"
          style="padding:3px 8px;font-size:9px;background:var(--color-primary);color:#000;border:none;cursor:pointer;letter-spacing:1px;"
          onclick={() => onedit?.(profile)}
          title="Edit profile"
        >✎</button>
        <button
          type="button"
          style="padding:3px 8px;font-size:9px;background:var(--color-error);color:#fff;border:none;cursor:pointer;letter-spacing:1px;"
          onclick={() => ondelete?.(profile.id)}
          title="Delete profile"
        >✕</button>
      </div>
    </div>
  {/each}
  {#if profiles.length === 0}
    <div style="text-align:center;font-size:10px;color:var(--color-text);opacity:0.5;padding:24px;">
      <div style="font-size:24px;margin-bottom:8px;opacity:0.3;">⇋</div>
      No saved connections
    </div>
  {/if}
</div>
