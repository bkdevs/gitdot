use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::DateTime;

use crate::app::Settings;
use crate::dto::legacy_repository::{
    DifftasticOutput, RepositoryCommit, RepositoryCommitDiffs, RepositoryFile, RepositoryFileDiff,
};
use crate::utils::git::normalize_repo_name;

fn open_repository(
    settings: &Settings,
    owner: &str,
    repo: &str,
) -> Result<git2::Repository, StatusCode> {
    let repo_path = format!("{}/{}/{}", settings.git_project_root, owner, repo);
    git2::Repository::open_bare(&repo_path).map_err(|_| StatusCode::NOT_FOUND)
}

fn resolve_ref<'repo>(
    repo: &'repo git2::Repository,
    ref_name: &str,
) -> Result<git2::Commit<'repo>, StatusCode> {
    let obj = repo
        .revparse_single(ref_name)
        .map_err(|_| StatusCode::NOT_FOUND)?;
    obj.peel_to_commit().map_err(|_| StatusCode::NOT_FOUND)
}

fn is_binary(data: &[u8]) -> bool {
    data.iter().take(8000).any(|&b| b == 0)
}

pub async fn get_repository_commit_stats(
    State(settings): State<Arc<Settings>>,
    Path((owner, repo, sha)): Path<(String, String, String)>,
) -> Result<Json<RepositoryCommitDiffs>, StatusCode> {
    if sha.len() < 4 || sha.len() > 40 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let repo_name = normalize_repo_name(&repo);
    let repository = open_repository(&settings, &owner, &repo_name)?;
    let commit = resolve_ref(&repository, &sha)?;
    let full_sha = commit.id().to_string();

    let commit_info = RepositoryCommit {
        sha: full_sha.clone(),
        message: commit.message().unwrap_or("").to_string(),
        author: commit.author().name().unwrap_or("Unknown").to_string(),
        date: DateTime::from_timestamp(commit.author().when().seconds(), 0).unwrap_or_default(),
    };

    let parent = commit.parent(0).ok();
    let parent_sha = parent.as_ref().map(|p| p.id().to_string());

    let current_tree = commit
        .tree()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let parent_tree = match &parent {
        Some(p) => Some(p.tree().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?),
        None => None,
    };

    let mut diff = repository
        .diff_tree_to_tree(parent_tree.as_ref(), Some(&current_tree), None)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut find_opts = git2::DiffFindOptions::new();
    find_opts.renames(true);
    find_opts.rename_threshold(50);
    diff.find_similar(Some(&mut find_opts))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut diffs = Vec::new();

    for (idx, delta) in diff.deltas().enumerate() {
        let old_path = delta
            .old_file()
            .path()
            .and_then(|p| p.to_str())
            .map(String::from);
        let new_path = delta
            .new_file()
            .path()
            .and_then(|p| p.to_str())
            .map(String::from);

        let left = old_path.map(|path| RepositoryFile {
            ref_name: parent_sha.clone().unwrap_or_default(),
            commit_sha: parent_sha.clone().unwrap_or_default(),
            path,
            sha: String::new(),
            content: String::new(),
            encoding: String::new(),
        });

        let right = new_path.map(|path| RepositoryFile {
            ref_name: full_sha.clone(),
            commit_sha: full_sha.clone(),
            path,
            sha: String::new(),
            content: String::new(),
            encoding: String::new(),
        });

        let (lines_added, lines_removed) = git2::Patch::from_diff(&diff, idx)
            .ok()
            .flatten()
            .and_then(|p| p.line_stats().ok())
            .map(|(_, additions, deletions)| (additions as u32, deletions as u32))
            .unwrap_or((0, 0));

        diffs.push(RepositoryFileDiff {
            left,
            right,
            lines_added,
            lines_removed,
            hunks: Vec::new(),
        });
    }

    Ok(Json(RepositoryCommitDiffs {
        sha: full_sha,
        parent_sha,
        commit: commit_info,
        diffs,
    }))
}

