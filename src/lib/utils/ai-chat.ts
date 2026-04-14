import type { ChatMessage, AIProvider } from '$lib/types';

export interface AIChatConfig {
  provider: string;
  model: string;
  apiKey: string;
  litellmBaseUrl: string;
  litellmApiKey: string;
  litellmModel: string;
}

function stripV1(url: string): string {
  return url.replace(/\/+$/, '').replace(/\/v1$/i, '');
}

function buildEndpointFromProvider(p: AIProvider): { url: string; headers: Record<string, string>; model: string } {
  const root = stripV1(p.baseUrl);

  if (p.type === 'anthropic') {
    return {
      url: `${root}/v1/messages`,
      headers: {
        'Content-Type': 'application/json',
        'x-api-key': p.apiKey,
        'anthropic-version': '2023-06-01',
        'anthropic-dangerous-direct-browser-access': 'true',
      },
      model: p.model || 'claude-sonnet-4-20250514',
    };
  }

  if (p.type === 'ollama') {
    return {
      url: `${root}/api/chat`,
      headers: { 'Content-Type': 'application/json' },
      model: p.model || 'llama3',
    };
  }

  // openai, litellm, openai-compatible → all use /v1/chat/completions
  const headers: Record<string, string> = { 'Content-Type': 'application/json' };
  if (p.apiKey) {
    headers['Authorization'] = `Bearer ${p.apiKey}`;
    if (p.type === 'litellm' || p.type === 'openai-compatible') {
      headers['x-litellm-api-key'] = p.apiKey;
    }
  }
  return {
    url: `${root}/v1/chat/completions`,
    headers,
    model: p.model || 'gpt-4',
  };
}

function buildEndpointLegacy(config: AIChatConfig): { url: string; headers: Record<string, string>; model: string } {
  if (config.provider === 'litellm') {
    const root = stripV1(config.litellmBaseUrl);
    const lHeaders: Record<string, string> = { 'Content-Type': 'application/json' };
    if (config.litellmApiKey) {
      lHeaders['Authorization'] = `Bearer ${config.litellmApiKey}`;
      lHeaders['x-litellm-api-key'] = config.litellmApiKey;
    }
    return {
      url: `${root}/v1/chat/completions`,
      headers: lHeaders,
      model: config.litellmModel || 'gpt-4',
    };
  }
  if (config.provider === 'openai') {
    return {
      url: 'https://api.openai.com/v1/chat/completions',
      headers: { 'Content-Type': 'application/json', 'Authorization': `Bearer ${config.apiKey}` },
      model: config.model || 'gpt-4',
    };
  }
  if (config.provider === 'anthropic') {
    return {
      url: 'https://api.anthropic.com/v1/messages',
      headers: {
        'Content-Type': 'application/json',
        'x-api-key': config.apiKey,
        'anthropic-version': '2023-06-01',
        'anthropic-dangerous-direct-browser-access': 'true',
      },
      model: config.model || 'claude-sonnet-4-20250514',
    };
  }
  return {
    url: 'http://localhost:11434/api/chat',
    headers: { 'Content-Type': 'application/json' },
    model: config.model || 'llama3',
  };
}

import type { ChatMode } from '$lib/types';

const SYSTEM_PROMPTS: Record<string, string> = {
  chat: `You are a helpful terminal AI assistant embedded in NEXTERM, an AI-powered terminal emulator and DevOps command center. You help users with shell commands, Docker, Kubernetes, Git, system administration, programming, and debugging. Keep answers concise and practical. When suggesting commands, wrap them in \`\`\`bash code blocks. Respond in English by default unless the user explicitly requests another language.`,

  plan: `You are a strategic planning AI assistant. Help the user design implementation plans, architecture decisions, and workflows step by step. Structure your responses with clear numbered steps, pros/cons when relevant, and actionable items. Use markdown formatting with headers and lists. When commands are needed, include them in \`\`\`bash code blocks. Respond in English by default unless the user explicitly requests another language.`,

  agent: `You are an AI agent that helps execute tasks. When the user describes a task, provide the exact commands to run, one step at a time. Focus on actionable shell commands wrapped in \`\`\`bash code blocks. After each command, briefly explain what it does. Be direct and efficient. If something could go wrong, warn the user. Respond in English by default unless the user explicitly requests another language.`,

  ask: `You are a knowledgeable AI assistant for answering technical questions. Explain concepts clearly with examples. You focus on understanding and education rather than execution. Use analogies when helpful. Include code examples when relevant. Respond in English by default unless the user explicitly requests another language.`,
};

export interface AIContext {
  cwd?: string;
  os?: string;
  recentCommands?: string[];
}

