use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{error::DatabaseError, model::SlackAccount};

#[async_trait]
pub trait SlackRepository: Send + Sync + Clone + 'static {
    async fn create_slack_account(
        &self,
        gitdot_user_id: Uuid,
        slack_user_id: &str,
        slack_team_id: &str,
    ) -> Result<SlackAccount, DatabaseError>;

    async fn get_slack_account_by_slack_identity(
        &self,
        slack_user_id: &str,
        slack_team_id: &str,
    ) -> Result<Option<SlackAccount>, DatabaseError>;
}

#[derive(Debug, Clone)]
pub struct SlackRepositoryImpl {
    pool: PgPool,
}

impl SlackRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl SlackRepository for SlackRepositoryImpl {
    async fn create_slack_account(
        &self,
        gitdot_user_id: Uuid,
        slack_user_id: &str,
        slack_team_id: &str,
    ) -> Result<SlackAccount, DatabaseError> {
        let slack_account = sqlx::query_as::<_, SlackAccount>(
            r#"
            INSERT INTO auth.slack_accounts (gitdot_user_id, slack_user_id, slack_team_id)
            VALUES ($1, $2, $3)
            RETURNING id, gitdot_user_id, slack_user_id, slack_team_id, created_at
            "#,
        )
        .bind(gitdot_user_id)
        .bind(slack_user_id)
        .bind(slack_team_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(slack_account)
    }

    async fn get_slack_account_by_slack_identity(
        &self,
        slack_user_id: &str,
        slack_team_id: &str,
    ) -> Result<Option<SlackAccount>, DatabaseError> {
        let slack_account = sqlx::query_as::<_, SlackAccount>(
            r#"
            SELECT id, gitdot_user_id, slack_user_id, slack_team_id, created_at
            FROM auth.slack_accounts
            WHERE slack_user_id = $1 AND slack_team_id = $2
            "#,
        )
        .bind(slack_user_id)
        .bind(slack_team_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(slack_account)
    }
}
