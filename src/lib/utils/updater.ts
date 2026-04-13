const GITHUB_OWNER = 'musanmaz';
const GITHUB_REPO = 'nexterm';
const CURRENT_VERSION = '1.1.0';

export interface UpdateInfo {
  currentVersion: string;
  latestVersion: string;
  hasUpdate: boolean;
  downloadUrl: string;
  releaseUrl: string;
  releaseNotes: string;
  publishedAt: string;
}

function compareVersions(a: string, b: string): number {
  const pa = a.replace(/^v/, '').split('.').map(Number);
  const pb = b.replace(/^v/, '').split('.').map(Number);
  for (let i = 0; i < Math.max(pa.length, pb.length); i++) {
    const na = pa[i] || 0;
    const nb = pb[i] || 0;
    if (na > nb) return 1;
    if (na < nb) return -1;
  }
  return 0;
}

export async function checkForUpdates(): Promise<UpdateInfo> {
  const resp = await fetch(
    `https://api.github.com/repos/${GITHUB_OWNER}/${GITHUB_REPO}/releases/latest`,
    { headers: { Accept: 'application/vnd.github.v3+json' } },
  );

  if (!resp.ok) {
    throw new Error(`GitHub API error: ${resp.status}`);
  }

  const data = await resp.json();
  const latestTag: string = data.tag_name || '';
  const latestVersion = latestTag.replace(/^v/, '');

  const assets: { name: string; browser_download_url: string }[] = data.assets || [];
  const dmgAsset = assets.find(a => a.name.endsWith('.dmg'));

  return {
    currentVersion: CURRENT_VERSION,
    latestVersion,
    hasUpdate: compareVersions(latestVersion, CURRENT_VERSION) > 0,
    downloadUrl: dmgAsset?.browser_download_url || '',
    releaseUrl: data.html_url || '',
    releaseNotes: data.body || '',
    publishedAt: data.published_at || '',
  };
}

export function getCurrentVersion(): string {
  return CURRENT_VERSION;
}
