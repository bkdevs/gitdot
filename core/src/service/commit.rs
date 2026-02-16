use async_trait::async_trait;

use crate::{
    dto::{CommitResponse, CreateCommitRequest},
    error::CommitError,
    repository::{CommitRepository, CommitRepositoryImpl},
};

#[async_trait]
pub trait CommitService: Send + Sync + 'static {
    async fn create_commit(
        &self,
        request: CreateCommitRequest,
    ) -> Result<CommitResponse, CommitError>;
}

#[derive(Debug, Clone)]
pub struct CommitServiceImpl<C>
where
    C: CommitRepository,
{
    commit_repo: C,
}

impl CommitServiceImpl<CommitRepositoryImpl> {
    pub fn new(commit_repo: CommitRepositoryImpl) -> Self {
        Self { commit_repo }
    }
}

#[async_trait]
impl<C> CommitService for CommitServiceImpl<C>
where
    C: CommitRepository,
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
}
