use std::time::Instant;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use tokio::{fs, task};

use crate::{
    dto::{
        InitialCommitFile, PathType, RepositoryBlobResponse, RepositoryBlobsResponse,
        RepositoryCommitResponse, RepositoryDiffFileResponse, RepositoryDiffStatResponse,
        RepositoryFileResponse, RepositoryFolderResponse, RepositoryPath, RepositoryPathsResponse,
    },
    error::GitError,
    util::{
        git::{DEFAULT_BRANCH, GitHookType, REPO_SUFFIX},
        review::MAGIC_REF_PREFIX,
    },
};

#[async_trait]
pub trait GitClient: Send + Sync + Clone + 'static {
    async fn repo_exists(&self, owner: &str, repo: &str) -> bool;

    async fn create_repo(&self, owner: &str, repo: &str) -> Result<(), GitError>;

    async fn delete_repo(&self, owner: &str, repo: &str) -> Result<(), GitError>;

    async fn mirror_repo(&self, owner: &str, repo: &str, url: &str) -> Result<(), GitError>;

    async fn get_default_ref(&self, owner: &str, repo: &str) -> Result<String, GitError>;

    async fn fetch_ref(
        &self,
        owner: &str,
        repo: &str,
        url: &str,
        ref_name: &str,
        sha: &str,
    ) -> Result<(), GitError>;

    async fn create_ref(
        &self,
        owner: &str,
        repo: &str,
        ref_name: &str,
        sha: &str,
    ) -> Result<(), GitError>;

    async fn update_ref(
        &self,
        owner: &str,
        repo: &str,
        ref_name: &str,
        sha: &str,
    ) -> Result<(), GitError>;

    async fn get_repo_blob(
        &self,
        owner: &str,
        repo: &str,
        ref_name: &str,
        path: &str,
    ) -> Result<RepositoryBlobResponse, GitError>;

    async fn get_repo_blobs(
        &self,
        owner: &str,
        repo: &str,
        ref_name: &str,
        paths: &[String],
    ) -> Result<RepositoryBlobsResponse, GitError>;

    async fn get_repo_blob_at_refs(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
        refs: &[String],
    ) -> Result<RepositoryBlobsResponse, GitError>;

    async fn get_repo_paths(
        &self,
        owner: &str,
        repo: &str,
        ref_name: &str,
    ) -> Result<RepositoryPathsResponse, GitError>;

    async fn get_repo_commit(
        &self,
        owner: &str,
        repo: &str,
        ref_name: &str,
    ) -> Result<RepositoryCommitResponse, GitError>;

    async fn get_repo_diff_files(
        &self,
        owner: &str,
        repo: &str,
        left_ref: Option<&str>,
        right_ref: &str,
    ) -> Result<Vec<RepositoryDiffFileResponse>, GitError>;

    async fn get_repo_diff_stats(
        &self,
        owner: &str,
        repo: &str,
        left_ref: Option<&str>,
        right_ref: &str,
    ) -> Result<Vec<RepositoryDiffStatResponse>, GitError>;

    async fn rev_list(
        &self,
        owner: &str,
        repo: &str,
        old_sha: &str,
        new_sha: &str,
    ) -> Result<Vec<RepositoryCommitResponse>, GitError>;

    async fn resolve_ref_sha(
        &self,
        owner: &str,
        repo: &str,
        ref_name: &str,
    ) -> Result<String, GitError>;

    async fn get_commit_patch_id(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
    ) -> Result<String, GitError>;

    async fn cherry_pick_commit(
        &self,
        owner: &str,
        repo: &str,
        commit_sha: &str,
        new_parent_sha: &str,
    ) -> Result<String, GitError>;

    async fn create_initial_commit(
        &self,
        owner: &str,
        repo: &str,
        files: Vec<InitialCommitFile>,
        author_name: &str,
        author_email: &str,
        committed_at: DateTime<Utc>,
    ) -> Result<String, GitError>;

    async fn install_hook(
        &self,
        owner: &str,
        repo: &str,
        hook_type: GitHookType,
        hook_script: &str,
    ) -> Result<(), GitError>;

    async fn empty_hooks(&self, owner: &str, repo: &str) -> Result<(), GitError>;

    fn normalize_repo_name(&self, repo: &str) -> String {
        format!(
            "{}{}",
            repo.strip_suffix(REPO_SUFFIX).unwrap_or(repo),
            REPO_SUFFIX
        )
    }
}

