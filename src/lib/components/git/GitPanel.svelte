<script lang="ts">
  import { untrack } from 'svelte';
  import BranchGraph from './BranchGraph.svelte';
  import Panel from '$lib/components/shared/Panel.svelte';
  import type { GitBranch, GitCommit, GitStatus } from '$lib/types';

  let { branches = [], commits = [], status = null, isRepo = false, currentPath = '', onrefresh, onpathchange }: {
    branches?: GitBranch[];
    commits?: GitCommit[];
    status?: GitStatus | null;
    isRepo?: boolean;
    currentPath?: string;
    onrefresh?: () => void;
    onpathchange?: (path: string) => void;
  } = $props();

  let activeTab = $state<'status' | 'log' | 'branches'>('status');
  let pathInput = $state('');

  $effect(() => {
    if (currentPath) {
      const prev = untrack(() => pathInput);
      if (prev !== currentPath) {
        pathInput = currentPath;
      }
    }
  });

  function submitPath() {
    const trimmed = pathInput.trim();
    if (trimmed && trimmed !== currentPath) {
      onpathchange?.(trimmed);
    }
  }
</script>

<div style="height:100%;display:flex;flex-direction:column;">
  <!-- Path input -->
  <div style="padding:8px;border-bottom:1px solid var(--color-border);">
    <div style="font-size:9px;color:var(--color-text);opacity:0.5;margin-bottom:4px;letter-spacing:1px;">REPO PATH</div>
    <div style="display:flex;gap:4px;">
      <input
        type="text"
        bind:value={pathInput}
        placeholder="/path/to/repo"
        onkeydown={(e) => e.key === 'Enter' && submitPath()}
        style="flex:1;background:var(--color-bg-primary);border:1px solid var(--color-border);color:var(--color-text);font-size:10px;padding:4px 8px;font-family:var(--font-mono);outline:none;"
      />
      <button
        type="button"
        style="padding:4px 8px;font-size:10px;background:var(--color-primary);color:var(--color-bg-primary);border:none;cursor:pointer;"
        onclick={submitPath}
      >SET</button>
    </div>
  </div>

  {#if !isRepo}
    <div style="flex:1;display:flex;align-items:center;justify-content:center;">
      <div style="text-align:center;">
        <span style="font-size:24px;display:block;margin-bottom:8px;opacity:0.3;">⎇</span>
        <span style="font-size:11px;color:var(--color-text);opacity:0.5;">Not a git repository</span>
        <div style="font-size:9px;color:var(--color-text);opacity:0.3;margin-top:4px;">Enter a repo path above</div>
      </div>
    </div>
  {:else}
    <div style="display:flex;gap:8px;padding:8px;border-bottom:1px solid var(--color-border);">
      {#each [['status', 'STATUS'], ['log', 'LOG'], ['branches', 'BRANCHES']] as [tab, label]}
        <button
          type="button"
          style="
            padding:4px 12px;font-size:10px;letter-spacing:1px;cursor:pointer;transition:all 0.2s;
            background:{activeTab === tab ? 'var(--color-primary)' : 'transparent'};
            color:{activeTab === tab ? 'var(--color-bg-primary)' : 'var(--color-text)'};
            border:1px solid {activeTab === tab ? 'var(--color-primary)' : 'var(--color-border)'};
          "
          onclick={() => activeTab = tab as typeof activeTab}
        >{label}</button>
      {/each}
      <div style="flex:1;"></div>
      <button
        type="button"
        style="padding:4px 8px;font-size:10px;color:var(--color-text);background:transparent;border:1px solid var(--color-border);cursor:pointer;"
        onclick={onrefresh}
      >↻</button>
    </div>

    <div style="flex:1;overflow-y:auto;padding:8px;">
      {#if activeTab === 'status' && status}
        <div style="display:flex;flex-direction:column;gap:12px;">
          <div style="display:flex;align-items:center;gap:8px;font-size:12px;">
            <span style="color:var(--color-text);opacity:0.6;">BRANCH</span>
            <span class="glow-text" style="font-weight:bold;">{status.branch}</span>
          </div>

          {#if status.staged.length > 0}
            <div>
              <div style="font-size:10px;color:var(--color-success);margin-bottom:4px;letter-spacing:1px;">STAGED ({status.staged.length})</div>
              {#each status.staged as file}
                <div style="font-size:10px;color:var(--color-text);padding-left:8px;">+ {file}</div>
              {/each}
            </div>
          {/if}

          {#if status.unstaged.length > 0}
            <div>
              <div style="font-size:10px;color:var(--color-warning);margin-bottom:4px;letter-spacing:1px;">MODIFIED ({status.unstaged.length})</div>
              {#each status.unstaged as file}
                <div style="font-size:10px;color:var(--color-text);padding-left:8px;">~ {file}</div>
              {/each}
            </div>
          {/if}

          {#if status.untracked.length > 0}
            <div>
              <div style="font-size:10px;color:var(--color-text);opacity:0.6;margin-bottom:4px;letter-spacing:1px;">UNTRACKED ({status.untracked.length})</div>
              {#each status.untracked as file}
                <div style="font-size:10px;color:var(--color-text);opacity:0.5;padding-left:8px;">? {file}</div>
              {/each}
            </div>
          {/if}

          {#if status.staged.length === 0 && status.unstaged.length === 0 && status.untracked.length === 0}
            <div style="font-size:10px;color:var(--color-success);">Working tree clean</div>
          {/if}
        </div>
      {:else if activeTab === 'log'}
        <BranchGraph {commits} />
      {:else if activeTab === 'branches'}
        <div style="display:flex;flex-direction:column;gap:4px;">
          {#each branches as branch}
            <div style="display:flex;align-items:center;gap:8px;font-size:10px;padding:4px 8px;">
              <span style="width:8px;height:8px;border-radius:50%;background:{branch.is_head ? 'var(--color-success)' : 'var(--color-border)'};"></span>
              <span style="color:{branch.is_head ? 'var(--color-text-bright)' : 'var(--color-text)'};">
                {branch.name}
              </span>
              {#if branch.is_remote}
                <span style="font-size:9px;color:var(--color-text);opacity:0.4;">remote</span>
              {/if}
              {#if branch.is_head}
                <span style="font-size:9px;color:var(--color-primary);">HEAD</span>
              {/if}
            </div>
          {/each}
        </div>
      {/if}
    </div>
  {/if}
</div>
