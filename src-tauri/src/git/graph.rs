use git2::Repository;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitBranch {
    pub name: String,
    pub is_head: bool,
    pub is_remote: bool,
    pub commit_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitCommit {
    pub id: String,
    pub short_id: String,
    pub message: String,
    pub author: String,
    pub email: String,
    pub time: i64,
    pub parents: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitStatus {
    pub staged: Vec<String>,
    pub unstaged: Vec<String>,
    pub untracked: Vec<String>,
    pub branch: String,
    pub ahead: usize,
    pub behind: usize,
}

#[tauri::command]
pub fn git_get_branches(path: String) -> Result<Vec<GitBranch>, String> {
    let repo = Repository::open(&path).map_err(|e| format!("Git error: {}", e))?;
    let mut branches = Vec::new();

    for branch_result in repo
        .branches(None)
        .map_err(|e| format!("Branch error: {}", e))?
    {
        let (branch, branch_type) = branch_result.map_err(|e| format!("Error: {}", e))?;
        let name = branch.name().ok().flatten().unwrap_or("unknown").to_string();
        let is_head = branch.is_head();
        let commit_id = branch
            .get()
            .peel_to_commit()
            .map(|c| c.id().to_string())
            .unwrap_or_default();

        branches.push(GitBranch {
            name,
            is_head,
            is_remote: branch_type == git2::BranchType::Remote,
            commit_id,
        });
    }

    Ok(branches)
}

#[tauri::command]
pub fn git_get_log(path: String, max_count: usize) -> Result<Vec<GitCommit>, String> {
    let repo = Repository::open(&path).map_err(|e| format!("Git error: {}", e))?;
    let mut revwalk = repo.revwalk().map_err(|e| format!("Revwalk error: {}", e))?;
    revwalk.push_head().map_err(|e| format!("Error: {}", e))?;
    revwalk
        .set_sorting(git2::Sort::TIME)
        .map_err(|e| format!("Sort error: {}", e))?;

    let mut commits = Vec::new();
    for oid_result in revwalk.take(max_count) {
        let oid = oid_result.map_err(|e| format!("Error: {}", e))?;
        let commit = repo.find_commit(oid).map_err(|e| format!("Error: {}", e))?;
        let id = commit.id().to_string();
        let short_id = id.chars().take(7).collect();

        commits.push(GitCommit {
            id,
            short_id,
            message: commit.message().unwrap_or("").to_string(),
            author: commit.author().name().unwrap_or("").to_string(),
            email: commit.author().email().unwrap_or("").to_string(),
            time: commit.time().seconds(),
            parents: commit.parent_ids().map(|id| id.to_string()).collect(),
        });
    }

    Ok(commits)
}

#[tauri::command]
pub fn git_get_status(path: String) -> Result<GitStatus, String> {
    let repo = Repository::open(&path).map_err(|e| format!("Git error: {}", e))?;
    let statuses = repo
        .statuses(None)
        .map_err(|e| format!("Status error: {}", e))?;

    let mut staged = Vec::new();
    let mut unstaged = Vec::new();
    let mut untracked = Vec::new();

    for entry in statuses.iter() {
        let path = entry.path().unwrap_or("").to_string();
        let status = entry.status();

        if status.intersects(
            git2::Status::INDEX_NEW
                | git2::Status::INDEX_MODIFIED
                | git2::Status::INDEX_DELETED
                | git2::Status::INDEX_RENAMED,
        ) {
            staged.push(path.clone());
        }

        if status.intersects(git2::Status::WT_MODIFIED | git2::Status::WT_DELETED | git2::Status::WT_RENAMED) {
            unstaged.push(path.clone());
        }

        if status.contains(git2::Status::WT_NEW) {
            untracked.push(path);
        }
    }

    let branch = repo
        .head()
        .ok()
        .and_then(|h| h.shorthand().map(|s| s.to_string()))
        .unwrap_or_else(|| "HEAD".to_string());

    Ok(GitStatus {
        staged,
        unstaged,
        untracked,
        branch,
        ahead: 0,
        behind: 0,
    })
}
