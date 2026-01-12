use crate::app::Settings;
use crate::dto::repository::{
    CreateRepositoryRequest, CreateRepositoryResponse, RepositoryCommit, RepositoryCommits,
    RepositoryCommitsQuery, RepositoryFile, RepositoryFileCommitsQuery, RepositoryFileQuery,
    RepositoryTree, RepositoryTreeEntry, RepositoryTreeQuery,
};
use crate::utils::git::normalize_repo_name;
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use chrono::DateTime;
use std::collections::{HashMap, HashSet};
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
    let fallback_commit = RepositoryCommit {
        sha: commit_sha.clone(),
        message: commit.message().unwrap_or("").to_string(),
        author: commit.author().name().unwrap_or("Unknown").to_string(),
        date: DateTime::from_timestamp(commit.author().when().seconds(), 0).unwrap_or_default(),
    };

    let mut path_collector = Vec::new();

    // note: it may be unnecessary to iterate like this
    collect_paths(&repository, &subtree, &query.path, &mut path_collector)?;
    let needed_paths: HashSet<String> = path_collector.into_iter().collect();
    let commit_map = build_path_commit_map(&repository, commit, &needed_paths)?;

    let mut entries = Vec::new();
    walk_tree_recursive(
        &repository,
        &subtree,
        &query.path,
        &mut entries,
        &commit_map,
        &fallback_commit,
    )?;

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
    let commit_sha = commit.id().to_string();

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

pub async fn get_repository_file_commits(
    State(settings): State<Arc<Settings>>,
    Path((owner, repo)): Path<(String, String)>,
    Query(query): Query<RepositoryFileCommitsQuery>,
) -> Result<Json<RepositoryCommits>, StatusCode> {
    if query.path.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let repo_name = normalize_repo_name(&repo);
    let repository = open_repository(&settings, &owner, &repo_name)?;
    let commit = resolve_ref(&repository, &query.ref_name)?;

    let skip = ((query.page.saturating_sub(1)) * query.per_page) as usize;
    let take = query.per_page as usize;
    let (commits, has_next) = get_file_commits(&repository, commit, &query.path, skip, take)?;

    Ok(Json(RepositoryCommits { commits, has_next }))
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

fn collect_paths(
    repo: &git2::Repository,
    tree: &git2::Tree,
    base_path: &str,
    paths: &mut Vec<String>,
) -> Result<(), StatusCode> {
    for entry in tree.iter() {
        let name = entry.name().unwrap_or("").to_string();
        let entry_path = if base_path.is_empty() {
            name.clone()
        } else {
            format!("{}/{}", base_path, name)
        };

        paths.push(entry_path.clone());

        // Recursively collect paths from subdirectories
        if entry.kind() == Some(git2::ObjectType::Tree) {
            if let Ok(subtree) = repo.find_tree(entry.id()) {
                collect_paths(repo, &subtree, &entry_path, paths)?;
            }
        }
    }
    Ok(())
}

fn walk_tree_recursive(
    repo: &git2::Repository,
    tree: &git2::Tree,
    base_path: &str,
    entries: &mut Vec<RepositoryTreeEntry>,
    commit_map: &HashMap<String, RepositoryCommit>,
    fallback_commit: &RepositoryCommit,
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

        // Get commit for this path, fallback to HEAD commit if not found
        let commit = commit_map
            .get(&entry_path)
            .cloned()
            .unwrap_or_else(|| fallback_commit.clone());

        let mut tree_entry = RepositoryTreeEntry {
            path: entry_path.clone(),
            name,
            entry_type: entry_type.clone(),
            sha,
            commit,
            preview: None,
        };

        if tree_entry.entry_type == "blob" {
            if let Ok(blob) = repo.find_blob(entry.id()) {
                tree_entry.preview = generate_blob_preview(&blob);
            }
        }
        entries.push(tree_entry);

        if entry_type == "tree" {
            if let Ok(subtree) = repo.find_tree(entry.id()) {
                walk_tree_recursive(
                    repo,
                    &subtree,
                    &entry_path,
                    entries,
                    commit_map,
                    fallback_commit,
                )?;
            }
        }
    }
    Ok(())
}

fn is_binary(data: &[u8]) -> bool {
    data.iter().take(8000).any(|&b| b == 0)
}

