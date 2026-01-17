use async_trait::async_trait;
use sqlx::PgPool;

use crate::dto::FindUserByNameRequest;
use crate::errors::UserError;
use crate::models::User;

#[async_trait]
pub trait UserRepository: Send + Sync + Clone + 'static {
    async fn find_by_name(&self, request: FindUserByNameRequest)
    -> Result<Option<User>, UserError>;
}

#[derive(Debug, Clone)]
pub struct UserRepositoryImpl {
    pool: PgPool,
}

impl UserRepositoryImpl {
    pub fn new(pool: PgPool) -> UserRepositoryImpl {
        UserRepositoryImpl { pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_by_name(
        &self,
        request: FindUserByNameRequest,
    ) -> Result<Option<User>, UserError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, email, name, created_at FROM users WHERE name = $1",
        )
        .bind(request.name.as_ref())
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }
}
