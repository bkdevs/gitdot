use crate::config::settings::Settings;
use crate::models::repository::{RepositoryFile, RepositoryTree, RepositoryTreeEntry};
use crate::utils::git::normalize_repo_name;
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use std::sync::Arc;

#[derive(serde::Deserialize)]
pub struct RepositoryTreeQuery {
    #[serde(default = "default_ref")]
    pub ref_name: String,
    #[serde(default)]
    pub path: String,
}

#[derive(serde::Deserialize)]
pub struct RepositoryFileQuery {
    #[serde(default = "default_ref")]
    pub ref_name: String,
    pub path: String,
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
    for entry in subtree.iter() {
        let name = entry.name().unwrap_or("").to_string();
        let entry_type = match entry.kind() {
            Some(git2::ObjectType::Blob) => "blob",
            Some(git2::ObjectType::Tree) => "tree",
            Some(git2::ObjectType::Commit) => "commit", // for Git submodule
            _ => "unknown",
        }
        .to_string();
        let sha = entry.id().to_string();
        entries.push(RepositoryTreeEntry {
            name,
            entry_type,
            sha,
        });
    }

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

    let repository = open_repository(&settings, &owner, &repo)?;
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

fn default_ref() -> String {
    "HEAD".to_string()
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

fn is_binary(data: &[u8]) -> bool {
    data.iter().take(8000).any(|&b| b == 0)
}
