use async_trait::async_trait;
use sqlx::{Error, PgPool};

use crate::model::User;

#[async_trait]
pub trait UserRepository: Send + Sync + Clone + 'static {
    async fn get(&self, user_name: &str) -> Result<Option<User>, Error>;
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
    async fn get(&self, user_name: &str) -> Result<Option<User>, Error> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, email, name, created_at FROM users WHERE name = $1",
        )
        .bind(user_name)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }
}