#[derive(Debug, Clone)]
pub struct Git2Client {
    project_root: String,
}

impl Git2Client {
    pub fn new(project_root: String) -> Self {
        Self { project_root }
    }

    fn get_owner_path(&self, owner: &str) -> String {
        format!("{}/{}", self.project_root, owner)
    }

    fn get_repo_path(&self, owner: &str, repo: &str) -> String {
        let repo = self.normalize_repo_name(repo);
        format!("{}/{}/{}", self.project_root, owner, repo)
    }

    fn open_repository(&self, owner: &str, repo: &str) -> Result<git2::Repository, git2::Error> {
        let repo_path = self.get_repo_path(owner, repo);
        git2::Repository::open_bare(&repo_path)
    }

    fn apply_repo_config(repo: &git2::Repository) -> Result<(), git2::Error> {
        let mut config = repo.config()?;

        // Configure the repository for HTTP access
        config.set_bool("http.receivepack", true)?;

        // Configure the magic ref to handle review creation via proc-receive hook
        config.set_str("receive.procReceiveRefs", MAGIC_REF_PREFIX)?;

        // Enable commit-graph to speed up ancestry/log/diff walks.
        config.set_bool("core.commitGraph", true)?;
        config.set_bool("gc.writeCommitGraph", true)?;
        config.set_bool("fetch.writeCommitGraph", true)?;
        Ok(())
    }

