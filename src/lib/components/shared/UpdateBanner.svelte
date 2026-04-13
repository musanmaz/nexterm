<script lang="ts">
  import { onMount } from 'svelte';
  import { checkForUpdates, type UpdateInfo } from '$lib/utils/updater';
  import { downloadUpdate, installAndRestart } from '$lib/utils/ipc';

  let updateInfo = $state<UpdateInfo | null>(null);
  let dismissed = $state(false);
  let downloading = $state(false);
  let downloadProgress = $state('');
  let downloadedPath = $state('');
  let error = $state('');

  onMount(async () => {
    try {
      const info = await checkForUpdates();
      if (info.hasUpdate) {
        updateInfo = info;
      }
    } catch (e) {
      console.warn('Update check failed:', e);
    }
  });

  async function handleDownload() {
    if (!updateInfo?.downloadUrl) return;

    downloading = true;
    downloadProgress = 'Downloading...';
    error = '';

    try {
      const filename = `NEXTERM-${updateInfo.latestVersion}-arm64.dmg`;
      const path = await downloadUpdate(updateInfo.downloadUrl, filename);
      downloadedPath = path;
      downloadProgress = 'Download complete!';
    } catch (e) {
      error = `Download failed: ${e}`;
      downloadProgress = '';
    } finally {
      downloading = false;
    }
  }

  async function handleRestart() {
    if (!downloadedPath) return;
    try {
      await installAndRestart(downloadedPath);
    } catch (e) {
      error = `Restart failed: ${e}`;
    }
  }
</script>

{#if updateInfo && !dismissed}
  <div style="
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    z-index: 99998;
    background: linear-gradient(135deg, #0a1628 0%, #0d2137 100%);
    border-bottom: 2px solid var(--color-primary, #00d4ff);
    padding: 10px 20px;
    display: flex;
    align-items: center;
    gap: 16px;
    font-family: inherit;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.5);
  ">
    <!-- Icon -->
    <div style="
      width: 36px;
      height: 36px;
      border-radius: 8px;
      background: var(--color-primary, #00d4ff);
      display: flex;
      align-items: center;
      justify-content: center;
      font-size: 18px;
      flex-shrink: 0;
    ">⬆</div>

    <!-- Info -->
    <div style="flex: 1; min-width: 0;">
      <div style="font-size: 12px; color: var(--color-text-bright, #fff); letter-spacing: 1px; font-weight: 600;">
        New version available: <span style="color: var(--color-primary, #00d4ff);">v{updateInfo.latestVersion}</span>
      </div>
      <div style="font-size: 10px; color: var(--color-text, #aaa); opacity: 0.7; margin-top: 2px;">
        Current: v{updateInfo.currentVersion}
        {#if downloadProgress}
          · <span style="color: var(--color-warning, #ffd700);">{downloadProgress}</span>
        {/if}
        {#if error}
          · <span style="color: var(--color-error, #ff4444);">{error}</span>
        {/if}
      </div>
    </div>

    <!-- Actions -->
    <div style="display: flex; gap: 8px; flex-shrink: 0;">
      {#if downloadedPath}
        <button
          type="button"
          onclick={handleRestart}
          style="
            padding: 6px 16px;
            font-size: 11px;
            background: var(--color-success, #00ff88);
            color: #000;
            border: none;
            cursor: pointer;
            letter-spacing: 1px;
            font-family: inherit;
            font-weight: 600;
          "
        >RESTART & LAUNCH</button>
      {:else}
        <button
          type="button"
          onclick={handleDownload}
          disabled={downloading}
          style="
            padding: 6px 16px;
            font-size: 11px;
            background: var(--color-primary, #00d4ff);
            color: #000;
            border: none;
            cursor: {downloading ? 'wait' : 'pointer'};
            letter-spacing: 1px;
            font-family: inherit;
            font-weight: 600;
            opacity: {downloading ? 0.6 : 1};
          "
        >{downloading ? 'DOWNLOADING...' : 'DOWNLOAD'}</button>
      {/if}
      <button
        type="button"
        onclick={() => dismissed = true}
        style="
          padding: 6px 12px;
          font-size: 11px;
          background: transparent;
          color: var(--color-text, #aaa);
          border: 1px solid var(--color-border, #333);
          cursor: pointer;
          letter-spacing: 1px;
          font-family: inherit;
        "
      >LATER</button>
    </div>
  </div>
{/if}
