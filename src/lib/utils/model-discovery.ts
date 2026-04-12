import type { AIProvider } from '$lib/types';

/**
 * Normalize base URL: strip trailing slash and /v1 suffix
 * so we always have a clean root like https://lite-llm.turknet.net.tr
 */
function stripV1(url: string): string {
  return url.replace(/\/+$/, '').replace(/\/v1$/i, '');
}

export async function discoverModels(provider: AIProvider): Promise<string[]> {
  try {
    if (provider.type === 'ollama') return await discoverOllamaModels(provider.baseUrl);
    if (provider.type === 'litellm') return await discoverLiteLLMModels(provider.baseUrl, provider.apiKey);
    if (provider.type === 'openai-compatible') return await discoverOpenAICompatibleModels(provider.baseUrl, provider.apiKey);
    if (provider.type === 'openai') return await discoverOpenAIModels(provider.apiKey);
    return [];
  } catch (e) {
    console.warn('Model discovery failed:', e);
    return [];
  }
}

function litellmHeaders(apiKey: string): Record<string, string> {
  const h: Record<string, string> = { 'Content-Type': 'application/json' };
  if (apiKey) {
    h['Authorization'] = `Bearer ${apiKey}`;
    h['x-litellm-api-key'] = apiKey;
  }
  return h;
}

async function discoverLiteLLMModels(baseUrl: string, apiKey: string): Promise<string[]> {
  const root = stripV1(baseUrl);
  const headers = litellmHeaders(apiKey);

  // 1) Try /model/info (LiteLLM-specific, at root level)
  try {
    const res = await fetch(`${root}/model/info`, { headers, signal: AbortSignal.timeout(5000) });
    if (res.ok) {
      const data = await res.json();
      if (data.data && Array.isArray(data.data)) {
        const models = data.data.map((m: any) => m.model_name || m.model_info?.id || '').filter(Boolean);
        if (models.length > 0) return models;
      }
    }
  } catch { /* fallback */ }

  // 2) Try /v1/models (OpenAI-compatible)
  try {
    const res = await fetch(`${root}/v1/models`, { headers, signal: AbortSignal.timeout(5000) });
    if (res.ok) {
      const data = await res.json();
      if (data.data && Array.isArray(data.data)) {
        return data.data.map((m: any) => m.id).filter(Boolean).sort();
      }
    }
  } catch { /* fallback */ }

  // 3) Try /models (some proxies serve it here)
  try {
    const res = await fetch(`${root}/models`, { headers, signal: AbortSignal.timeout(5000) });
    if (res.ok) {
      const data = await res.json();
      if (data.data && Array.isArray(data.data)) {
        return data.data.map((m: any) => m.id).filter(Boolean).sort();
      }
    }
  } catch { /* nothing left */ }

  return [];
}

async function discoverOpenAICompatibleModels(baseUrl: string, apiKey: string): Promise<string[]> {
  const root = stripV1(baseUrl);
  const headers: Record<string, string> = { 'Content-Type': 'application/json' };
  if (apiKey) headers['Authorization'] = `Bearer ${apiKey}`;

  const res = await fetch(`${root}/v1/models`, { headers, signal: AbortSignal.timeout(5000) });
  if (!res.ok) throw new Error(`${res.status}`);
  const data = await res.json();
  if (data.data && Array.isArray(data.data)) {
    return data.data.map((m: any) => m.id).filter(Boolean).sort();
  }
  return [];
}

async function discoverOllamaModels(baseUrl: string): Promise<string[]> {
  const base = baseUrl.replace(/\/+$/, '');
  const res = await fetch(`${base}/api/tags`, { signal: AbortSignal.timeout(5000) });
  if (!res.ok) throw new Error(`${res.status}`);
  const data = await res.json();
  if (data.models && Array.isArray(data.models)) {
    return data.models.map((m: any) => m.name).filter(Boolean).sort();
  }
  return [];
}

async function discoverOpenAIModels(apiKey: string): Promise<string[]> {
  if (!apiKey) return ['gpt-4', 'gpt-4o', 'gpt-4o-mini', 'gpt-3.5-turbo', 'o1', 'o1-mini'];
  const res = await fetch('https://api.openai.com/v1/models', {
    headers: { 'Authorization': `Bearer ${apiKey}` },
    signal: AbortSignal.timeout(5000),
  });
  if (!res.ok) return ['gpt-4', 'gpt-4o', 'gpt-4o-mini', 'gpt-3.5-turbo'];
  const data = await res.json();
  if (data.data && Array.isArray(data.data)) {
    return data.data
      .map((m: any) => m.id)
      .filter((id: string) => id.startsWith('gpt-') || id.startsWith('o1') || id.startsWith('o3') || id.startsWith('o4'))
      .sort();
  }
  return [];
}

export async function testConnection(provider: AIProvider): Promise<{ ok: boolean; message: string; models?: string[] }> {
  try {
    const models = await discoverModels(provider);
    if (models.length > 0) {
      return { ok: true, message: `Connection successful! ${models.length} model(s) found.`, models };
    }
    const root = stripV1(provider.baseUrl);
    let headers: Record<string, string> = {};
    if (provider.type === 'litellm' || provider.type === 'openai-compatible') {
      headers = litellmHeaders(provider.apiKey);
    } else if (provider.type === 'anthropic') {
      headers = { 'x-api-key': provider.apiKey, 'anthropic-version': '2023-06-01' };
    } else if (provider.apiKey) {
      headers = { 'Authorization': `Bearer ${provider.apiKey}` };
    }
    const endpoint = provider.type === 'ollama' ? `${root}/api/tags` :
                     provider.type === 'anthropic' ? 'https://api.anthropic.com/v1/models' :
                     `${root}/v1/models`;
    const res = await fetch(endpoint, { headers, signal: AbortSignal.timeout(5000) });
    if (res.ok) return { ok: true, message: 'Connection is available, but model list could not be fetched', models: [] };
    return { ok: false, message: `HTTP ${res.status}: ${res.statusText}` };
  } catch (e) {
    return { ok: false, message: `Connection error: ${e}` };
  }
}