    fn resolve_ref<'repo>(
        repo: &'repo git2::Repository,
        ref_name: &str,
    ) -> Result<git2::Commit<'repo>, git2::Error> {
        let obj = repo.revparse_single(ref_name)?;
        obj.peel_to_commit()
    }

    fn diff_trees<'repo>(
        repo: &'repo git2::Repository,
        left_tree: &git2::Tree<'repo>,
        right_tree: &git2::Tree<'repo>,
    ) -> Result<git2::Diff<'repo>, git2::Error> {
        let mut diff_opts = git2::DiffOptions::new();
        diff_opts.skip_binary_check(true);
        let diff =
            repo.diff_tree_to_tree(Some(left_tree), Some(right_tree), Some(&mut diff_opts))?;
        Ok(diff)
    }

    fn get_blob<'repo>(
        repo: &'repo git2::Repository,
        tree: &git2::Tree<'repo>,
        path: &str,
    ) -> Result<git2::Blob<'repo>, git2::Error> {
        let tree_entry = tree.get_path(std::path::Path::new(path))?;

        if tree_entry.kind() != Some(git2::ObjectType::Blob) {
            return Err(git2::Error::from_str("Path is not a blob"));
        }

        repo.find_blob(tree_entry.id())
    }

    fn is_binary(data: &[u8]) -> bool {
        data.iter().take(8000).any(|&b| b == 0)
    }

    fn blob_to_response(blob: &git2::Blob, path: &str, commit_sha: &str) -> RepositoryFileResponse {
        let content_bytes = blob.content();
        let sha = blob.id().to_string();

        let (content, encoding) = if Self::is_binary(content_bytes) {
            use base64::prelude::*;
            (BASE64_STANDARD.encode(content_bytes), "base64".to_string())
        } else {
            (
                String::from_utf8_lossy(content_bytes).to_string(),
                "utf-8".to_string(),
            )
        };

        RepositoryFileResponse {
            commit_sha: commit_sha.to_string(),
            path: path.to_string(),
            sha,
            content,
            encoding,
        }
    }

    fn blob_content_string(blob: &git2::Blob) -> String {
        let bytes = blob.content();
        if Self::is_binary(bytes) {
            use base64::prelude::*;
            BASE64_STANDARD.encode(bytes)
        } else {
            String::from_utf8_lossy(bytes).to_string()
        }
    }

    fn tree_to_response(
        tree: &git2::Tree,
        path: &str,
        commit_sha: &str,
    ) -> RepositoryFolderResponse {
        let entries = tree
            .iter()
            .map(|e| {
                let name = e.name().unwrap_or("").to_string();
                let entry_path = format!("{}/{}", path, name);
                RepositoryPath {
                    path: entry_path,
                    name,
                    path_type: PathType::from_git2(e.kind()),
                    sha: e.id().to_string(),
                }
            })
            .collect();

        RepositoryFolderResponse {
            commit_sha: commit_sha.to_string(),
            path: path.to_string(),
            entries,
        }
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl GitClient for Git2Client {
    async fn repo_exists(&self, owner: &str, repo: &str) -> bool {
        let repo_path = self.get_repo_path(owner, repo);
        match fs::metadata(&repo_path).await {
            Ok(metadata) => metadata.is_dir(),
            Err(_) => false,
        }
    }

    async fn create_repo(&self, owner: &str, repo: &str) -> Result<(), GitError> {
        let owner_path = self.get_owner_path(owner);
        fs::create_dir_all(&owner_path).await?;

        let repo_path = self.get_repo_path(owner, repo);
        let repo_path_clone = repo_path.clone();
        task::spawn_blocking(move || -> Result<(), git2::Error> {
            let repo = git2::Repository::init_bare(&repo_path_clone)?;
            repo.set_head(&format!("refs/heads/{}", DEFAULT_BRANCH))?;
            Self::apply_repo_config(&repo)?;
            Ok(())
        })
        .await??;

        // Create git-daemon-export-ok file to allow HTTP access
        let export_ok_path = format!("{}/git-daemon-export-ok", repo_path);
        fs::write(&export_ok_path, "").await?;

        Ok(())
    }

    async fn delete_repo(&self, owner: &str, repo: &str) -> Result<(), GitError> {
        let repo_path = self.get_repo_path(owner, repo);
        fs::remove_dir_all(&repo_path).await?;
        Ok(())
    }

    async fn mirror_repo(&self, owner: &str, repo: &str, url: &str) -> Result<(), GitError> {
        let owner_path = self.get_owner_path(owner);
        fs::create_dir_all(&owner_path).await?;

        let repo_path = self.get_repo_path(owner, repo);
        let output = tokio::process::Command::new("git")
            .arg("clone")
            .arg("--mirror")
            .arg(&url)
            .arg(&repo_path)
            .output()
            .await?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(GitError::Git2Error(git2::Error::from_str(&format!(
                "git clone failed: {}",
                stderr
            ))));
        }

        // Scrub the tokenized remote URL written into .git/config by `git clone`.
        // The clone embedded a 1h GitHub installation token in remote.origin.url;
        // dropping the entire `origin` remote prevents on-disk token persistence.
        // Incremental syncs use `fetch_ref` with a URL supplied per call, so no
        // caller relies on `origin` afterwards.
        let remove = tokio::process::Command::new("git")
            .arg("-C")
            .arg(&repo_path)
            .arg("remote")
            .arg("remove")
            .arg("origin")
            .output()
            .await?;
        if !remove.status.success() {
            tracing::warn!(
                owner = %owner,
                repo = %repo,
                stderr = %String::from_utf8_lossy(&remove.stderr),
                "post-clone `git remote remove origin` failed; token may persist in .git/config",
            );
        }

        let repo_path_clone = repo_path.clone();
        task::spawn_blocking(move || -> Result<(), git2::Error> {
            let repo = git2::Repository::open_bare(&repo_path_clone)?;
            Self::apply_repo_config(&repo)?;
            Ok(())
        })
        .await??;

        // Materialize the commit-graph once from the freshly-imported history.
        // `git clone --mirror` does not trigger `fetch.writeCommitGraph`, so
        // without this the file won't exist until the next incremental fetch
        // or maintenance run. Non-fatal: a missing graph just falls back to
        // the slower walk and will be built on next fetch.
        let graph_write = tokio::process::Command::new("git")
            .arg("-C")
            .arg(&repo_path)
            .arg("commit-graph")
            .arg("write")
            .arg("--reachable")
            .arg("--split")
            .output()
            .await?;
        if !graph_write.status.success() {
            tracing::warn!(
                owner = %owner,
                repo = %repo,
                stderr = %String::from_utf8_lossy(&graph_write.stderr),
                "initial `git commit-graph write` failed; graph will be built on next fetch",
            );
        }

        // Create git-daemon-export-ok file to allow HTTP access
        let export_ok_path = format!("{}/git-daemon-export-ok", repo_path);
        fs::write(&export_ok_path, "").await?;

        Ok(())
    }

    async fn get_default_ref(&self, owner: &str, repo: &str) -> Result<String, GitError> {
        let repo_path = self.get_repo_path(owner, repo);

        let output = tokio::process::Command::new("git")
            .arg("-C")
            .arg(&repo_path)
            .arg("symbolic-ref")
            .arg("HEAD")
            .output()
            .await?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(GitError::Git2Error(git2::Error::from_str(&format!(
                "git symbolic-ref HEAD failed: {}",
                stderr
            ))));
        }
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    async fn fetch_ref(
        &self,
        owner: &str,
        repo: &str,
        url: &str,
        ref_name: &str,
        sha: &str,
    ) -> Result<(), GitError> {
        let repo_path = self.get_repo_path(owner, repo);
        let refspec = format!("+{}:{}", sha, ref_name);

        let output = tokio::process::Command::new("git")
            .arg("-C")
            .arg(&repo_path)
            .arg("fetch")
            .arg(url)
            .arg(&refspec)
            .output()
            .await?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(GitError::Git2Error(git2::Error::from_str(&format!(
                "git fetch failed: {}",
                stderr
            ))));
        }
        Ok(())
    }

    async fn create_ref(
        &self,
        owner: &str,
        repo: &str,
        ref_name: &str,
        sha: &str,
    ) -> Result<(), GitError> {
        let ref_name = ref_name.to_string();
        let sha = sha.to_string();
        let repository = self.open_repository(owner, repo)?;

        task::spawn_blocking(move || {
            let oid = git2::Oid::from_str(&sha)?;
            let commit = repository.find_commit(oid)?;
            repository.reference(&ref_name, commit.id(), false, "create_ref")?;
            Ok(())
        })
        .await?
    }

    async fn update_ref(
        &self,
        owner: &str,
        repo: &str,
        ref_name: &str,
        sha: &str,
    ) -> Result<(), GitError> {
        let ref_name = ref_name.to_string();
        let sha = sha.to_string();
        let repository = self.open_repository(owner, repo)?;

        task::spawn_blocking(move || {
            let oid = git2::Oid::from_str(&sha)?;
            let commit = repository.find_commit(oid)?;
            repository.reference(&ref_name, commit.id(), true, "update_ref")?;
            Ok(())
        })
        .await?
    }

    async fn get_repo_blob(
        &self,
        owner: &str,
        repo: &str,
        ref_name: &str,
        path: &str,
    ) -> Result<RepositoryBlobResponse, GitError> {
        let ref_name = ref_name.to_string();
        let path = path.to_string();
        let repository = self.open_repository(owner, repo)?;

        task::spawn_blocking(move || {
            let commit = Self::resolve_ref(&repository, &ref_name)?;
            let commit_sha = commit.id().to_string();
            let tree = commit.tree()?;

            let tree_entry = tree.get_path(std::path::Path::new(&path))?;

            match tree_entry.kind() {
                Some(git2::ObjectType::Blob) => {
                    let blob = repository.find_blob(tree_entry.id())?;
                    Ok(RepositoryBlobResponse::File(Self::blob_to_response(
                        &blob,
                        &path,
                        &commit_sha,
                    )))
                }
                Some(git2::ObjectType::Tree) => {
                    let subtree = repository.find_tree(tree_entry.id())?;
                    Ok(RepositoryBlobResponse::Folder(Self::tree_to_response(
                        &subtree,
                        &path,
                        &commit_sha,
                    )))
                }
                _ => Err(git2::Error::from_str("Path is not a blob or tree").into()),
            }
        })
        .await?
    }

    async fn get_repo_blobs(
        &self,
        owner: &str,
        repo: &str,
        ref_name: &str,
        paths: &[String],
    ) -> Result<RepositoryBlobsResponse, GitError> {
        let ref_name = ref_name.to_string();
        let paths = paths.to_vec();
        let repository = self.open_repository(owner, repo)?;

        task::spawn_blocking(move || {
            let commit = Self::resolve_ref(&repository, &ref_name)?;
            let commit_sha = commit.id().to_string();
            let tree = commit.tree()?;

            let mut blobs = Vec::new();

            for path in &paths {
                let tree_entry = match tree.get_path(std::path::Path::new(path)) {
                    Ok(e) => e,
                    Err(_) => continue,
                };

                match tree_entry.kind() {
                    Some(git2::ObjectType::Blob) => {
                        let blob = repository.find_blob(tree_entry.id())?;
                        blobs.push(RepositoryBlobResponse::File(Self::blob_to_response(
                            &blob,
                            path,
                            &commit_sha,
                        )));
                    }
                    Some(git2::ObjectType::Tree) => {
                        let subtree = repository.find_tree(tree_entry.id())?;
                        let entries = subtree
                            .iter()
                            .map(|e| {
                                let name = e.name().unwrap_or("").to_string();
                                let entry_path = format!("{}/{}", path, name);
                                RepositoryPath {
                                    path: entry_path,
                                    name,
                                    path_type: PathType::from_git2(e.kind()),
                                    sha: e.id().to_string(),
                                }
                            })
                            .collect();
                        blobs.push(RepositoryBlobResponse::Folder(RepositoryFolderResponse {
                            commit_sha: commit_sha.clone(),
                            path: path.clone(),
                            entries,
                        }));
                    }
                    _ => continue,
                }
            }

            Ok(RepositoryBlobsResponse { blobs })
        })
        .await?
    }

    async fn get_repo_blob_at_refs(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
        refs: &[String],
    ) -> Result<RepositoryBlobsResponse, GitError> {
        let path = path.to_string();
        let refs = refs.to_vec();
        let repository = self.open_repository(owner, repo)?;

        task::spawn_blocking(move || {
            let mut blobs = Vec::new();

            for ref_name in &refs {
                let commit = match Self::resolve_ref(&repository, ref_name).map_err(GitError::from)
                {
                    Ok(c) => c,
                    Err(GitError::NotFound(_)) => continue,
                    Err(e) => return Err(e),
                };
                let commit_sha = commit.id().to_string();
                let tree = commit.tree()?;

                let tree_entry = match tree.get_path(std::path::Path::new(&path)) {
                    Ok(e) => e,
                    Err(_) => continue,
                };

                match tree_entry.kind() {
                    Some(git2::ObjectType::Blob) => {
                        let blob = repository.find_blob(tree_entry.id())?;
                        blobs.push(RepositoryBlobResponse::File(Self::blob_to_response(
                            &blob,
                            &path,
                            &commit_sha,
                        )));
                    }
                    Some(git2::ObjectType::Tree) => {
                        let subtree = repository.find_tree(tree_entry.id())?;
                        blobs.push(RepositoryBlobResponse::Folder(Self::tree_to_response(
                            &subtree,
                            &path,
                            &commit_sha,
                        )));
                    }
                    _ => continue,
                }
            }

            Ok(RepositoryBlobsResponse { blobs })
        })
        .await?
    }

    async fn get_repo_paths(
        &self,
        owner: &str,
        repo: &str,
        ref_name: &str,
    ) -> Result<RepositoryPathsResponse, GitError> {
        let ref_name = ref_name.to_string();
        let repository = self.open_repository(owner, repo)?;

        task::spawn_blocking(move || {
            let commit = Self::resolve_ref(&repository, &ref_name)?;
            let commit_sha = commit.id().to_string();
            let tree = commit.tree()?;
            let mut entries = Vec::new();
            tree.walk(git2::TreeWalkMode::PreOrder, |root, entry| {
                let name = entry.name().unwrap_or("").to_string();
                let path = if root.is_empty() {
                    name.clone()
                } else {
                    format!("{}{}", root, name)
                };
                entries.push(RepositoryPath {
                    path,
                    name,
                    path_type: PathType::from_git2(entry.kind()),
                    sha: entry.id().to_string(),
                });
                git2::TreeWalkResult::Ok
            })?;
            Ok(RepositoryPathsResponse {
                ref_name,
                commit_sha,
                entries,
            })
        })
        .await?
    }

    async fn get_repo_commit(
        &self,
        owner: &str,
        repo: &str,
        ref_name: &str,
    ) -> Result<RepositoryCommitResponse, GitError> {
        let ref_name = ref_name.to_string();
        let repository = self.open_repository(owner, repo)?;

        task::spawn_blocking(move || {
            let commit = Self::resolve_ref(&repository, &ref_name)?;

            Ok(RepositoryCommitResponse::from(&commit))
        })
        .await?
    }

    async fn get_repo_diff_files(
        &self,
        owner: &str,
        repo: &str,
        left_ref: Option<&str>,
        right_ref: &str,
    ) -> Result<Vec<RepositoryDiffFileResponse>, GitError> {
        let owner = owner.to_string();
        let repo = repo.to_string();
        let left_ref = left_ref.map(str::to_string);
        let right_ref = right_ref.to_string();

        let open_start = Instant::now();
        let repository = self.open_repository(&owner, &repo)?;
        let open_ms = open_start.elapsed().as_millis() as u64;

        let owner_log = owner.clone();
        let repo_log = repo.clone();
        let right_ref_log = right_ref.clone();

        task::spawn_blocking(move || {
            let trees_start = Instant::now();
            let left_tree = match left_ref {
                None => {
                    let empty_oid = repository.treebuilder(None)?.write()?;
                    repository.find_tree(empty_oid)?
                }
                Some(ref r) => Self::resolve_ref(&repository, r)?.tree()?,
            };
            let right_tree = Self::resolve_ref(&repository, &right_ref)?.tree()?;
            let trees_ms = trees_start.elapsed().as_millis() as u64;

            let diff_start = Instant::now();
            let diff = Self::diff_trees(&repository, &left_tree, &right_tree)?;
            let num_deltas = diff.deltas().count();
            let diff_ms = diff_start.elapsed().as_millis() as u64;

            let loop_start = Instant::now();
            let mut blob_ms: u64 = 0;
            let mut patch_ms: u64 = 0;
            let mut total_left_bytes: u64 = 0;
            let mut total_right_bytes: u64 = 0;
            let mut results = Vec::with_capacity(num_deltas);

            for i in 0..num_deltas {
                let delta = diff
                    .get_delta(i)
                    .ok_or_else(|| git2::Error::from_str("delta index out of range"))?;
                let status = delta.status();

                let blob_start = Instant::now();
                let left_content = if status != git2::Delta::Added {
                    delta
                        .old_file()
                        .path()
                        .and_then(|p| p.to_str())
                        .and_then(|path| {
                            let blob = Self::get_blob(&repository, &left_tree, path).ok()?;
                            let content = Self::blob_content_string(&blob);
                            total_left_bytes += content.len() as u64;
                            Some(content)
                        })
                } else {
                    None
                };

                let right_content = if status != git2::Delta::Deleted {
                    delta
                        .new_file()
                        .path()
                        .and_then(|p| p.to_str())
                        .and_then(|path| {
                            let blob = Self::get_blob(&repository, &right_tree, path).ok()?;
                            let content = Self::blob_content_string(&blob);
                            total_right_bytes += content.len() as u64;
                            Some(content)
                        })
                } else {
                    None
                };
                blob_ms += blob_start.elapsed().as_millis() as u64;

                let path = delta
                    .new_file()
                    .path()
                    .or_else(|| delta.old_file().path())
                    .and_then(|p| p.to_str())
                    .unwrap_or("")
                    .to_string();

                let patch_start = Instant::now();
                let patch = git2::Patch::from_diff(&diff, i)?;
                let (_, insertions, deletions) =
                    patch.map(|p| p.line_stats()).unwrap_or(Ok((0, 0, 0)))?;
                patch_ms += patch_start.elapsed().as_millis() as u64;

                results.push(RepositoryDiffFileResponse {
                    path,
                    left_content,
                    right_content,
                    lines_added: insertions as u32,
                    lines_removed: deletions as u32,
                });
            }
            let loop_ms = loop_start.elapsed().as_millis() as u64;

            tracing::error!(
                owner = %owner_log,
                repo = %repo_log,
                right_ref = %right_ref_log,
                num_deltas,
                open_ms,
                trees_ms,
                diff_ms,
                loop_ms,
                blob_ms,
                patch_ms,
                total_left_bytes,
                total_right_bytes,
                "get_repo_diff_files stage timings"
            );

            Ok(results)
        })
        .await?
    }

    async fn get_repo_diff_stats(
        &self,
        owner: &str,
        repo: &str,
        left_ref: Option<&str>,
        right_ref: &str,
    ) -> Result<Vec<RepositoryDiffStatResponse>, GitError> {
        let owner = owner.to_string();
        let repo = repo.to_string();
        let left_ref = left_ref.map(str::to_string);
        let right_ref = right_ref.to_string();
        let repository = self.open_repository(&owner, &repo)?;

        task::spawn_blocking(move || {
            let left_tree = match left_ref {
                None => {
                    let empty_oid = repository.treebuilder(None)?.write()?;
                    repository.find_tree(empty_oid)?
                }
                Some(ref r) => Self::resolve_ref(&repository, r)?.tree()?,
            };
            let right_tree = Self::resolve_ref(&repository, &right_ref)?.tree()?;
            let diff = Self::diff_trees(&repository, &left_tree, &right_tree)?;

            let num_deltas = diff.deltas().count();
            let mut results = Vec::new();
            for i in 0..num_deltas {
                let delta = diff
                    .get_delta(i)
                    .ok_or_else(|| git2::Error::from_str("delta index out of range"))?;
                let path = delta
                    .new_file()
                    .path()
                    .or_else(|| delta.old_file().path())
                    .and_then(|p| p.to_str())
                    .unwrap_or("")
                    .to_string();
                let patch = git2::Patch::from_diff(&diff, i)?;
                let (_, insertions, deletions) =
                    patch.map(|p| p.line_stats()).unwrap_or(Ok((0, 0, 0)))?;
                results.push(RepositoryDiffStatResponse {
                    path,
                    lines_added: insertions as u32,
                    lines_removed: deletions as u32,
                });
            }

            Ok(results)
        })
        .await?
    }

    async fn rev_list(
        &self,
        owner: &str,
        repo: &str,
        old_sha: &str,
        new_sha: &str,
    ) -> Result<Vec<RepositoryCommitResponse>, GitError> {
        let old_sha = old_sha.to_string();
        let new_sha = new_sha.to_string();
        let repository = self.open_repository(owner, repo)?;

        task::spawn_blocking(move || {
            let new_oid = git2::Oid::from_str(&new_sha)?;

            let mut revwalk = repository.revwalk()?;
            revwalk.push(new_oid)?;
            revwalk.set_sorting(git2::Sort::TIME)?;

            // For non-initial pushes, hide everything reachable from old_sha
            let is_initial = old_sha.chars().all(|c| c == '0');
            if !is_initial {
                let old_oid = git2::Oid::from_str(&old_sha)?;
                revwalk.hide(old_oid)?;
            }

            let mut commits = Vec::new();
            for oid_result in revwalk {
                let oid = oid_result?;
                let commit = repository.find_commit(oid)?;
                commits.push(RepositoryCommitResponse::from(&commit));
            }

            Ok(commits)
        })
        .await?
    }

    async fn resolve_ref_sha(
        &self,
        owner: &str,
        repo: &str,
        ref_name: &str,
    ) -> Result<String, GitError> {
        let ref_name = ref_name.to_string();
        let repository = self.open_repository(owner, repo)?;

        task::spawn_blocking(move || {
            let commit = Self::resolve_ref(&repository, &ref_name)?;
            Ok(commit.id().to_string())
        })
        .await?
    }

    async fn get_commit_patch_id(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
    ) -> Result<String, GitError> {
        let sha = sha.to_string();
        let repository = self.open_repository(owner, repo)?;

        task::spawn_blocking(move || {
            let oid = git2::Oid::from_str(&sha)?;
            let commit = repository.find_commit(oid)?;
            let tree = commit.tree()?;

            let parent_tree = if commit.parent_count() > 0 {
                Some(commit.parent(0)?.tree()?)
            } else {
                None
            };

            let mut diff_opts = git2::DiffOptions::new();
            let diff = repository.diff_tree_to_tree(
                parent_tree.as_ref(),
                Some(&tree),
                Some(&mut diff_opts),
            )?;

            let patch_bytes = std::cell::RefCell::new(Vec::<u8>::new());
            diff.foreach(
                &mut |delta, _progress| {
                    let mut bytes = patch_bytes.borrow_mut();
                    if let Some(path) = delta.old_file().path() {
                        bytes.extend_from_slice(path.to_string_lossy().as_bytes());
                    }
                    if let Some(path) = delta.new_file().path() {
                        bytes.extend_from_slice(path.to_string_lossy().as_bytes());
                    }
                    bytes.push(delta.status() as u8);
                    true
                },
                None,
                None,
                Some(&mut |_delta, _hunk, line| {
                    match line.origin() {
                        '+' | '-' => {
                            let mut bytes = patch_bytes.borrow_mut();
                            bytes.push(line.origin() as u8);
                            bytes.extend_from_slice(line.content());
                        }
                        _ => {}
                    }
                    true
                }),
            )?;

            let patch_bytes = patch_bytes.into_inner();

            let patch_id = git2::Oid::hash_object(git2::ObjectType::Blob, &patch_bytes)?;
            Ok(patch_id.to_string())
        })
        .await?
    }

    async fn cherry_pick_commit(
        &self,
        owner: &str,
        repo: &str,
        commit_sha: &str,
        new_parent_sha: &str,
    ) -> Result<String, GitError> {
        let commit_sha = commit_sha.to_string();
        let new_parent_sha = new_parent_sha.to_string();
        let repository = self.open_repository(owner, repo)?;

        task::spawn_blocking(move || {
            let commit_oid = git2::Oid::from_str(&commit_sha)?;
            let parent_oid = git2::Oid::from_str(&new_parent_sha)?;

            let commit = repository.find_commit(commit_oid)?;
            let new_parent = repository.find_commit(parent_oid)?;

            let mut index = repository.cherrypick_commit(&commit, &new_parent, 0, None)?;

            if index.has_conflicts() {
                return Err(GitError::MergeConflict(format!(
                    "conflict when rebasing commit {}",
                    &commit_sha[..8]
                )));
            }

            let tree_oid = index.write_tree_to(&repository)?;
            let tree = repository.find_tree(tree_oid)?;

            let new_oid = repository.commit(
                None,
                &commit.author(),
                &commit.committer(),
                commit.message().unwrap_or(""),
                &tree,
                &[&new_parent],
            )?;

            Ok(new_oid.to_string())
        })
        .await?
    }

    async fn create_initial_commit(
        &self,
        owner: &str,
        repo: &str,
        files: Vec<InitialCommitFile>,
        author_name: &str,
        author_email: &str,
        committed_at: DateTime<Utc>,
    ) -> Result<String, GitError> {
        let repo_path = self.get_repo_path(owner, repo);
        let author_name = author_name.to_string();
        let author_email = author_email.to_string();
        let when = git2::Time::new(committed_at.timestamp(), 0);

        let sha = task::spawn_blocking(move || -> Result<String, git2::Error> {
            let repo = git2::Repository::open_bare(&repo_path)?;

            let mut tree_builder = repo.treebuilder(None)?;
            for file in &files {
                let blob_oid = repo.blob(file.content.as_bytes())?;
                tree_builder.insert(file.path, blob_oid, git2::FileMode::Blob.into())?;
            }
            let tree_oid = tree_builder.write()?;
            let tree = repo.find_tree(tree_oid)?;

            let sig = git2::Signature::new(&author_name, &author_email, &when)?;
            let ref_name = format!("refs/heads/{}", DEFAULT_BRANCH);
            let oid = repo.commit(Some(&ref_name), &sig, &sig, "Initial commit", &tree, &[])?;

            Ok(oid.to_string())
        })
        .await??;

        Ok(sha)
    }

    async fn install_hook(
        &self,
        owner: &str,
        repo: &str,
        hook_type: GitHookType,
        hook_script: &str,
    ) -> Result<(), GitError> {
        let repo_path = self.get_repo_path(owner, repo);
        let hook_path = format!("{}/hooks/{}", repo_path, hook_type.as_str());
        fs::write(&hook_path, hook_script).await?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let perms = std::fs::Permissions::from_mode(0o755);
            fs::set_permissions(&hook_path, perms).await?;
        }

        Ok(())
    }

    async fn empty_hooks(&self, owner: &str, repo: &str) -> Result<(), GitError> {
        let repo_path = self.get_repo_path(owner, repo);
        let hooks_dir = format!("{}/hooks", repo_path);

        let mut entries = fs::read_dir(&hooks_dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_file() {
                fs::remove_file(&path).await?;
            }
        }

        Ok(())
    }
}
