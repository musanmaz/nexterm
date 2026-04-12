import type { GitBranch, GitCommit, GitStatus, FileDiff } from '$lib/types';
import { gitGetBranches, gitGetLog, gitGetStatus, gitGetDiff } from '$lib/utils/ipc';

let branches = $state<GitBranch[]>([]);
let commits = $state<GitCommit[]>([]);
let status = $state<GitStatus | null>(null);
let diffs = $state<FileDiff[]>([]);
let currentPath = $state('');
let isRepo = $state(false);

export function getGitStore() {
  async function setPath(path: string) {
    currentPath = path;
    try {
      await refresh();
      isRepo = true;
    } catch {
      isRepo = false;
    }
  }

  async function refresh() {
    if (!currentPath) return;
    try {
      const [b, c, s] = await Promise.all([
        gitGetBranches(currentPath),
        gitGetLog(currentPath, 100),
        gitGetStatus(currentPath),
      ]);
      branches = b;
      commits = c;
      status = s;
    } catch (e) {
      console.error('Git refresh error:', e);
    }
  }

  async function loadDiff(staged: boolean) {
    if (!currentPath) return;
    try {
      diffs = await gitGetDiff(currentPath, staged);
    } catch (e) {
      console.error('Git diff error:', e);
    }
  }

  return {
    get branches() { return branches; },
    get commits() { return commits; },
    get status() { return status; },
    get diffs() { return diffs; },
    get currentPath() { return currentPath; },
    get isRepo() { return isRepo; },
    setPath,
    refresh,
    loadDiff,
  };
}
