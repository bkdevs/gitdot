use crate::app::settings::Settings;
use crate::dto::repository::{
    CreateRepositoryRequest, CreateRepositoryResponse, FileHistoryEntry, RepositoryCommit,
    RepositoryCommits, RepositoryCommitsQuery, RepositoryFile, RepositoryFileHistory,
    RepositoryFileHistoryQuery, RepositoryFileQuery, RepositoryTree, RepositoryTreeEntry,
    RepositoryTreeQuery,
};
use crate::utils::git::normalize_repo_name;
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use chrono::DateTime;
use std::sync::Arc;

pub async fn create_repository(
    State(settings): State<Arc<Settings>>,
    Path((owner, repo)): Path<(String, String)>,
    Json(request): Json<CreateRepositoryRequest>,
) -> Result<Json<CreateRepositoryResponse>, StatusCode> {
    let repo_name = normalize_repo_name(&repo);
    let repo_path = format!("{}/{}/{}", settings.git_project_root, owner, repo_name);
    if tokio::fs::try_exists(&repo_path).await.unwrap_or(false) {
        return Err(StatusCode::CONFLICT);
    }

    let owner_path = format!("{}/{}", settings.git_project_root, owner);
    tokio::fs::create_dir_all(&owner_path)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Initialize bare repository (blocking operation, spawn on blocking thread)
    let repo_path_clone = repo_path.clone();
    let default_branch = request.default_branch.clone();
    tokio::task::spawn_blocking(move || {
        let repo = git2::Repository::init_bare(&repo_path_clone)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        repo.set_head(&format!("refs/heads/{}", default_branch))
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        // Configure the repository for HTTP access
        let mut config = repo
            .config()
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        config
            .set_bool("http.receivepack", true)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok::<(), StatusCode>(())
    })
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)??;

    // Create git-daemon-export-ok file to allow HTTP access
    let export_ok_path = format!("{}/git-daemon-export-ok", repo_path);
    tokio::fs::write(&export_ok_path, "")
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(CreateRepositoryResponse {
        owner: owner,
        name: repo_name,
        default_branch: request.default_branch,
    }))
}

pub async fn get_repository_tree(
    State(settings): State<Arc<Settings>>,
    Path((owner, repo)): Path<(String, String)>,
    Query(query): Query<RepositoryTreeQuery>,
) -> Result<Json<RepositoryTree>, StatusCode> {
    let repo_path = normalize_repo_name(&repo);
    let repository = open_repository(&settings, &owner, &repo_path)?;
    let commit = resolve_ref(&repository, &query.ref_name)?;
    let commit_sha = commit.id().to_string();
    let tree = commit
        .tree()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let subtree = get_tree_at_path(&repository, &tree, &query.path)?;
    let mut entries = Vec::new();
    walk_tree_recursive(&repository, &subtree, &query.path, &mut entries)?;
    Ok(Json(RepositoryTree {
        ref_name: query.ref_name,
        commit_sha,
        path: query.path,
        entries,
    }))
}

pub async fn get_repository_file(
    State(settings): State<Arc<Settings>>,
    Path((owner, repo)): Path<(String, String)>,
    Query(query): Query<RepositoryFileQuery>,
) -> Result<Json<RepositoryFile>, StatusCode> {
    if query.path.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let repo_name = normalize_repo_name(&repo);
    let repository = open_repository(&settings, &owner, &repo_name)?;
    let commit = resolve_ref(&repository, &query.ref_name)?;
    let commit_sha = get_latest_file_commit_sha(&repository, commit.clone(), &query.path)?;

    let tree = commit
        .tree()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let blob = get_blob_at_path(&repository, &tree, &query.path)?;
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
    Ok(Json(RepositoryFile {
        ref_name: query.ref_name,
        commit_sha,
        path: query.path,
        sha,
        content,
        encoding,
    }))
}

pub async fn get_repository_commits(
    State(settings): State<Arc<Settings>>,
    Path((owner, repo)): Path<(String, String)>,
    Query(query): Query<RepositoryCommitsQuery>,
) -> Result<Json<RepositoryCommits>, StatusCode> {
    let repo_name = normalize_repo_name(&repo);
    let repository = open_repository(&settings, &owner, &repo_name)?;
    let commit = resolve_ref(&repository, &query.ref_name)?;

    let skip = ((query.page.saturating_sub(1)) * query.per_page) as usize;
    let take = query.per_page as usize;
    let (commits, has_next) = get_commits(&repository, commit, skip, take)?;

    Ok(Json(RepositoryCommits { commits, has_next }))
}

