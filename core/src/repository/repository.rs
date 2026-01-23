use async_trait::async_trait;
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::model::{Repository, RepositoryOwnerType, RepositoryVisibility};

#[async_trait]
pub trait RepositoryRepository: Send + Sync + Clone + 'static {
    async fn create(
        &self,
        name: &str,
        owner_id: Uuid,
        owner_name: &str,
        owner_type: &RepositoryOwnerType,
        visibility: &RepositoryVisibility,
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
        name: &str,
        owner_id: Uuid,
        owner_name: &str,
        owner_type: &RepositoryOwnerType,
        visibility: &RepositoryVisibility,
    ) -> Result<Repository, Error> {
        let repository = sqlx::query_as::<_, Repository>(
            r#"
            INSERT INTO repositories (name, owner_id, owner_name, owner_type, visibility)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, name, owner_id, owner_name, owner_type, visibility, created_at
            "#,
        )
        .bind(name)
        .bind(owner_id)
        .bind(owner_name)
        .bind(owner_type)
        .bind(visibility)
        .fetch_one(&self.pool)
        .await?;

        Ok(repository)
    }
}
