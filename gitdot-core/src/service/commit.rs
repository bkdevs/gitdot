use std::collections::HashMap;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    client::{DiffClient, DifftClient, Git2Client, GitClient},
    dto::{
        CommitResponse, CommitsResponse, CreateCommitsRequest, GetCommitRequest, GetCommitsRequest,
    },
    error::CommitError,
    model,
    repository::{
        CommitRepository, CommitRepositoryImpl, RepositoryRepository, RepositoryRepositoryImpl,
        UserRepository, UserRepositoryImpl,
    },
};

#[async_trait]
pub trait CommitService: Send + Sync + 'static {
    async fn get_commit(&self, request: GetCommitRequest) -> Result<CommitResponse, CommitError>;

    async fn get_commits(&self, request: GetCommitsRequest)
    -> Result<CommitsResponse, CommitError>;

    async fn create_commits(
        &self,
        request: CreateCommitsRequest,
    ) -> Result<Vec<CommitResponse>, CommitError>;
}

#[derive(Debug, Clone)]
pub struct CommitServiceImpl<C, R, U, G, D>
where
    C: CommitRepository,
    R: RepositoryRepository,
    U: UserRepository,
    G: GitClient,
    D: DiffClient,
{
    commit_repo: C,
    repo_repo: R,
    user_repo: U,
    git_client: G,
    diff_client: D,
}

impl
    CommitServiceImpl<
        CommitRepositoryImpl,
        RepositoryRepositoryImpl,
        UserRepositoryImpl,
        Git2Client,
        DifftClient,
    >
{
    pub fn new(
        commit_repo: CommitRepositoryImpl,
        repo_repo: RepositoryRepositoryImpl,
        user_repo: UserRepositoryImpl,
        git_client: Git2Client,
        diff_client: DifftClient,
    ) -> Self {
        Self {
            commit_repo,
            repo_repo,
            user_repo,
            git_client,
            diff_client,
        }
    }
}

#[crate::instrument_all]
#[async_trait]
impl<C, R, U, G, D> CommitService for CommitServiceImpl<C, R, U, G, D>
where
    C: CommitRepository,
    R: RepositoryRepository,
    U: UserRepository,
    G: GitClient,
    D: DiffClient,
{
    async fn get_commit(&self, request: GetCommitRequest) -> Result<CommitResponse, CommitError> {
        let owner = request.owner.to_string();
        let repo_name = request.repo.to_string();

        let repository = self
            .repo_repo
            .get(&owner, &repo_name)
            .await?
            .ok_or_else(|| CommitError::RepositoryNotFound(format!("{}/{}", owner, repo_name)))?;

        self.commit_repo
            .get_commit(repository.id, &request.sha)
            .await?
            .map(Into::into)
            .ok_or_else(|| CommitError::NotFound(request.sha))
    }

    async fn get_commits(
        &self,
        request: GetCommitsRequest,
    ) -> Result<CommitsResponse, CommitError> {
        let owner = request.owner.to_string();
        let repo_name = request.repo.to_string();

        let repository = self
            .repo_repo
            .get(&owner, &repo_name)
            .await?
            .ok_or_else(|| CommitError::RepositoryNotFound(format!("{}/{}", owner, repo_name)))?;

        tracing::debug!(
            repo_id = %repository.id,
            ref_name = %request.ref_name,
            page = request.page,
            per_page = request.per_page,
            "get_commits: querying db",
        );

        let fetch_count = request.per_page + 1;
        let mut commits = self
            .commit_repo
            .get_commits(repository.id, request.page, fetch_count)
            .await?;

        tracing::debug!(count = commits.len(), "get_commits: db returned {} rows", commits.len());

        let has_next = commits.len() > request.per_page as usize;
        commits.truncate(request.per_page as usize);

        Ok(CommitsResponse {
            commits: commits.into_iter().map(Into::into).collect(),
            has_next,
        })
    }

    async fn create_commits(
        &self,
        request: CreateCommitsRequest,
    ) -> Result<Vec<CommitResponse>, CommitError> {
        let owner = request.owner.to_string();
        let repo_name = request.repo.to_string();

        let repository = self
            .repo_repo
            .get(&owner, &repo_name)
            .await?
            .ok_or_else(|| CommitError::RepositoryNotFound(format!("{}/{}", owner, repo_name)))?;
        let repo_id = repository.id;

        let git_commits = self
            .git_client
            .rev_list(&owner, &repo_name, &request.old_sha, &request.new_sha)
            .await?;
        if git_commits.is_empty() {
            return Ok(Vec::new());
        }

        let emails: Vec<String> = git_commits
            .iter()
            .map(|c| c.author.email.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        let users = self.user_repo.get_by_emails(&emails).await?;
        let email_to_id: HashMap<String, Uuid> =
            users.into_iter().map(|u| (u.email.clone(), u.id)).collect();

        let mut author_ids: Vec<Option<Uuid>> = Vec::new();
        let mut git_author_names = Vec::new();
        let mut git_author_emails = Vec::new();
        let mut repo_ids = Vec::new();
        let mut ref_names = Vec::new();
        let mut shas = Vec::new();
        let mut messages = Vec::new();
        let mut created_ats: Vec<DateTime<Utc>> = Vec::new();
        let mut diffs_per_commit: Vec<Vec<model::CommitDiff>> = Vec::new();

        // TODO: parallelize this, this is rather slow.
        for commit in &git_commits {
            let diff_files = self
                .git_client
                .get_repo_diff_files(
                    &owner,
                    &repo_name,
                    commit.parent_sha.as_deref(),
                    &commit.sha,
                )
                .await?;
            let mut diffs = Vec::new();
            for (left, right) in diff_files {
                let path = right
                    .as_ref()
                    .or(left.as_ref())
                    .map(|f| f.path.clone())
                    .unwrap_or_default();
                let diff = self
                    .diff_client
                    .diff_files(left.as_ref(), right.as_ref())
                    .await?;
                diffs.push(model::CommitDiff {
                    path,
                    lines_added: diff.lines_added as i32,
                    lines_removed: diff.lines_removed as i32,
                    hunks: diff
                        .hunks
                        .into_iter()
                        .map(|hunk| hunk.into_iter().map(Into::into).collect())
                        .collect(),
                });
            }
            diffs_per_commit.push(diffs);
        }

        for commit in git_commits {
            author_ids.push(email_to_id.get(&commit.author.email).copied());
            git_author_names.push(commit.author.name.clone());
            git_author_emails.push(commit.author.email.clone());
            repo_ids.push(repo_id);
            ref_names.push(request.ref_name.clone());
            shas.push(commit.sha);
            messages.push(commit.message);
            created_ats.push(commit.date);
        }

        let commits = self
            .commit_repo
            .create_bulk(
                &author_ids,
                &git_author_names,
                &git_author_emails,
                &repo_ids,
                &ref_names,
                &shas,
                &messages,
                &created_ats,
                &diffs_per_commit,
            )
            .await?;
        Ok(commits.into_iter().map(|c| c.into()).collect())
    }
}
