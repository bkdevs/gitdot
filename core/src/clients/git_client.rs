use async_trait::async_trait;
use tokio::fs;
use tokio::task;

use crate::errors::GitError;

static REPO_SUFFIX: &str = ".git";
static DEFAULT_BRANCH: &str = "main";

#[async_trait]
pub trait GitClient: Send + Sync + Clone + 'static {
    async fn repo_exists(&self, owner: &str, repo_name: &str) -> bool;

    async fn create_repo(&self, owner: &str, repo_name: &str) -> Result<(), GitError>;

    async fn delete_repo(&self, owner: &str, repo_name: &str) -> Result<(), GitError>;

    fn normalize_repo_name(&self, repo_name: &str) -> String {
        format!(
            "{}{}",
            repo_name.strip_suffix(REPO_SUFFIX).unwrap_or(repo_name),
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

    fn get_repo_path(&self, owner: &str, repo_name: &str) -> String {
        let repo_name = self.normalize_repo_name(repo_name);
        format!("{}/{}/{}", self.project_root, owner, repo_name)
    }
}

#[async_trait]
impl GitClient for Git2Client {
    async fn repo_exists(&self, owner: &str, repo_name: &str) -> bool {
        let repo_path = self.get_repo_path(owner, repo_name);
        match fs::metadata(&repo_path).await {
            Ok(metadata) => metadata.is_dir(),
            Err(_) => false,
        }
    }

    async fn create_repo(&self, owner: &str, repo_name: &str) -> Result<(), GitError> {
        let owner_path = self.get_owner_path(owner);
        fs::create_dir_all(&owner_path).await?;

        let repo_path = self.get_repo_path(owner, repo_name);
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

    async fn delete_repo(&self, owner: &str, repo_name: &str) -> Result<(), GitError> {
        let repo_path = self.get_repo_path(owner, repo_name);
        fs::remove_dir_all(&repo_path).await?;
        Ok(())
    }
}
