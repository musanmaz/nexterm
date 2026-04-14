<script lang="ts">
  import type { Theme } from '$lib/types';

  let { open = $bindable(false), theme, onsave }: {
    open: boolean;
    theme: Theme | null;
    onsave?: (theme: Theme) => void;
  } = $props();

  let editedColors = $state<Record<string, string>>({});
  let editedTerminal = $state<Record<string, string>>({});
  let customName = $state('');
  let activeSection = $state<'ui' | 'terminal'>('ui');

  $effect(() => {
    if (open && theme) {
      editedColors = { ...theme.colors };
      editedTerminal = { ...theme.terminal };
      customName = theme.name + ' (Custom)';
    }
  });

  function handleSave() {
    if (!theme) return;
    const newTheme: Theme = {
      name: customName || theme.name,
      colors: editedColors as Theme['colors'],
      terminal: editedTerminal as Theme['terminal'],
      globe: theme.globe,
    };
    onsave?.(newTheme);
    open = false;
  }

  const colorLabels: Record<string, string> = {
    primary: 'Primary', secondary: 'Secondary', accent: 'Accent',
    bgPrimary: 'BG Primary', bgSecondary: 'BG Secondary', bgPanel: 'BG Panel',
    text: 'Text', textBright: 'Text Bright', border: 'Border',
    borderBright: 'Border Bright', success: 'Success', warning: 'Warning', error: 'Error',
  };

  const termLabels: Record<string, string> = {
    background: 'Background', foreground: 'Foreground', cursor: 'Cursor',
    black: 'Black', red: 'Red', green: 'Green', yellow: 'Yellow',
    blue: 'Blue', magenta: 'Magenta', cyan: 'Cyan', white: 'White',
    brightBlack: 'Bright Black', brightRed: 'Bright Red', brightGreen: 'Bright Green',
    brightYellow: 'Bright Yellow', brightBlue: 'Bright Blue', brightMagenta: 'Bright Magenta',
    brightCyan: 'Bright Cyan', brightWhite: 'Bright White',
  };
</script>

