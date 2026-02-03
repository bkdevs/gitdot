use async_trait::async_trait;
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::model::User;

#[async_trait]
pub trait UserRepository: Send + Sync + Clone + 'static {
    async fn get(&self, user_name: &str) -> Result<Option<User>, Error>;

    async fn get_by_id(&self, id: Uuid) -> Result<Option<User>, Error>;

    async fn get_by_emails(&self, emails: &[String]) -> Result<Vec<User>, Error>;
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

    async fn get_by_id(&self, id: Uuid) -> Result<Option<User>, Error> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, email, name, created_at FROM users WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn get_by_emails(&self, emails: &[String]) -> Result<Vec<User>, Error> {
        if emails.is_empty() {
            return Ok(Vec::new());
        }

        let users = sqlx::query_as::<_, User>(
            "SELECT id, email, name, created_at FROM users WHERE email = ANY($1)",
        )
        .bind(emails)
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }
}
