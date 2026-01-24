use async_trait::async_trait;
use tokio::fs;
use tokio::task;

use crate::dto::{RepositoryFileResponse, RepositoryTreeResponse};
use crate::error::GitError;
use crate::util::consts::{DEFAULT_BRANCH, REPO_SUFFIX};

#[async_trait]
pub trait GitClient: Send + Sync + Clone + 'static {
    async fn repo_exists(&self, owner: &str, repo: &str) -> bool;

    async fn create_repo(&self, owner: &str, repo: &str) -> Result<(), GitError>;

    async fn delete_repo(&self, owner: &str, repo: &str) -> Result<(), GitError>;

    async fn get_repo_tree(
        &self,
        owner: &str,
        repo: &str,
        ref_name: &str,
    ) -> Result<RepositoryTreeResponse, GitError>;

    async fn get_repo_file(
        &self,
        owner: &str,
        repo: &str,
        ref_name: &str,
        path: &str,
    ) -> Result<RepositoryFileResponse, GitError>;

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
}

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

            // Configure the repository for HTTP access
            let mut config = repo.config()?;
            config.set_bool("http.receivepack", true)?;

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

    async fn get_repo_tree(
        &self,
        owner: &str,
        repo: &str,
        ref_name: &str,
    ) -> Result<RepositoryTreeResponse, GitError> {
        todo!();
    }

    async fn get_repo_file(
        &self,
        owner: &str,
        repo: &str,
        ref_name: &str,
        path: &str,
    ) -> Result<RepositoryFileResponse, GitError> {
        todo!();
    }
}