{#if open}
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    style="position:fixed;inset:0;background:rgba(0,0,0,0.7);z-index:10000;display:flex;align-items:center;justify-content:center;"
    onclick={() => open = false}
  >
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      style="width:540px;max-height:80vh;background:var(--color-bg-panel);border:1px solid var(--color-border);display:flex;flex-direction:column;"
      onclick={(e) => e.stopPropagation()}
    >
      <!-- Header -->
      <div style="display:flex;align-items:center;justify-content:space-between;padding:10px 14px;border-bottom:1px solid var(--color-border);">
        <span style="font-size:11px;letter-spacing:2px;color:var(--color-primary);">THEME EDITOR</span>
        <button type="button" style="background:none;border:none;color:var(--color-text);font-size:14px;cursor:pointer;opacity:0.5;" onclick={() => open = false}>✕</button>
      </div>

      <!-- Name -->
      <div style="padding:8px 14px;border-bottom:1px solid var(--color-border);">
        <input
          bind:value={customName}
          placeholder="Theme name"
          style="width:100%;background:var(--color-bg-primary);border:1px solid var(--color-border);color:var(--color-text-bright);font-size:11px;padding:5px 8px;outline:none;"
        />
      </div>

      <!-- Section Tabs -->
      <div style="display:flex;border-bottom:1px solid var(--color-border);">
        {#each [['ui', 'UI COLORS'], ['terminal', 'TERMINAL COLORS']] as [sec, label]}
          <button
            type="button"
            style="
              flex:1;padding:6px;font-size:10px;letter-spacing:1px;cursor:pointer;border:none;
              background:{activeSection === sec ? 'var(--color-primary)' : 'transparent'};
              color:{activeSection === sec ? 'var(--color-bg-primary)' : 'var(--color-text)'};
            "
            onclick={() => activeSection = sec as typeof activeSection}
          >{label}</button>
        {/each}
      </div>

      <!-- Color Grid -->
      <div style="flex:1;overflow-y:auto;padding:10px 14px;">
        {#if activeSection === 'ui'}
          <div style="display:grid;grid-template-columns:1fr 1fr;gap:6px;">
            {#each Object.entries(editedColors) as [key, value]}
              <div style="display:flex;align-items:center;gap:6px;">
                <input
                  type="color"
                  value={value}
                  oninput={(e) => editedColors[key] = (e.target as HTMLInputElement).value}
                  style="width:24px;height:24px;border:1px solid var(--color-border);cursor:pointer;background:none;padding:0;"
                />
                <div style="flex:1;min-width:0;">
                  <div style="font-size:9px;color:var(--color-text);opacity:0.7;">{colorLabels[key] || key}</div>
                  <div style="font-size:8px;color:var(--color-text);opacity:0.4;font-family:var(--font-mono);">{value}</div>
                </div>
              </div>
            {/each}
          </div>
        {:else}
          <div style="display:grid;grid-template-columns:1fr 1fr;gap:6px;">
            {#each Object.entries(editedTerminal).filter(([k]) => k !== 'selectionBackground') as [key, value]}
              <div style="display:flex;align-items:center;gap:6px;">
                <input
                  type="color"
                  value={value}
                  oninput={(e) => editedTerminal[key] = (e.target as HTMLInputElement).value}
                  style="width:24px;height:24px;border:1px solid var(--color-border);cursor:pointer;background:none;padding:0;"
                />
                <div style="flex:1;min-width:0;">
                  <div style="font-size:9px;color:var(--color-text);opacity:0.7;">{termLabels[key] || key}</div>
                  <div style="font-size:8px;color:var(--color-text);opacity:0.4;font-family:var(--font-mono);">{value}</div>
                </div>
              </div>
            {/each}
          </div>

          <!-- Terminal Preview -->
          <div style="margin-top:12px;padding:8px;border:1px solid var(--color-border);background:{editedTerminal.background};font-family:var(--font-mono);font-size:11px;line-height:1.5;border-radius:2px;">
            <span style="color:{editedTerminal.green};">user</span><span style="color:{editedTerminal.foreground};">@</span><span style="color:{editedTerminal.cyan};">host</span>
            <span style="color:{editedTerminal.foreground};">:</span><span style="color:{editedTerminal.blue};">~/project</span><span style="color:{editedTerminal.foreground};">$</span>
            <span style="color:{editedTerminal.foreground};"> npm run build</span><br/>
            <span style="color:{editedTerminal.green};">✓</span> <span style="color:{editedTerminal.foreground};">Build completed</span><br/>
            <span style="color:{editedTerminal.red};">✗</span> <span style="color:{editedTerminal.yellow};">2 warnings</span>
            <span style="color:{editedTerminal.magenta};"> [lint]</span>
            <span style="color:{editedTerminal.brightBlack};"> 3.2s</span>
          </div>
        {/if}

        <!-- Live Preview Bar -->
        <div style="margin-top:12px;border:1px solid var(--color-border);padding:6px;display:flex;gap:4px;flex-wrap:wrap;">
          <span style="font-size:8px;opacity:0.5;width:100%;margin-bottom:2px;">PREVIEW</span>
          {#each Object.entries(editedColors) as [key, value]}
            <div style="width:16px;height:16px;background:{value};border:1px solid rgba(255,255,255,0.1);" title="{colorLabels[key] || key}: {value}"></div>
          {/each}
        </div>
      </div>

      <!-- Footer -->
      <div style="display:flex;gap:8px;padding:10px 14px;border-top:1px solid var(--color-border);justify-content:flex-end;">
        <button
          type="button"
          style="padding:5px 16px;font-size:10px;background:transparent;color:var(--color-text);border:1px solid var(--color-border);cursor:pointer;letter-spacing:1px;"
          onclick={() => open = false}
        >CANCEL</button>
        <button
          type="button"
          style="padding:5px 16px;font-size:10px;background:var(--color-primary);color:var(--color-bg-primary);border:none;cursor:pointer;letter-spacing:1px;"
          onclick={handleSave}
        >SAVE THEME</button>
      </div>
    </div>
  </div>
{/if}
