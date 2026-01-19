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
        let repository = sqlx::query_as::<_, Repository>(
            r#"
            INSERT INTO repositories (name, owner_id, owner_name, owner_type, visibility)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, name, owner_id, owner_name, owner_type, visibility, created_at
            "#,
        )
        .bind(request.name.as_ref())
        .bind(owner_id)
        .bind(&request.owner_name)
        .bind(&request.owner_type)
        .bind(&request.visibility)
        .fetch_one(&self.pool)
        .await?;

        Ok(repository)
    }
}
