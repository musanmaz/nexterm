<script lang="ts">
  import type { GitCommit } from '$lib/types';

  let { commits = [] }: { commits: GitCommit[] } = $props();

  function timeAgo(timestamp: number): string {
    const seconds = Math.floor(Date.now() / 1000 - timestamp);
    if (seconds < 60) return 'just now';
    if (seconds < 3600) return Math.floor(seconds / 60) + 'm ago';
    if (seconds < 86400) return Math.floor(seconds / 3600) + 'h ago';
    if (seconds < 604800) return Math.floor(seconds / 86400) + 'd ago';
    return new Date(timestamp * 1000).toLocaleDateString();
  }
</script>

<div class="space-y-0">
  {#each commits as commit, i}
    <div class="flex gap-2 py-1 hover:bg-[var(--color-border)] transition-colors px-1">
      <div class="flex flex-col items-center w-4 flex-shrink-0">
        <div class="w-2 h-2 rounded-full bg-[var(--color-primary)] flex-shrink-0 mt-1"></div>
        {#if i < commits.length - 1}
          <div class="w-px flex-1 bg-[var(--color-border)]"></div>
        {/if}
      </div>
      <div class="flex-1 min-w-0">
        <div class="text-[10px] text-[var(--color-text-bright)] truncate">{commit.message.split('\n')[0]}</div>
        <div class="flex gap-2 text-[9px] text-[var(--color-text)] opacity-50">
          <span class="text-[var(--color-primary)]">{commit.short_id}</span>
          <span>{commit.author}</span>
          <span>{timeAgo(commit.time)}</span>
        </div>
      </div>
    </div>
  {/each}
  {#if commits.length === 0}
    <div class="text-center text-[10px] text-[var(--color-text)] opacity-50 py-4">No commits</div>
  {/if}
</div>