function getSystemPrompt(mode?: ChatMode, context?: AIContext): string {
  let prompt = SYSTEM_PROMPTS[mode || 'chat'] || SYSTEM_PROMPTS.chat;
  if (context) {
    const parts: string[] = [];
    if (context.os) parts.push(`OS: ${context.os}`);
    if (context.cwd) parts.push(`Working directory: ${context.cwd}`);
    if (context.recentCommands && context.recentCommands.length > 0) {
      parts.push(`Recent commands: ${context.recentCommands.slice(0, 10).join(', ')}`);
    }
    if (parts.length > 0) {
      prompt += `\n\nCurrent environment:\n${parts.join('\n')}`;
    }
  }
  return prompt;
}

export async function sendChatMessage(
  messages: ChatMessage[],
  config: AIChatConfig,
  mode?: ChatMode,
  context?: AIContext,
): Promise<string> {
  const { url, headers, model } = buildEndpointLegacy(config);
  const sysPrompt = getSystemPrompt(mode, context);
  if (config.provider === 'anthropic') return sendAnthropic(url, headers, model, messages, sysPrompt);
  if (config.provider === 'ollama') return sendOllama(url, headers, model, messages, sysPrompt);
  return sendOpenAICompatible(url, headers, model, messages, sysPrompt);
}

export async function sendChatWithProvider(
  messages: ChatMessage[],
  provider: AIProvider,
  mode?: ChatMode,
  context?: AIContext,
): Promise<string> {
  const { url, headers, model } = buildEndpointFromProvider(provider);
  const sysPrompt = getSystemPrompt(mode, context);
  if (provider.type === 'anthropic') return sendAnthropic(url, headers, model, messages, sysPrompt);
  if (provider.type === 'ollama') return sendOllama(url, headers, model, messages, sysPrompt);
  return sendOpenAICompatible(url, headers, model, messages, sysPrompt);
}

export async function getCommandSuggestion(
  partialCommand: string,
  provider: AIProvider,
  context?: AIContext,
): Promise<string> {
  const sysPrompt = `You are a shell command auto-complete assistant. Given a partial command, suggest the most likely completion. Return ONLY the completed command, nothing else. No explanation, no quotes, just the raw command.${context?.cwd ? ` CWD: ${context.cwd}` : ''}${context?.os ? ` OS: ${context.os}` : ''}`;
  const messages: ChatMessage[] = [{ role: 'user', content: partialCommand, timestamp: Date.now() }];
  const { url, headers, model } = buildEndpointFromProvider(provider);
  if (provider.type === 'ollama') return sendOllama(url, headers, model, messages, sysPrompt);
  if (provider.type === 'anthropic') return sendAnthropic(url, headers, model, messages, sysPrompt);
  return sendOpenAICompatible(url, headers, model, messages, sysPrompt);
}

const NO_TEMPERATURE_MODELS = /codex|o1|o3|o4/i;

async function sendOpenAICompatible(
  url: string, headers: Record<string, string>, model: string, messages: ChatMessage[], sysPrompt?: string,
): Promise<string> {
  const body: Record<string, unknown> = {
    model,
    messages: [
      { role: 'system', content: sysPrompt || SYSTEM_PROMPTS.chat },
      ...messages.map(m => ({ role: m.role, content: m.content })),
    ],
    max_tokens: 2048,
  };
  if (!NO_TEMPERATURE_MODELS.test(model)) {
    body.temperature = 0.7;
  }
  const res = await fetch(url, { method: 'POST', headers, body: JSON.stringify(body) });
  if (!res.ok) {
    const err = await res.text();
    throw new Error(`API error ${res.status}: ${err}`);
  }
  const data = await res.json();
  return data.choices?.[0]?.message?.content || 'No response';
}

async function sendAnthropic(
  url: string, headers: Record<string, string>, model: string, messages: ChatMessage[], sysPrompt?: string,
): Promise<string> {
  const body = {
    model,
    max_tokens: 2048,
    system: sysPrompt || SYSTEM_PROMPTS.chat,
    messages: messages.map(m => ({ role: m.role, content: m.content })),
  };
  const res = await fetch(url, { method: 'POST', headers, body: JSON.stringify(body) });
  if (!res.ok) {
    const err = await res.text();
    throw new Error(`Anthropic error ${res.status}: ${err}`);
  }
  const data = await res.json();
  return data.content?.[0]?.text || 'No response';
}

async function sendOllama(
  url: string, headers: Record<string, string>, model: string, messages: ChatMessage[], sysPrompt?: string,
): Promise<string> {
  const body = {
    model,
    messages: [
      { role: 'system', content: sysPrompt || SYSTEM_PROMPTS.chat },
      ...messages.map(m => ({ role: m.role, content: m.content })),
    ],
    stream: false,
  };
  const res = await fetch(url, { method: 'POST', headers, body: JSON.stringify(body) });
  if (!res.ok) {
    const err = await res.text();
    throw new Error(`Ollama error ${res.status}: ${err}`);
  }
  const data = await res.json();
  return data.message?.content || 'No response';
}
