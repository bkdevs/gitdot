use std::collections::HashMap;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    client::{Git2Client, GitClient},
    dto::{CommitResponse, CreateCommitsRequest},
    error::CommitError,
    model,
    repository::{
        CommitRepository, CommitRepositoryImpl, RepositoryRepository, RepositoryRepositoryImpl,
        UserRepository, UserRepositoryImpl,
    },
    util::git::EMPTY_TREE_REF,
};

#[async_trait]
pub trait CommitService: Send + Sync + 'static {
    async fn create_commits(
        &self,
        request: CreateCommitsRequest,
    ) -> Result<Vec<CommitResponse>, CommitError>;
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

#[crate::instrument_all]
#[async_trait]
impl<C, R, U, G> CommitService for CommitServiceImpl<C, R, U, G>
where
    C: CommitRepository,
    R: RepositoryRepository,
    U: UserRepository,
    G: GitClient,
{
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

        for commit in &git_commits {
            let left_ref = commit.parent_sha.as_deref().unwrap_or(EMPTY_TREE_REF);
            let stats = self
                .git_client
                .get_repo_diff_stats(&owner, &repo_name, left_ref, &commit.sha)
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
