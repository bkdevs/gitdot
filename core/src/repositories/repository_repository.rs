use async_trait::async_trait;
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::dto::CreateRepositoryRequest;
use crate::models::Repository;

#[async_trait]
pub trait RepositoryRepository: Send + Sync + Clone + 'static {
    async fn create(
        &self,
        owner_id: Uuid,
        request: CreateRepositoryRequest,
    ) -> Result<Repository, Error>;
}

#[derive(Debug, Clone)]
pub struct RepositoryRepositoryImpl {
    pool: PgPool,
}

impl RepositoryRepositoryImpl {
    pub fn new(pool: PgPool) -> RepositoryRepositoryImpl {
        RepositoryRepositoryImpl { pool }
    }
}

#[async_trait]
impl RepositoryRepository for RepositoryRepositoryImpl {
    async fn create(
        &self,
        owner_id: Uuid,
        request: CreateRepositoryRequest,
    ) -> Result<Repository, Error> {
        todo!();
    }
}
