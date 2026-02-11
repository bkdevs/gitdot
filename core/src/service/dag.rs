use async_trait::async_trait;

use crate::dto::{CreateDagRequest, DagResponse};
use crate::error::DagError;
use crate::repository::{DagRepository, DagRepositoryImpl};

#[async_trait]
pub trait DagService: Send + Sync + 'static {
    async fn create_dag(&self, request: CreateDagRequest) -> Result<DagResponse, DagError>;
}

#[derive(Debug, Clone)]
pub struct DagServiceImpl<R>
where
    R: DagRepository,
{
    dag_repo: R,
}

impl DagServiceImpl<DagRepositoryImpl> {
    pub fn new(dag_repo: DagRepositoryImpl) -> Self {
        Self { dag_repo }
    }
}

#[async_trait]
impl<R> DagService for DagServiceImpl<R>
where
    R: DagRepository,
{
    async fn create_dag(&self, request: CreateDagRequest) -> Result<DagResponse, DagError> {
        let dag = self
            .dag_repo
            .create(
                request.repo_owner.as_ref(),
                request.repo_name.as_ref(),
                &request.task_ids,
            )
            .await?;

        Ok(dag.into())
    }
}