pub async fn get_repository_file_history(
    State(settings): State<Arc<Settings>>,
    Path((owner, repo)): Path<(String, String)>,
    Query(query): Query<RepositoryFileHistoryQuery>,
) -> Result<Json<RepositoryFileHistory>, StatusCode> {
    if query.path.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let repo_name = normalize_repo_name(&repo);
    let repository = open_repository(&settings, &owner, &repo_name)?;
    let commit = resolve_ref(&repository, &query.ref_name)?;

    let skip = ((query.page.saturating_sub(1)) * query.per_page) as usize;
    let take = query.per_page as usize;
    let (history, has_next) = get_file_history(
        &repository,
        commit,
        &query.path,
        query.ref_name.clone(),
        skip,
        take,
    )?;

    Ok(Json(RepositoryFileHistory { history, has_next }))
}

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

fn get_tree_at_path<'repo>(
    repo: &'repo git2::Repository,
    tree: &git2::Tree<'repo>,
    path: &str,
) -> Result<git2::Tree<'repo>, StatusCode> {
    if path.is_empty() {
        return Ok(tree.clone());
    }

    let tree_entry = tree
        .get_path(std::path::Path::new(path))
        .map_err(|_| StatusCode::NOT_FOUND)?;

    if tree_entry.kind() != Some(git2::ObjectType::Tree) {
        return Err(StatusCode::BAD_REQUEST);
    }

    repo.find_tree(tree_entry.id())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

fn get_blob_at_path<'repo>(
    repo: &'repo git2::Repository,
    tree: &git2::Tree<'repo>,
    path: &str,
) -> Result<git2::Blob<'repo>, StatusCode> {
    let tree_entry = tree
        .get_path(std::path::Path::new(path))
        .map_err(|_| StatusCode::NOT_FOUND)?;

    if tree_entry.kind() != Some(git2::ObjectType::Blob) {
        return Err(StatusCode::BAD_REQUEST);
    }

    repo.find_blob(tree_entry.id())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

fn walk_tree_recursive(
    repo: &git2::Repository,
    tree: &git2::Tree,
    base_path: &str,
    entries: &mut Vec<RepositoryTreeEntry>,
) -> Result<(), StatusCode> {
    for entry in tree.iter() {
        let name = entry.name().unwrap_or("").to_string();
        let entry_path = if base_path.is_empty() {
            name.clone()
        } else {
            format!("{}/{}", base_path, name)
        };

        let entry_type = match entry.kind() {
            Some(git2::ObjectType::Blob) => "blob",
            Some(git2::ObjectType::Tree) => "tree",
            Some(git2::ObjectType::Commit) => "commit", // for Git submodule
            _ => "unknown",
        }
        .to_string();
        let sha = entry.id().to_string();

        entries.push(RepositoryTreeEntry {
            path: entry_path.clone(),
            name,
            entry_type: entry_type.clone(),
            sha,
        });

        if entry_type == "tree" {
            if let Ok(subtree) = repo.find_tree(entry.id()) {
                walk_tree_recursive(repo, &subtree, &entry_path, entries)?;
            }
        }
    }
    Ok(())
}

fn is_binary(data: &[u8]) -> bool {
    data.iter().take(8000).any(|&b| b == 0)
}

fn get_latest_file_commit_sha(
    repo: &git2::Repository,
    start_commit: git2::Commit,
    file_path: &str,
) -> Result<String, StatusCode> {
    let mut revwalk = repo
        .revwalk()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    revwalk
        .push(start_commit.id())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    revwalk.set_sorting(git2::Sort::TIME).ok();

    let mut current_path = file_path.to_string();

    for oid_result in revwalk {
        let oid = oid_result.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let commit = repo
            .find_commit(oid)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let mut modified = false;

        if commit.parent_count() == 0 {
            let tree = commit
                .tree()
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            if tree.get_path(std::path::Path::new(&current_path)).is_ok() {
                modified = true;
            }
        } else {
            for parent in commit.parents() {
                let parent_tree = parent
                    .tree()
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                let commit_tree = commit
                    .tree()
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

                let mut diff_opts = git2::DiffOptions::new();
                diff_opts.pathspec(&current_path);

                let diff = repo
                    .diff_tree_to_tree(Some(&parent_tree), Some(&commit_tree), Some(&mut diff_opts))
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

                // Check if diff contains our path
                if diff.deltas().any(|delta| {
                    delta
                        .new_file()
                        .path()
                        .and_then(|p| p.to_str())
                        .map(|p| p == current_path)
                        .unwrap_or(false)
                        || delta
                            .old_file()
                            .path()
                            .and_then(|p| p.to_str())
                            .map(|p| p == current_path)
                            .unwrap_or(false)
                }) {
                    modified = true;

                    // Check for renames and update path for next iteration
                    for delta in diff.deltas() {
                        if delta.status() == git2::Delta::Renamed {
                            if let Some(new_path) = delta.new_file().path() {
                                if new_path.to_str() == Some(&current_path) {
                                    if let Some(old_path) = delta.old_file().path() {
                                        current_path = old_path.to_str().unwrap().to_string();
                                    }
                                }
                            }
                        }
                    }

                    break; // Found modification, no need to check other parents
                }
            }
        }

        if modified {
            return Ok(commit.id().to_string());
        }
    }

    Err(StatusCode::NOT_FOUND)
}

