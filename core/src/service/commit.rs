use async_trait::async_trait;

use crate::client::{Git2Client, GitClient};
use crate::dto::{CommitResponse, CreateCommitRequest, ProcessPushCommitsRequest};
use crate::error::CommitError;
use crate::repository::{
    CommitRepository, CommitRepositoryImpl, RepositoryRepository, RepositoryRepositoryImpl,
    UserRepository, UserRepositoryImpl,
};

#[async_trait]
pub trait CommitService: Send + Sync + 'static {
    async fn create_commit(
        &self,
        request: CreateCommitRequest,
    ) -> Result<CommitResponse, CommitError>;

    async fn process_push_commits(
        &self,
        request: ProcessPushCommitsRequest,
    ) -> Result<(), CommitError>;
}

#[derive(Debug, Clone)]
pub struct CommitServiceImpl<C, R, U, G>
where
    C: CommitRepository,
    R: RepositoryRepository,
    U: UserRepository,
    G: GitClient,
{
    commit_repo: C,
    repo_repo: R,
    user_repo: U,
    git_client: G,
}

impl
    CommitServiceImpl<
        CommitRepositoryImpl,
        RepositoryRepositoryImpl,
        UserRepositoryImpl,
        Git2Client,
    >
{
    pub fn new(
        commit_repo: CommitRepositoryImpl,
        repo_repo: RepositoryRepositoryImpl,
        user_repo: UserRepositoryImpl,
        git_client: Git2Client,
    ) -> Self {
        Self {
            commit_repo,
            repo_repo,
            user_repo,
            git_client,
        }
    }
}

#[async_trait]
impl<C, R, U, G> CommitService for CommitServiceImpl<C, R, U, G>
where
    C: CommitRepository,
    R: RepositoryRepository,
    U: UserRepository,
    G: GitClient,
{
    async fn create_commit(
        &self,
        request: CreateCommitRequest,
    ) -> Result<CommitResponse, CommitError> {
        let commit = self
            .commit_repo
            .create(
                request.author_id,
                request.repo_id,
                &request.sha,
                &request.message,
            )
            .await?;

        Ok(commit.into())
    }

    async fn process_push_commits(
        &self,
        request: ProcessPushCommitsRequest,
    ) -> Result<(), CommitError> {
        let owner = &request.owner;
        let repo = &request.repo;

        // Get repository from DB
        let repository = self
            .repo_repo
            .get(owner, repo)
            .await?
            .ok_or_else(|| CommitError::RepositoryNotFound(format!("{}/{}", owner, repo)))?;

        // Process each ref update
        for ref_update in &request.ref_updates {
            // Skip deletions
            if ref_update.is_delete() {
                continue;
            }

            // Get commits in range
            let git_commits = self
                .git_client
                .get_commits_in_range(owner, repo, &ref_update.old_sha, &ref_update.new_sha)
                .await?;

            if git_commits.is_empty() {
                continue;
            }

            // Collect unique author emails
            let emails: Vec<String> = git_commits
                .iter()
                .map(|c| c.author.email.clone())
                .collect::<std::collections::HashSet<_>>()
                .into_iter()
                .collect();

            // Look up users by email
            let users = self.user_repo.get_by_emails(&emails).await?;
            let email_to_user: std::collections::HashMap<_, _> =
                users.into_iter().map(|u| (u.email.clone(), u)).collect();

            // Insert commits (skip if already exists or no matching user)
            for git_commit in git_commits {
                let author_id = match email_to_user.get(&git_commit.author.email) {
                    Some(user) => user.id,
                    None => continue, // Skip if no matching user
                };

                // Try to insert, ignore duplicate errors
                let result = self
                    .commit_repo
                    .create(
                        author_id,
                        repository.id,
                        &git_commit.sha,
                        &git_commit.message,
                    )
                    .await;

                // Ignore unique constraint violations (commit already exists)
                if let Err(sqlx::Error::Database(ref e)) = result {
                    if e.is_unique_violation() {
                        continue;
                    }
                }

                result?;
            }
        }

        Ok(())
    }
}
