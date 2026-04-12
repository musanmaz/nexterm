<script lang="ts">
  import type { FileDiff } from '$lib/types';

  let { diffs = [] }: { diffs: FileDiff[] } = $props();
</script>

<div class="space-y-2">
  {#each diffs as file}
    <div class="sci-fi-panel">
      <div class="panel-title flex items-center gap-2">
        <span class="text-[var(--color-text-bright)]">{file.path}</span>
        <span class="text-[var(--color-success)] text-[9px]">+{file.additions}</span>
        <span class="text-[var(--color-error)] text-[9px]">-{file.deletions}</span>
      </div>
      <div class="p-1 font-mono text-[10px] leading-4 overflow-x-auto">
        {#each file.hunks as hunk}
          <div class="text-[var(--color-text)] opacity-40 bg-[var(--color-bg-secondary)] px-2 py-0.5">{hunk.header.trim()}</div>
          {#each hunk.lines as line}
            <div
              class="px-2"
              class:bg-green-900={line.origin === '+'}
              class:bg-opacity-20={line.origin === '+' || line.origin === '-'}
              class:bg-red-900={line.origin === '-'}
              class:text-[var(--color-success)]={line.origin === '+'}
              class:text-[var(--color-error)]={line.origin === '-'}
              class:text-[var(--color-text)]={line.origin !== '+' && line.origin !== '-'}
            >
              <span class="inline-block w-3 opacity-50">{line.origin}</span>{line.content}
            </div>
          {/each}
        {/each}
      </div>
    </div>
  {/each}
  {#if diffs.length === 0}
    <div class="text-center text-[10px] text-[var(--color-text)] opacity-50 py-4">No changes</div>
  {/if}
</div>
