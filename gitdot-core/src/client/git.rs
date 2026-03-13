use async_trait::async_trait;
use tokio::{fs, task};

use crate::{
    dto::{
        FilePreview, PathType, RepositoryBlobResponse, RepositoryBlobsResponse,
        RepositoryCommitResponse, RepositoryCommitStatResponse, RepositoryCommitsResponse,
        RepositoryFileResponse, RepositoryFolderResponse, RepositoryPath, RepositoryPathsResponse,
        RepositoryPreviewEntry, RepositoryPreviewResponse,
    },
    error::GitError,
    util::{
        git::{DEFAULT_BRANCH, EMPTY_TREE_REF, GitHookType, REPO_SUFFIX},
        review::MAGIC_REF_PREFIX,
    },
};

#[async_trait]
pub trait GitClient: Send + Sync + Clone + 'static {
    async fn repo_exists(&self, owner: &str, repo: &str) -> bool;

    async fn create_repo(&self, owner: &str, repo: &str) -> Result<(), GitError>;

    async fn delete_repo(&self, owner: &str, repo: &str) -> Result<(), GitError>;

    async fn mirror_repo(&self, owner: &str, repo: &str, url: &str) -> Result<(), GitError>;

    async fn create_ref(
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

    async fn get_repo_commits(
        &self,
        owner: &str,
        repo: &str,
        ref_name: &str,
        page: u32,
        per_page: u32,
    ) -> Result<RepositoryCommitsResponse, GitError>;

    async fn get_repo_file_commits(
        &self,
        owner: &str,
        repo: &str,
        ref_name: &str,
        path: &str,
        page: u32,
        per_page: u32,
    ) -> Result<RepositoryCommitsResponse, GitError>;

    async fn get_repo_preview(
        &self,
        owner: &str,
        repo: &str,
        ref_name: &str,
        preview_lines: u32,
    ) -> Result<RepositoryPreviewResponse, GitError>;

    async fn get_repo_diff_files(
        &self,
        owner: &str,
        repo: &str,
        left_ref: &str,
        right_ref: &str,
    ) -> Result<
        Vec<(
            Option<RepositoryFileResponse>,
            Option<RepositoryFileResponse>,
        )>,
        GitError,
    >;

    async fn get_repo_diff_stats(
        &self,
        owner: &str,
        repo: &str,
        left_ref: &str,
        right_ref: &str,
    ) -> Result<Vec<RepositoryCommitStatResponse>, GitError>;

    async fn rev_list(
        &self,
        owner: &str,
        repo: &str,
        old_sha: &str,
        new_sha: &str,
    ) -> Result<Vec<RepositoryCommitResponse>, GitError>;

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
        let mut diff =
            repo.diff_tree_to_tree(Some(left_tree), Some(right_tree), Some(&mut diff_opts))?;

        let mut find_opts = git2::DiffFindOptions::new();
        find_opts.renames(true);
        find_opts.rename_threshold(50);
        diff.find_similar(Some(&mut find_opts))?;

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

    fn blob_to_response(blob: &git2::Blob, path: &str) -> RepositoryFileResponse {
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
            path: path.to_string(),
            sha,
            content,
            encoding,
        }
    }

    fn get_file_preview(
        repo: &git2::Repository,
        blob_id: git2::Oid,
        preview_lines: u32,
    ) -> Option<FilePreview> {
        let blob = repo.find_blob(blob_id).ok()?;
        let content_bytes = blob.content();

        if Self::is_binary(content_bytes) {
            return None;
        }

        let content_str = std::str::from_utf8(content_bytes).ok()?;
        let lines: Vec<&str> = content_str.lines().collect();
        let total_lines = lines.len() as u32;
        let preview_line_count = std::cmp::min(preview_lines, total_lines);
        let truncated = total_lines > preview_lines;

        let preview_content = lines
            .into_iter()
            .take(preview_lines as usize)
            .collect::<Vec<_>>()
            .join("\n");

        Some(FilePreview {
            content: preview_content,
            total_lines,
            preview_lines: preview_line_count,
            truncated,
            encoding: "utf-8".to_string(),
        })
    }

    fn walk_tree_with_preview(
        repo: &git2::Repository,
        tree: &git2::Tree,
        base_path: &str,
        entries: &mut Vec<RepositoryPreviewEntry>,
        preview_lines: u32,
    ) -> Result<(), git2::Error> {
        for entry in tree.iter() {
            let name = entry.name().unwrap_or("").to_string();
            let entry_path = if base_path.is_empty() {
                name.clone()
            } else {
                format!("{}/{}", base_path, name)
            };

            match entry.kind() {
                Some(git2::ObjectType::Blob) => {
                    let sha = entry.id().to_string();
                    let preview = Self::get_file_preview(repo, entry.id(), preview_lines);

                    entries.push(RepositoryPreviewEntry {
                        path: entry_path,
                        name,
                        sha,
                        preview,
                    });
                }
                Some(git2::ObjectType::Tree) => {
                    if let Ok(subtree) = repo.find_tree(entry.id()) {
                        Self::walk_tree_with_preview(
                            repo,
                            &subtree,
                            &entry_path,
                            entries,
                            preview_lines,
                        )?;
                    }
                }
                _ => {}
            }
        }
        Ok(())
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
            let mut config = repo.config()?;

            // Configure the repository for HTTP access
            config.set_bool("http.receivepack", true)?;

            // Configure the magic ref to handle review creation via proc-receive hook
            config.set_str("receive.procReceiveRefs", MAGIC_REF_PREFIX)?;

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

        let repo_path_clone = repo_path.clone();
        task::spawn_blocking(move || -> Result<(), git2::Error> {
            let repo = git2::Repository::open_bare(&repo_path_clone)?;
            let mut config = repo.config()?;

            // Configure the repository for HTTP access
            config.set_bool("http.receivepack", true)?;

            // Configure the magic ref to handle review creation via proc-receive hook
            config.set_str("receive.procReceiveRefs", MAGIC_REF_PREFIX)?;
            Ok(())
        })
        .await??;

        // Create git-daemon-export-ok file to allow HTTP access
        let export_ok_path = format!("{}/git-daemon-export-ok", repo_path);
        fs::write(&export_ok_path, "").await?;

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
            let tree = commit.tree()?;

            let tree_entry = tree.get_path(std::path::Path::new(&path))?;

            match tree_entry.kind() {
                Some(git2::ObjectType::Blob) => {
                    let blob = repository.find_blob(tree_entry.id())?;
                    Ok(RepositoryBlobResponse::File(Self::blob_to_response(
                        &blob, &path,
                    )))
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
                    Ok(RepositoryBlobResponse::Folder(RepositoryFolderResponse {
                        path,
                        entries,
                    }))
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
                            &blob, path,
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
                            path: path.clone(),
                            entries,
                        }));
                    }
                    _ => continue,
                }
            }

            Ok(RepositoryBlobsResponse {
                ref_name,
                commit_sha,
                blobs,
            })
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

    async fn get_repo_commits(
        &self,
        owner: &str,
        repo: &str,
        ref_name: &str,
        page: u32,
        per_page: u32,
    ) -> Result<RepositoryCommitsResponse, GitError> {
        let ref_name = ref_name.to_string();
        let repository = self.open_repository(owner, repo)?;

        let skip = ((page.saturating_sub(1)) * per_page) as usize;
        let take = per_page as usize;

        task::spawn_blocking(move || {
            let commit = Self::resolve_ref(&repository, &ref_name)?;

            let mut revwalk = repository.revwalk()?;
            revwalk.push(commit.id())?;
            revwalk.set_sorting(git2::Sort::TIME).ok();

            let commits: Result<Vec<RepositoryCommitResponse>, git2::Error> = revwalk
                .skip(skip)
                .take(take + 1)
                .map(|oid| {
                    let oid = oid?;
                    let commit = repository.find_commit(oid)?;
                    Ok(RepositoryCommitResponse::from(&commit))
                })
                .collect();

            let mut commits = commits?;
            let has_next = commits.len() > take;

            if has_next {
                commits.pop();
            }

            Ok(RepositoryCommitsResponse { commits, has_next })
        })
        .await?
    }

    async fn get_repo_file_commits(
        &self,
        owner: &str,
        repo: &str,
        ref_name: &str,
        path: &str,
        page: u32,
        per_page: u32,
    ) -> Result<RepositoryCommitsResponse, GitError> {
        let ref_name = ref_name.to_string();
        let path = path.to_string();
        let repository = self.open_repository(owner, repo)?;

        let skip = ((page.saturating_sub(1)) * per_page) as usize;
        let take = per_page as usize;

        task::spawn_blocking(move || {
            let start_commit = Self::resolve_ref(&repository, &ref_name)?;

            let mut revwalk = repository.revwalk()?;
            revwalk.push(start_commit.id())?;
            revwalk.set_sorting(git2::Sort::TIME).ok();

            let mut current_path = path;
            let mut commits = Vec::new();
            let mut count = 0;

            for oid_result in revwalk {
                let oid = oid_result?;
                let commit = repository.find_commit(oid)?;

                let mut modified = false;

                if commit.parent_count() == 0 {
                    // Initial commit - check if file exists
                    let tree = commit.tree()?;
                    if tree.get_path(std::path::Path::new(&current_path)).is_ok() {
                        modified = true;
                    }
                } else {
                    // Check diff against parent
                    for parent in commit.parents() {
                        let parent_tree = parent.tree()?;
                        let commit_tree = commit.tree()?;

                        let mut diff_opts = git2::DiffOptions::new();
                        diff_opts.pathspec(&current_path);

                        let diff = repository.diff_tree_to_tree(
                            Some(&parent_tree),
                            Some(&commit_tree),
                            Some(&mut diff_opts),
                        )?;

                        // Check if diff contains our path
                        let path_modified = diff.deltas().any(|delta| {
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
                        });

                        if path_modified {
                            modified = true;

                            // Check for renames and update path for next iteration
                            for delta in diff.deltas() {
                                if delta.status() == git2::Delta::Renamed {
                                    if let Some(new_path) = delta.new_file().path() {
                                        if new_path.to_str() == Some(&current_path) {
                                            if let Some(old_path) = delta.old_file().path() {
                                                if let Some(old_path_str) = old_path.to_str() {
                                                    current_path = old_path_str.to_string();
                                                }
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
                        commits.push(RepositoryCommitResponse::from(&commit));
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

            Ok(RepositoryCommitsResponse { commits, has_next })
        })
        .await?
    }

    async fn get_repo_preview(
        &self,
        owner: &str,
        repo: &str,
        ref_name: &str,
        preview_lines: u32,
    ) -> Result<RepositoryPreviewResponse, GitError> {
        let ref_name = ref_name.to_string();
        let repository = self.open_repository(owner, repo)?;

        task::spawn_blocking(move || {
            let commit = Self::resolve_ref(&repository, &ref_name)?;
            let commit_sha = commit.id().to_string();
            let tree = commit.tree()?;

            let mut entries = Vec::new();
            Self::walk_tree_with_preview(&repository, &tree, "", &mut entries, preview_lines)?;

            Ok(RepositoryPreviewResponse {
                ref_name,
                commit_sha,
                entries,
            })
        })
        .await?
    }

    async fn get_repo_diff_files(
        &self,
        owner: &str,
        repo: &str,
        left_ref: &str,
        right_ref: &str,
    ) -> Result<
        Vec<(
            Option<RepositoryFileResponse>,
            Option<RepositoryFileResponse>,
        )>,
        GitError,
    > {
        let owner = owner.to_string();
        let repo = repo.to_string();
        let left_ref = left_ref.to_string();
        let right_ref = right_ref.to_string();
        let repository = self.open_repository(&owner, &repo)?;

        task::spawn_blocking(move || {
            let (left_tree, _left_sha) = if left_ref == EMPTY_TREE_REF {
                let empty_oid = repository.treebuilder(None)?.write()?;
                (repository.find_tree(empty_oid)?, EMPTY_TREE_REF.to_string())
            } else {
                let left_commit = Self::resolve_ref(&repository, &left_ref)?;
                let sha = left_commit.id().to_string();
                (left_commit.tree()?, sha)
            };

            let right_commit = Self::resolve_ref(&repository, &right_ref)?;
            let right_tree = right_commit.tree()?;
            let _right_sha = right_commit.id().to_string();

            let diff = Self::diff_trees(&repository, &left_tree, &right_tree)?;

            let mut results = Vec::new();

            for delta in diff.deltas() {
                let status = delta.status();

                let left = if status != git2::Delta::Added {
                    delta
                        .old_file()
                        .path()
                        .and_then(|p| p.to_str())
                        .and_then(|path| {
                            let blob = Self::get_blob(&repository, &left_tree, path).ok()?;
                            Some(Self::blob_to_response(&blob, path))
                        })
                } else {
                    None
                };

                let right = if status != git2::Delta::Deleted {
                    delta
                        .new_file()
                        .path()
                        .and_then(|p| p.to_str())
                        .and_then(|path| {
                            let blob = Self::get_blob(&repository, &right_tree, path).ok()?;
                            Some(Self::blob_to_response(&blob, path))
                        })
                } else {
                    None
                };

                results.push((left, right));
            }

            Ok(results)
        })
        .await?
    }

    async fn get_repo_diff_stats(
        &self,
        owner: &str,
        repo: &str,
        left_ref: &str,
        right_ref: &str,
    ) -> Result<Vec<RepositoryCommitStatResponse>, GitError> {
        let left_ref = left_ref.to_string();
        let right_ref = right_ref.to_string();
        let repository = self.open_repository(owner, repo)?;

        task::spawn_blocking(move || {
            let left_tree = if left_ref == EMPTY_TREE_REF {
                let empty_oid = repository.treebuilder(None)?.write()?;
                repository.find_tree(empty_oid)?
            } else {
                let left_commit = Self::resolve_ref(&repository, &left_ref)?;
                left_commit.tree()?
            };

            let right_commit = Self::resolve_ref(&repository, &right_ref)?;
            let right_tree = right_commit.tree()?;

            let diff = Self::diff_trees(&repository, &left_tree, &right_tree)?;

            let mut results = Vec::new();

            for (idx, delta) in diff.deltas().enumerate() {
                let path = delta
                    .new_file()
                    .path()
                    .or_else(|| delta.old_file().path())
                    .and_then(|p| p.to_str())
                    .unwrap_or("")
                    .to_string();

                let (lines_added, lines_removed) = git2::Patch::from_diff(&diff, idx)
                    .ok()
                    .flatten()
                    .and_then(|p| p.line_stats().ok())
                    .map(|(_, additions, deletions)| (additions as u32, deletions as u32))
                    .unwrap_or((0, 0));

                results.push(RepositoryCommitStatResponse {
                    path,
                    lines_added,
                    lines_removed,
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
