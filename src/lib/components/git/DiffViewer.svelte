<script lang="ts">
  import type { FileDiff } from '$lib/types';

  let { diffs = [] }: { diffs: FileDiff[] } = $props();

  function lineStyle(origin: string): string {
    if (origin === '+') return 'padding:0 8px;color:var(--color-success);background:rgba(0,255,0,0.07);';
    if (origin === '-') return 'padding:0 8px;color:var(--color-error);background:rgba(255,0,0,0.07);';
    return 'padding:0 8px;color:var(--color-text);';
  }
</script>

<div style="display:flex;flex-direction:column;gap:8px;">
  {#each diffs as file}
    <div style="border:1px solid var(--color-border);background:var(--color-bg-secondary);">
      <div style="display:flex;align-items:center;gap:8px;padding:6px 8px;border-bottom:1px solid var(--color-border);">
        <span style="font-size:10px;color:var(--color-text-bright);font-weight:bold;">{file.path}</span>
        <span style="font-size:9px;color:var(--color-success);">+{file.additions}</span>
        <span style="font-size:9px;color:var(--color-error);">-{file.deletions}</span>
      </div>
      <div style="padding:4px 0;font-family:var(--font-mono);font-size:10px;line-height:1.6;overflow-x:auto;">
        {#each file.hunks as hunk}
          <div style="color:var(--color-text);opacity:0.4;background:var(--color-bg-primary);padding:2px 8px;font-size:9px;">{hunk.header.trim()}</div>
          {#each hunk.lines as line}
            <div style={lineStyle(line.origin)}>
              <span style="display:inline-block;width:12px;opacity:0.5;text-align:center;">{line.origin}</span>{line.content}
            </div>
          {/each}
        {/each}
      </div>
    </div>
  {/each}
  {#if diffs.length === 0}
    <div style="text-align:center;font-size:10px;color:var(--color-text);opacity:0.5;padding:16px 0;">No changes</div>
  {/if}
</div>
