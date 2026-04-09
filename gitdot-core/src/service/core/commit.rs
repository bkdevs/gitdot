use std::collections::HashMap;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    client::{DiffClient, DifftClient, Git2Client, GitClient},
    dto::{
        CommitDiffResponse, CommitResponse, CommitsResponse, CreateCommitsRequest,
        GetCommitDiffRequest, GetCommitRequest, GetCommitsRequest,
    },
    error::{CommitError, OptionNotFoundExt},
    model,
    repository::{
        CommitRepository, CommitRepositoryImpl, RepositoryRepository, RepositoryRepositoryImpl,
        UserRepository, UserRepositoryImpl,
    },
};

#[async_trait]
pub trait CommitService: Send + Sync + 'static {
    async fn get_commit(&self, request: GetCommitRequest) -> Result<CommitResponse, CommitError>;

    async fn get_commit_diff(
        &self,
        request: GetCommitDiffRequest,
    ) -> Result<CommitDiffResponse, CommitError>;

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
            .or_not_found("repository", format!("{}/{}", owner, repo_name))?;

        let commit = self
            .commit_repo
            .get_commit(repository.id, &request.sha)
            .await?
            .map(Into::into)
            .or_not_found("commit", &request.sha)?;

        Ok(commit)
    }

    async fn get_commit_diff(
        &self,
        request: GetCommitDiffRequest,
    ) -> Result<CommitDiffResponse, CommitError> {
        let owner = request.owner.to_string();
        let repo_name = request.repo.to_string();

        let repository = self
            .repo_repo
            .get(&owner, &repo_name)
            .await?
            .or_not_found("repository", format!("{}/{}", owner, repo_name))?;

        let commit = self
            .commit_repo
            .get_commit(repository.id, &request.sha)
            .await?
            .or_not_found("commit", &request.sha)?;

        let sha = commit.sha.clone();
        let parent_sha = commit.parent_sha.clone();
        let is_initial = parent_sha == "0000000000000000000000000000000000000000";
        let left_ref = if is_initial {
            None
        } else {
            Some(parent_sha.as_str())
        };

        let diff_files = self
            .git_client
            .get_repo_diff_files(&owner, &repo_name, left_ref, &sha)
            .await?;

        let diff_futures: Vec<_> = diff_files
            .iter()
            .map(|(left, right)| self.diff_client.diff_files(left.as_ref(), right.as_ref()))
            .collect();
        let diff_results = futures::future::join_all(diff_futures).await;

        let files = diff_results
            .into_iter()
            .map(|diff_result| diff_result.map_err(CommitError::from))
            .collect::<Result<Vec<_>, CommitError>>()?;

        Ok(CommitDiffResponse {
            sha,
            parent_sha,
            files,
        })
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
            .or_not_found("repository", format!("{}/{}", owner, repo_name))?;

        tracing::debug!(
            repo_id = %repository.id,
            ref_name = %request.ref_name,
            from = %request.from,
            to = %request.to,
            "get_commits: querying db",
        );

        let commits = self
            .commit_repo
            .get_commits(repository.id, request.from, request.to)
            .await?;

        tracing::debug!(
            count = commits.len(),
            "get_commits: db returned {} rows",
            commits.len()
        );

        Ok(CommitsResponse {
            commits: commits.into_iter().map(Into::into).collect(),
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
            .or_not_found("repository", format!("{}/{}", owner, repo_name))?;
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
        let mut owner_names = Vec::new();
        let mut repo_names = Vec::new();
        let mut ref_names = Vec::new();
        let mut shas = Vec::new();
        let mut parent_shas: Vec<String> = Vec::new();
        let mut messages = Vec::new();
        let mut created_ats: Vec<DateTime<Utc>> = Vec::new();
        let mut diffs_per_commit: Vec<Vec<model::CommitDiff>> = Vec::new();

        for commit in &git_commits {
            let stats = self
                .git_client
                .get_repo_diff_stats(
                    &owner,
                    &repo_name,
                    commit.parent_sha.as_deref(),
                    &commit.sha,
                )
                .await?;
            let diffs = stats
                .into_iter()
                .map(|s| model::CommitDiff {
                    path: s.path,
                    lines_added: s.lines_added as i32,
                    lines_removed: s.lines_removed as i32,
                })
                .collect();
            diffs_per_commit.push(diffs);
        }

        let mut review_numbers: Vec<Option<i32>> = Vec::new();
        let mut diff_positions: Vec<Option<i32>> = Vec::new();

        for commit in git_commits {
            author_ids.push(email_to_id.get(&commit.author.email).copied());
            git_author_names.push(commit.author.name.clone());
            git_author_emails.push(commit.author.email.clone());
            repo_ids.push(repo_id);
            owner_names.push(owner.clone());
            repo_names.push(repo_name.clone());
            ref_names.push(request.ref_name.clone());
            review_numbers.push(request.review_number);
            diff_positions.push(request.diff_positions.get(&commit.sha).copied());
            shas.push(commit.sha);
            parent_shas.push(
                commit
                    .parent_sha
                    .unwrap_or_else(|| "0000000000000000000000000000000000000000".to_string()),
            );
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
                &owner_names,
                &repo_names,
                &ref_names,
                &shas,
                &parent_shas,
                &messages,
                &created_ats,
                &diffs_per_commit,
                &review_numbers,
                &diff_positions,
            )
            .await?;
        Ok(commits.into_iter().map(|c| c.into()).collect())
    }
}