fn generate_blob_preview(blob: &git2::Blob) -> Option<String> {
    if is_binary(blob.content()) {
        return None;
    }
    let content = String::from_utf8_lossy(blob.content());
    let preview: Vec<&str> = content.lines().take(100).collect();

    if preview.is_empty() {
        return None;
    }
    Some(preview.join("\n"))
}

fn get_file_commits(
    repo: &git2::Repository,
    start_commit: git2::Commit,
    file_path: &str,
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

    let mut current_path = file_path.to_string();
    let mut commits = Vec::new();
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
            if count >= skip && commits.len() < take + 1 {
                let commit_sha = commit.id().to_string();
                let message = commit.message().unwrap_or("").to_string();
                let author = commit.author();
                let author_name = author.name().unwrap_or("Unknown").to_string();
                let timestamp = author.when().seconds();
                let date = DateTime::from_timestamp(timestamp, 0).unwrap_or_default();

                commits.push(RepositoryCommit {
                    sha: commit_sha,
                    message,
                    author: author_name,
                    date,
                });
            }
            count += 1;
        }

        // Early exit when we have enough results
        if commits.len() > take {
            break;
        }
    }

    let has_next = commits.len() > take;
    if has_next {
        commits.pop();
    }

    Ok((commits, has_next))
}

fn build_path_commit_map(
    repo: &git2::Repository,
    start_commit: git2::Commit,
    needed_paths: &HashSet<String>,
) -> Result<HashMap<String, RepositoryCommit>, StatusCode> {
    let mut path_commit_map = HashMap::new();
    let mut found_paths = HashSet::new();

    let mut revwalk = repo
        .revwalk()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    revwalk
        .push(start_commit.id())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    revwalk.set_sorting(git2::Sort::TIME).ok();

    for oid_result in revwalk {
        let oid = oid_result.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let commit = repo
            .find_commit(oid)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let commit_sha = commit.id().to_string();
        let message = commit.message().unwrap_or("").to_string();
        let author = commit.author();
        let author_name = author.name().unwrap_or("Unknown").to_string();
        let timestamp = author.when().seconds();
        let date = DateTime::from_timestamp(timestamp, 0).unwrap_or_default();

        let repo_commit = RepositoryCommit {
            sha: commit_sha,
            message,
            author: author_name,
            date,
        };

        // Handle initial commit (no parents)
        if commit.parent_count() == 0 {
            let tree = commit
                .tree()
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            // All paths in the initial commit were created by it
            for path in needed_paths.iter() {
                if path_commit_map.contains_key(path) {
                    continue;
                }

                if tree.get_path(std::path::Path::new(path)).is_ok() {
                    path_commit_map.insert(path.clone(), repo_commit.clone());
                    found_paths.insert(path.clone());
                }
            }
        } else {
            // Check diff against all parents
            for parent in commit.parents() {
                let parent_tree = parent
                    .tree()
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                let commit_tree = commit
                    .tree()
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

                let diff = repo
                    .diff_tree_to_tree(Some(&parent_tree), Some(&commit_tree), None)
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

                // Process each modified file
                for delta in diff.deltas() {
                    let modified_path = delta
                        .new_file()
                        .path()
                        .or_else(|| delta.old_file().path())
                        .and_then(|p| p.to_str())
                        .map(|s| s.to_string());

                    if let Some(path) = modified_path {
                        // Add commit for this exact path if needed
                        if needed_paths.contains(&path) && !path_commit_map.contains_key(&path) {
                            path_commit_map.insert(path.clone(), repo_commit.clone());
                            found_paths.insert(path.clone());
                        }

                        // Add commit for all parent folder paths
                        let mut parent_path = path.as_str();
                        while let Some(idx) = parent_path.rfind('/') {
                            parent_path = &parent_path[..idx];
                            if needed_paths.contains(parent_path)
                                && !path_commit_map.contains_key(parent_path)
                            {
                                path_commit_map
                                    .insert(parent_path.to_string(), repo_commit.clone());
                                found_paths.insert(parent_path.to_string());
                            }
                        }
                    }
                }
            }
        }

        // Early exit optimization
        if found_paths.len() == needed_paths.len() {
            break;
        }
    }

    Ok(path_commit_map)
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
