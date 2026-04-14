<script lang="ts">
  import { sendChatWithProvider, sendChatMessage, type AIChatConfig, type AIContext } from '$lib/utils/ai-chat';
  import type { ChatMessage, AIProvider, ChatSession, ChatMode } from '$lib/types';

  let { provider = 'ollama', model = 'llama3', apiKey = '',
        litellmBaseUrl = 'http://localhost:4000', litellmApiKey = '', litellmModel = 'gpt-4',
        providers = [], activeProviderId = '',
        cwd = '', osInfo = '', recentCommands = [],
        oncommand, onchangeprovider, onchangemodel }: {
    provider?: string;
    model?: string;
    apiKey?: string;
    litellmBaseUrl?: string;
    litellmApiKey?: string;
    litellmModel?: string;
    providers?: AIProvider[];
    activeProviderId?: string;
    cwd?: string;
    osInfo?: string;
    recentCommands?: string[];
    oncommand?: (cmd: string) => void;
    onchangeprovider?: (id: string) => void;
    onchangemodel?: (providerId: string, model: string) => void;
  } = $props();

  const aiContext = $derived<AIContext>({ cwd, os: osInfo, recentCommands });

  const MODES: { id: ChatMode; icon: string; label: string; desc: string; color: string }[] = [
    { id: 'chat', icon: '💬', label: 'CHAT', desc: 'General conversation', color: 'var(--color-primary)' },
    { id: 'plan', icon: '📋', label: 'PLAN', desc: 'Planning and architecture', color: 'var(--color-warning)' },
    { id: 'agent', icon: '🤖', label: 'AGENT', desc: 'Command execution', color: 'var(--color-success)' },
    { id: 'ask', icon: '❓', label: 'ASK', desc: 'Question and answer', color: 'var(--color-accent)' },
  ];

  let sessions = $state<ChatSession[]>([]);
  let activeSessionId = $state('');
  let input = $state('');
  let loading = $state(false);
  let expanded = $state(false);
  let showProviderPicker = $state(false);
  let showSessionList = $state(false);
  let showModelPicker = $state(false);
  let scrollContainer = $state<HTMLDivElement | undefined>(undefined);

  const activeProvider = $derived(providers.find(p => p.id === activeProviderId));
  const activeSession = $derived(sessions.find(s => s.id === activeSessionId));
  const messages = $derived(activeSession?.messages || []);
  const currentMode = $derived(activeSession?.mode || 'chat');
  const modeInfo = $derived(MODES.find(m => m.id === currentMode) || MODES[0]);

  function uid(): string {
    return crypto.randomUUID?.() || Math.random().toString(36).slice(2);
  }

  function createSession(mode: ChatMode = 'chat') {
    const mInfo = MODES.find(m => m.id === mode) || MODES[0];
    const s: ChatSession = {
      id: uid(),
      title: `${mInfo.label} ${sessions.filter(x => x.mode === mode).length + 1}`,
      mode,
      messages: [],
      createdAt: Date.now(),
    };
    sessions = [...sessions, s];
    activeSessionId = s.id;
    showSessionList = false;
    saveSessions();
  }

  function switchSession(id: string) {
    activeSessionId = id;
    showSessionList = false;
    scrollToBottom();
  }

  function deleteSession(id: string) {
    sessions = sessions.filter(s => s.id !== id);
    if (activeSessionId === id) {
      activeSessionId = sessions[sessions.length - 1]?.id || '';
    }
    saveSessions();
  }

  function changeMode(mode: ChatMode) {
    if (!activeSession) { createSession(mode); return; }
    sessions = sessions.map(s => s.id === activeSessionId ? { ...s, mode } : s);
    saveSessions();
  }

  // Init: load from localStorage or create default
  function initSessions() {
    try {
      const stored = localStorage.getItem('nexterm-ai-sessions');
      if (stored) {
        const parsed = JSON.parse(stored);
        if (Array.isArray(parsed) && parsed.length > 0) {
          sessions = parsed;
          activeSessionId = sessions[sessions.length - 1].id;
          return;
        }
      }
    } catch { /* ignore */ }
    createSession('chat');
  }

  function saveSessions() {
    try {
      const toSave = sessions.map(s => ({
        ...s,
        messages: s.messages.slice(-50),
      }));
      localStorage.setItem('nexterm-ai-sessions', JSON.stringify(toSave));
    } catch { /* ignore */ }
  }

  // Run once
  initSessions();

  function scrollToBottom() {
    requestAnimationFrame(() => {
      if (scrollContainer) scrollContainer.scrollTop = scrollContainer.scrollHeight;
    });
  }

  async function send() {
    const text = input.trim();
    if (!text || loading || !activeSession) return;

    const userMsg: ChatMessage = { role: 'user', content: text, timestamp: Date.now() };
    sessions = sessions.map(s => s.id === activeSessionId
      ? { ...s, messages: [...s.messages, userMsg], title: s.messages.length === 0 ? text.slice(0, 30) : s.title }
      : s
    );
    input = '';
    loading = true;
    scrollToBottom();

    try {
      let reply: string;
      const msgs = sessions.find(s => s.id === activeSessionId)!.messages;
      const mode = activeSession.mode;
      if (activeProvider) {
        reply = await sendChatWithProvider(msgs, activeProvider, mode, aiContext);
      } else {
        const config: AIChatConfig = { provider, model, apiKey, litellmBaseUrl, litellmApiKey, litellmModel };
        reply = await sendChatMessage(msgs, config, mode, aiContext);
      }
      const assistantMsg: ChatMessage = { role: 'assistant', content: reply, timestamp: Date.now() };
      sessions = sessions.map(s => s.id === activeSessionId ? { ...s, messages: [...s.messages, assistantMsg] } : s);
    } catch (e) {
      const errMsg: ChatMessage = { role: 'assistant', content: `⚠ Error: ${e}`, timestamp: Date.now() };
      sessions = sessions.map(s => s.id === activeSessionId ? { ...s, messages: [...s.messages, errMsg] } : s);
    } finally {
      loading = false;
      scrollToBottom();
      saveSessions();
    }
  }

  function handleKey(e: KeyboardEvent) {
    if (e.key === 'Enter' && !e.shiftKey) { e.preventDefault(); send(); }
  }

  function switchProvider(id: string) {
    onchangeprovider?.(id);
    showProviderPicker = false;
  }

  function switchModel(m: string) {
    if (activeProvider) {
      onchangemodel?.(activeProvider.id, m);
    }
    showModelPicker = false;
  }

  function extractCommands(content: string): string[] {
    const cmds: string[] = [];
    const re = /```(?:bash|sh|shell|zsh)?\n([\s\S]*?)```/g;
    let m;
    while ((m = re.exec(content)) !== null) {
      for (const line of m[1].trim().split('\n')) {
        const clean = line.replace(/^\$\s*/, '').trim();
        if (clean && !clean.startsWith('#')) cmds.push(clean);
      }
    }
    return cmds;
  }

  function formatContent(content: string): string {
    let html = content.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
    html = html.replace(/```(?:bash|sh|shell|zsh|typescript|javascript|python|rust|json|yaml|sql|css|html|text|diff|dockerfile|toml)?\n([\s\S]*?)```/g,
      (_m, code) => `<pre style="background:var(--color-bg-primary);border:1px solid var(--color-border);padding:8px;margin:4px 0;border-radius:2px;overflow-x:auto;font-size:11px;line-height:1.4;">${code}</pre>`
    );
    html = html.replace(/`([^`]+)`/g, '<code style="background:var(--color-bg-primary);padding:1px 4px;border-radius:2px;font-size:11px;">$1</code>');
    html = html.replace(/\*\*([^*]+)\*\*/g, '<strong>$1</strong>');
    html = html.replace(/(?<!\*)\*([^*]+)\*(?!\*)/g, '<em>$1</em>');
    html = html.replace(/\n/g, '<br>');
    return html;
  }

  function timeStr(ts: number): string {
    return new Date(ts).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
  }
</script>

<div style="border-top:1px solid var(--color-border);background:var(--color-bg-secondary);display:flex;flex-direction:column;{expanded ? 'flex:1;min-height:200px;max-height:50vh;' : ''}position:relative;">
  <!-- Header Bar -->
  <div style="display:flex;align-items:center;height:28px;">
    <!-- Expand/Collapse -->
    <button type="button" style="display:flex;align-items:center;gap:6px;padding:0 10px;height:100%;font-size:10px;letter-spacing:2px;color:var(--color-text);background:transparent;border:none;cursor:pointer;"
      onclick={() => { expanded = !expanded; if (expanded) scrollToBottom(); }}
    >
      <span style="color:{modeInfo.color};font-weight:bold;">AI</span>
      <span>{modeInfo.label}</span>
      <span style="font-size:10px;">{expanded ? '▼' : '▲'}</span>
    </button>

    <!-- Session Switcher -->
    <button type="button" style="display:flex;align-items:center;gap:4px;padding:0 8px;height:100%;font-size:9px;color:var(--color-text);background:transparent;border:none;border-left:1px solid var(--color-border);cursor:pointer;max-width:150px;overflow:hidden;white-space:nowrap;text-overflow:ellipsis;"
      onclick={() => { showSessionList = !showSessionList; showProviderPicker = false; }}
      title={activeSession?.title || 'No session'}
    >
      {activeSession?.title || '—'}
      <span style="opacity:0.3;font-size:8px;flex-shrink:0;">({sessions.length})</span>
    </button>

    <!-- Mode Buttons -->
    <div style="display:flex;height:100%;border-left:1px solid var(--color-border);">
      {#each MODES as m}
        <button type="button" style="padding:0 6px;height:100%;font-size:8px;border:none;cursor:pointer;letter-spacing:1px;background:{currentMode === m.id ? m.color : 'transparent'};color:{currentMode === m.id ? 'var(--color-bg-primary)' : 'var(--color-text)'};opacity:{currentMode === m.id ? '1' : '0.5'};"
          onclick={() => changeMode(m.id)}
          title={m.desc}
        >{m.icon}</button>
      {/each}
    </div>

    <div style="flex:1;"></div>

    <!-- Provider Switcher -->
    {#if providers.length > 0}
      <button type="button" style="display:flex;align-items:center;gap:4px;padding:0 8px;height:100%;font-size:8px;color:var(--color-primary);background:transparent;border:none;border-left:1px solid var(--color-border);cursor:pointer;white-space:nowrap;"
        onclick={() => { showProviderPicker = !showProviderPicker; showSessionList = false; showModelPicker = false; }}
      >
        <span style="width:5px;height:5px;border-radius:50%;background:var(--color-success);flex-shrink:0;"></span>
        {activeProvider?.name || '?'}
      </button>
      <!-- Model Switcher -->
      {#if activeProvider && activeProvider.models.length > 0}
        <button type="button" style="display:flex;align-items:center;gap:3px;padding:0 8px;height:100%;font-size:8px;color:var(--color-text-bright);background:transparent;border:none;border-left:1px solid var(--color-border);cursor:pointer;white-space:nowrap;max-width:140px;overflow:hidden;text-overflow:ellipsis;"
          onclick={() => { showModelPicker = !showModelPicker; showProviderPicker = false; showSessionList = false; }}
          title={activeProvider.model}
        >
          {activeProvider.model || '?'}
          <span style="font-size:7px;opacity:0.4;">▼</span>
        </button>
      {/if}
    {/if}
  </div>

  <!-- Session List Dropdown -->
  {#if showSessionList}
    <div style="position:absolute;left:60px;top:28px;z-index:100;background:var(--color-bg-secondary);border:1px solid var(--color-border);min-width:260px;max-height:300px;overflow-y:auto;box-shadow:0 4px 12px rgba(0,0,0,0.5);">
      <div style="padding:6px 10px;font-size:9px;color:var(--color-text);opacity:0.5;letter-spacing:2px;border-bottom:1px solid var(--color-border);display:flex;justify-content:space-between;align-items:center;">
        <span>SESSIONS</span>
        <div style="display:flex;gap:4px;">
          {#each MODES as m}
            <button type="button" style="padding:2px 6px;font-size:8px;background:{m.color};color:var(--color-bg-primary);border:none;cursor:pointer;letter-spacing:1px;"
              onclick={() => createSession(m.id)}
              title="New {m.label}"
            >+ {m.icon}</button>
          {/each}
        </div>
      </div>
      {#each [...sessions].reverse() as sess}
        <div style="display:flex;align-items:center;gap:6px;padding:6px 10px;border-bottom:1px solid var(--color-border);background:{sess.id === activeSessionId ? 'var(--color-bg-panel)' : 'transparent'};">
          <button type="button" style="flex:1;display:flex;align-items:center;gap:6px;background:none;border:none;cursor:pointer;text-align:left;padding:0;min-width:0;"
            onclick={() => switchSession(sess.id)}
          >
            <span style="font-size:10px;">{MODES.find(m => m.id === sess.mode)?.icon || '💬'}</span>
            <div style="flex:1;min-width:0;">
              <div style="font-size:10px;color:var(--color-text-bright);overflow:hidden;text-overflow:ellipsis;white-space:nowrap;">{sess.title}</div>
              <div style="font-size:8px;color:var(--color-text);opacity:0.4;">{sess.messages.length} msg · {new Date(sess.createdAt).toLocaleDateString()}</div>
            </div>
          </button>
          {#if sessions.length > 1}
            <button type="button" style="padding:2px 6px;font-size:8px;background:var(--color-error);color:#fff;border:none;cursor:pointer;flex-shrink:0;"
              onclick={() => deleteSession(sess.id)}
            >✕</button>
          {/if}
        </div>
      {/each}
    </div>
  {/if}

  <!-- Provider Picker Dropdown -->
  {#if showProviderPicker && providers.length > 0}
    <div style="position:absolute;right:0;top:28px;z-index:100;background:var(--color-bg-secondary);border:1px solid var(--color-border);min-width:220px;box-shadow:0 4px 12px rgba(0,0,0,0.5);">
      <div style="padding:6px 10px;font-size:9px;color:var(--color-text);opacity:0.5;letter-spacing:2px;border-bottom:1px solid var(--color-border);">PROVIDER</div>
      {#each providers as prov}
        <button type="button" style="width:100%;display:flex;align-items:center;gap:6px;padding:6px 10px;border:none;cursor:pointer;text-align:left;background:{prov.id === activeProviderId ? 'var(--color-bg-panel)' : 'transparent'};color:var(--color-text);"
          onclick={() => switchProvider(prov.id)}
        >
          <span style="width:6px;height:6px;border-radius:50%;flex-shrink:0;background:{prov.id === activeProviderId ? 'var(--color-success)' : 'var(--color-border)'};"></span>
          <div style="flex:1;min-width:0;">
            <div style="font-size:10px;color:var(--color-text-bright);">{prov.name}</div>
            <div style="font-size:8px;opacity:0.4;">{prov.model}</div>
          </div>
          {#if prov.id === activeProviderId}<span style="font-size:9px;color:var(--color-success);">✓</span>{/if}
        </button>
      {/each}
    </div>
  {/if}

  <!-- Model Picker Dropdown -->
  {#if showModelPicker && activeProvider && activeProvider.models.length > 0}
    <div style="position:absolute;right:0;top:28px;z-index:100;background:var(--color-bg-secondary);border:1px solid var(--color-border);min-width:220px;max-height:300px;overflow-y:auto;box-shadow:0 4px 12px rgba(0,0,0,0.5);">
      <div style="padding:6px 10px;font-size:9px;color:var(--color-text);opacity:0.5;letter-spacing:2px;border-bottom:1px solid var(--color-border);">MODEL ({activeProvider.models.length})</div>
      {#each activeProvider.models as m}
        <button type="button" style="width:100%;display:flex;align-items:center;gap:6px;padding:6px 10px;border:none;cursor:pointer;text-align:left;background:{m === activeProvider.model ? 'var(--color-bg-panel)' : 'transparent'};color:var(--color-text);"
          onclick={() => switchModel(m)}
        >
          <span style="width:6px;height:6px;border-radius:50%;flex-shrink:0;background:{m === activeProvider.model ? 'var(--color-primary)' : 'var(--color-border)'};"></span>
          <span style="font-size:10px;color:{m === activeProvider.model ? 'var(--color-text-bright)' : 'var(--color-text)'};overflow:hidden;text-overflow:ellipsis;white-space:nowrap;">{m}</span>
          {#if m === activeProvider.model}<span style="font-size:9px;color:var(--color-primary);margin-left:auto;flex-shrink:0;">✓</span>{/if}
        </button>
      {/each}
    </div>
  {/if}

  {#if expanded}
    <!-- Mode Banner + Context -->
    <div style="display:flex;align-items:center;gap:6px;padding:4px 12px;background:{modeInfo.color};opacity:0.15;">
      <span style="font-size:9px;letter-spacing:2px;color:var(--color-text-bright);">{modeInfo.icon} {modeInfo.label} MODE — {modeInfo.desc}</span>
      <div style="flex:1;"></div>
      {#if cwd}
        <span style="font-size:8px;color:var(--color-text);opacity:0.6;font-family:var(--font-mono);">{cwd}</span>
      {/if}
    </div>

    <!-- Messages -->
    <div bind:this={scrollContainer} style="flex:1;overflow-y:auto;padding:8px 12px;display:flex;flex-direction:column;gap:8px;">
      {#if messages.length === 0}
        <div style="text-align:center;padding:20px;opacity:0.4;">
          <div style="font-size:24px;margin-bottom:6px;">{modeInfo.icon}</div>
          <div style="font-size:11px;color:var(--color-text);">{modeInfo.desc}</div>
          <div style="font-size:8px;color:var(--color-text);margin-top:6px;opacity:0.6;">
            {#if currentMode === 'plan'}Ask me to create a plan for a project or task
            {:else if currentMode === 'agent'}Describe the task and I will prepare actionable commands
            {:else if currentMode === 'ask'}Ask a technical question for a detailed explanation
            {:else}Ask about commands, errors, and system administration
            {/if}
          </div>
        </div>
      {/if}

      {#each messages as msg}
        <div style="display:flex;flex-direction:column;{msg.role === 'user' ? 'align-items:flex-end;' : 'align-items:flex-start;'}">
          <div style="
            max-width:85%;padding:8px 12px;border-radius:4px;font-size:12px;line-height:1.5;
            font-family:inherit;word-break:break-word;
            {msg.role === 'user'
              ? `background:${modeInfo.color};color:var(--color-bg-primary);`
              : 'background:var(--color-bg-primary);color:var(--color-text-bright);border:1px solid var(--color-border);'}
          ">
            {#if msg.role === 'assistant'}
              {@html formatContent(msg.content)}
              {#if extractCommands(msg.content).length > 0}
                <div style="margin-top:8px;display:flex;flex-wrap:wrap;gap:4px;">
                  {#each extractCommands(msg.content) as cmd}
                    <button type="button" style="padding:3px 8px;font-size:9px;background:var(--color-success);color:#000;border:none;cursor:pointer;letter-spacing:1px;border-radius:2px;" onclick={() => oncommand?.(cmd)} title="Run: {cmd}">▶ RUN</button>
                  {/each}
                </div>
              {/if}
            {:else}
              {msg.content}
            {/if}
          </div>
          <span style="font-size:8px;color:var(--color-text);opacity:0.3;margin-top:2px;padding:0 4px;">{timeStr(msg.timestamp)}</span>
        </div>
      {/each}

      {#if loading}
        <div style="display:flex;align-items:flex-start;">
          <div style="padding:8px 12px;background:var(--color-bg-primary);border:1px solid var(--color-border);border-radius:4px;font-size:11px;color:{modeInfo.color};">
            <span class="typing-dots">●●●</span>
          </div>
        </div>
      {/if}
    </div>

    <!-- Input -->
    <div style="display:flex;gap:4px;padding:6px 10px;border-top:1px solid var(--color-border);align-items:center;">
      <button type="button" style="padding:3px 6px;font-size:9px;background:transparent;color:var(--color-text);border:1px solid var(--color-border);cursor:pointer;opacity:0.4;flex-shrink:0;"
        onclick={() => createSession(currentMode)} title="New session">+</button>
      <input
        bind:value={input}
        onkeydown={handleKey}
        placeholder={currentMode === 'plan' ? 'Describe the plan...' : currentMode === 'agent' ? 'Describe the task...' : currentMode === 'ask' ? 'Type your question...' : 'Type your message...'}
        disabled={loading}
        style="flex:1;background:var(--color-bg-primary);border:1px solid var(--color-border);color:var(--color-text-bright);font-size:12px;padding:5px 10px;font-family:inherit;outline:none;box-sizing:border-box;"
      />
      <button type="button"
        style="padding:5px 12px;font-size:10px;background:{modeInfo.color};color:var(--color-bg-primary);border:none;cursor:pointer;letter-spacing:1px;flex-shrink:0;{loading ? 'opacity:0.4;' : ''}"
        onclick={send}
        disabled={loading}
      >{loading ? '...' : 'SEND'}</button>
    </div>
  {/if}
</div>

<style>
  @keyframes blink {
    0%, 20% { opacity: 1; }
    50% { opacity: 0.2; }
    80%, 100% { opacity: 1; }
  }
  .typing-dots {
    animation: blink 1.2s infinite;
    letter-spacing: 2px;
  }
</style>
