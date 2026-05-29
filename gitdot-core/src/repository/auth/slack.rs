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

#[cfg(all(test, feature = "db-tests"))]
mod tests {
    use sqlx::PgPool;
    use uuid::Uuid;

    use super::{SlackRepository, SlackRepositoryImpl};
    use crate::repository::test_common::insert_user;

    #[sqlx::test]
    async fn create_and_get_slack_account(pool: PgPool) {
        let repo = SlackRepositoryImpl::new(pool.clone());
        let user = Uuid::new_v4();
        insert_user(&pool, user, "alice").await;

        let account = repo
            .create_slack_account(user, "U123", "T456")
            .await
            .unwrap();
        assert_eq!(account.gitdot_user_id, user);
        assert_eq!(account.slack_user_id, "U123");
        assert_eq!(account.slack_team_id, "T456");

        let found = repo
            .get_slack_account_by_slack_identity("U123", "T456")
            .await
            .unwrap()
            .expect("found");
        assert_eq!(found.id, account.id);

        // Identity lookup is scoped to the (user, team) pair.
        assert!(
            repo.get_slack_account_by_slack_identity("U123", "OTHER")
                .await
                .unwrap()
                .is_none()
        );
        assert!(
            repo.get_slack_account_by_slack_identity("UNKNOWN", "T456")
                .await
                .unwrap()
                .is_none()
        );
    }
}