fn get_file_history(
    repo: &git2::Repository,
    start_commit: git2::Commit,
    file_path: &str,
    ref_name: String,
    skip: usize,
    take: usize,
) -> Result<(Vec<FileHistoryEntry>, bool), StatusCode> {
    let mut revwalk = repo
        .revwalk()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    revwalk
        .push(start_commit.id())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    revwalk.set_sorting(git2::Sort::TIME).ok();

    let mut current_path = file_path.to_string();
    let mut history = Vec::new();
    let mut count = 0;

    for oid_result in revwalk {
        let oid = oid_result.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let commit = repo
            .find_commit(oid)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let mut modified = false;

        if commit.parent_count() == 0 {
            let tree = commit
                .tree()
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            if tree.get_path(std::path::Path::new(&current_path)).is_ok() {
                modified = true;
            }
        } else {
            for parent in commit.parents() {
                let parent_tree = parent
                    .tree()
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                let commit_tree = commit
                    .tree()
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

                let mut diff_opts = git2::DiffOptions::new();
                diff_opts.pathspec(&current_path);

                let diff = repo
                    .diff_tree_to_tree(Some(&parent_tree), Some(&commit_tree), Some(&mut diff_opts))
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

                // Check if diff contains our path
                if diff.deltas().any(|delta| {
                    delta
                        .new_file()
                        .path()
                        .and_then(|p| p.to_str())
                        .map(|p| p == current_path)
                        .unwrap_or(false)
                        || delta
                            .old_file()
                            .path()
                            .and_then(|p| p.to_str())
                            .map(|p| p == current_path)
                            .unwrap_or(false)
                }) {
                    modified = true;

                    // Check for renames and update path for next iteration
                    for delta in diff.deltas() {
                        if delta.status() == git2::Delta::Renamed {
                            if let Some(new_path) = delta.new_file().path() {
                                if new_path.to_str() == Some(&current_path) {
                                    if let Some(old_path) = delta.old_file().path() {
                                        current_path = old_path.to_str().unwrap().to_string();
                                    }
                                }
                            }
                        }
                    }

                    break; // Found modification, no need to check other parents
                }
            }
        }

        if modified {
            if count >= skip && history.len() < take + 1 {
                // Get the commit tree
                let tree = commit
                    .tree()
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

                // Get the file blob at this commit
                let blob = get_blob_at_path(repo, &tree, &current_path)?;
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

                // Create RepositoryFile
                let repository_file = RepositoryFile {
                    ref_name: ref_name.clone(),
                    commit_sha: commit.id().to_string(),
                    path: current_path.clone(),
                    sha,
                    content,
                    encoding,
                };

                // Create RepositoryCommit
                let commit_sha = commit.id().to_string();
                let message = commit.message().unwrap_or("").to_string();
                let author = commit.author();
                let author_name = author.name().unwrap_or("Unknown").to_string();
                let timestamp = author.when().seconds();
                let date = DateTime::from_timestamp(timestamp, 0).unwrap_or_default();

                let repository_commit = RepositoryCommit {
                    sha: commit_sha,
                    message,
                    author: author_name,
                    date,
                };

                history.push(FileHistoryEntry {
                    file: repository_file,
                    commit: repository_commit,
                });
            }
            count += 1;
        }

        // Early exit when we have enough results
        if history.len() > take {
            break;
        }
    }

    let has_next = history.len() > take;
    if has_next {
        history.pop();
    }

    Ok((history, has_next))
}

fn get_commits(
    repo: &git2::Repository,
    start_commit: git2::Commit,
    skip: usize,
    take: usize,
) -> Result<(Vec<RepositoryCommit>, bool), StatusCode> {
    let mut revwalk = repo
        .revwalk()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    revwalk
        .push(start_commit.id())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    revwalk.set_sorting(git2::Sort::TIME).ok();

    let commits: Result<Vec<RepositoryCommit>, StatusCode> = revwalk
        .skip(skip)
        .take(take + 1)
        .map(|oid| {
            let oid = oid.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            let commit = repo
                .find_commit(oid)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            let sha = commit.id().to_string();
            let message = commit.message().unwrap_or("").to_string();
            let author = commit.author();
            let author_name = author.name().unwrap_or("Unknown").to_string();
            let timestamp = author.when().seconds();
            let date = DateTime::from_timestamp(timestamp, 0).unwrap_or_default();

            Ok(RepositoryCommit {
                sha,
                message,
                author: author_name,
                date,
            })
        })
        .collect();

    let mut commits = commits?;
    let has_next = commits.len() > take;

    // hack to check if there are more commits to fetch
    if has_next {
        commits.pop();
    }
    Ok((commits, has_next))
}