pub async fn get_repository_commit_diffs(
    State(settings): State<Arc<Settings>>,
    Path((owner, repo, sha)): Path<(String, String, String)>,
) -> Result<Json<RepositoryCommitDiffs>, StatusCode> {
    if sha.len() < 4 || sha.len() > 40 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let repo_name = normalize_repo_name(&repo);
    let repository = open_repository(&settings, &owner, &repo_name)?;
    let commit = resolve_ref(&repository, &sha)?;
    let full_sha = commit.id().to_string();

    let commit_info = RepositoryCommit {
        sha: full_sha.clone(),
        message: commit.message().unwrap_or("").to_string(),
        author: commit.author().name().unwrap_or("Unknown").to_string(),
        date: DateTime::from_timestamp(commit.author().when().seconds(), 0).unwrap_or_default(),
    };

    let parent = commit.parent(0).ok();
    let parent_sha = parent.as_ref().map(|p| p.id().to_string());

    let current_tree = commit
        .tree()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let parent_tree = match &parent {
        Some(p) => Some(p.tree().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?),
        None => None,
    };

    let mut diff = repository
        .diff_tree_to_tree(parent_tree.as_ref(), Some(&current_tree), None)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut find_opts = git2::DiffFindOptions::new();
    find_opts.renames(true);
    find_opts.rename_threshold(50); // 50% similarity (git default)
    diff.find_similar(Some(&mut find_opts))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut diffs = Vec::new();

    for (idx, delta) in diff.deltas().enumerate() {
        let old_path = delta
            .old_file()
            .path()
            .and_then(|p| p.to_str())
            .map(String::from);
        let new_path = delta
            .new_file()
            .path()
            .and_then(|p| p.to_str())
            .map(String::from);

        let left = get_repository_file_at_path(
            &repository,
            parent_tree.as_ref(),
            old_path.as_deref(),
            parent_sha.as_deref().unwrap_or(""),
        );

        let right = get_repository_file_at_path(
            &repository,
            Some(&current_tree),
            new_path.as_deref(),
            &full_sha,
        );

        let (lines_added, lines_removed) = git2::Patch::from_diff(&diff, idx)
            .ok()
            .flatten()
            .and_then(|p| p.line_stats().ok())
            .map(|(_, additions, deletions)| (additions as u32, deletions as u32))
            .unwrap_or((0, 0));

        let left_content = left.as_ref().map(|f| f.content.as_str()).unwrap_or("");
        let right_content = right.as_ref().map(|f| f.content.as_str()).unwrap_or("");
        let file_path_for_diff = new_path.as_deref().or(old_path.as_deref());
        let diff = execute_difftastic(left_content, right_content, file_path_for_diff)
            .expect("Failed to generate diff");

        diffs.push(RepositoryFileDiff {
            left,
            right,
            lines_added,
            lines_removed,
            hunks: diff.chunks.unwrap_or(Vec::new()),
        });
    }

    Ok(Json(RepositoryCommitDiffs {
        sha: full_sha,
        parent_sha,
        commit: commit_info,
        diffs,
    }))
}

fn get_repository_file_at_path(
    repo: &git2::Repository,
    tree: Option<&git2::Tree>,
    path: Option<&str>,
    commit_sha: &str,
) -> Option<RepositoryFile> {
    let tree = tree?;
    let path = path?;
    let entry = tree.get_path(std::path::Path::new(path)).ok()?;

    if entry.kind() != Some(git2::ObjectType::Blob) {
        return None;
    }

    let blob = repo.find_blob(entry.id()).ok()?;
    let content_bytes = blob.content();
    let sha = blob.id().to_string();

    let (content, encoding) = if is_binary(content_bytes) {
        use base64::prelude::*;
        (BASE64_STANDARD.encode(content_bytes), "base64".to_string())
    } else {
        (
            String::from_utf8_lossy(content_bytes).to_string(),
            "utf-8".to_string(),
        )
    };

    Some(RepositoryFile {
        ref_name: commit_sha.to_string(),
        commit_sha: commit_sha.to_string(),
        path: path.to_string(),
        sha,
        content,
        encoding,
    })
}

/**
 * runs difft in a process with tempfiles as that's the only API we can use
 * TODO: quite brittle, copy over internal code and maintain ourselves.
 */
fn execute_difftastic(
    left_content: &str,
    right_content: &str,
    file_path: Option<&str>,
) -> Option<DifftasticOutput> {
    use std::io::Write;
    use tempfile::Builder;

    let extension = file_path
        .and_then(|p| std::path::Path::new(p).extension())
        .and_then(|ext| ext.to_str())
        .unwrap_or("txt");

    let mut left_file = Builder::new()
        .suffix(&format!(".{}", extension))
        .tempfile()
        .ok()?;
    let mut right_file = Builder::new()
        .suffix(&format!(".{}", extension))
        .tempfile()
        .ok()?;

    left_file.write_all(left_content.as_bytes()).ok()?;
    right_file.write_all(right_content.as_bytes()).ok()?;

    let output = std::process::Command::new("difft")
        .arg(left_file.path())
        .arg(right_file.path())
        .arg("--display")
        .arg("json")
        .env("DFT_UNSTABLE", "yes")
        .output()
        .ok()?;

    serde_json::from_slice(&output.stdout).ok()
}
