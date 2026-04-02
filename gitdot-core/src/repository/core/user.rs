use async_trait::async_trait;
use rand::RngExt as _;
use sqlx::{Error, PgPool, Row as _};
use uuid::Uuid;

use crate::model::{AuthProvider, User, UserSettings};

#[async_trait]
pub trait UserRepository: Send + Sync + Clone + 'static {
    async fn create(
        &self,
        email: &str,
        is_email_verified: bool,
        provider: AuthProvider,
    ) -> Result<User, Error>;

    async fn get(&self, user_name: &str) -> Result<Option<User>, Error>;

    async fn update(&self, id: Uuid, name: &str) -> Result<User, Error>;

    async fn get_by_id(&self, id: Uuid) -> Result<Option<User>, Error>;

    async fn get_by_email(&self, email: &str) -> Result<Option<User>, Error>;

    async fn get_by_emails(&self, emails: &[String]) -> Result<Vec<User>, Error>;

    async fn get_settings(&self, id: Uuid) -> Result<Option<UserSettings>, Error>;

    async fn update_settings(
        &self,
        id: Uuid,
        settings: UserSettings,
    ) -> Result<Option<UserSettings>, Error>;

    async fn verify_email(&self, id: Uuid) -> Result<(), Error>;

    async fn get_org_memberships(&self, user_id: Uuid) -> Result<Vec<(String, String)>, Error>;

    async fn is_name_taken(&self, name: &str) -> Result<bool, Error>;

    async fn is_email_taken(&self, email: &str) -> Result<bool, Error>;
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

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create(
        &self,
        email: &str,
        is_email_verified: bool,
        provider: AuthProvider,
    ) -> Result<User, Error> {
        let suffix: String = {
            let mut rng = rand::rng();
            let bytes: [u8; 4] = rng.random();
            hex::encode(bytes)
        };
        let name = format!("user_{suffix}");
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO core.users (email, name, is_email_verified, provider)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
        )
        .bind(email)
        .bind(name)
        .bind(is_email_verified)
        .bind(provider)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    async fn get(&self, user_name: &str) -> Result<Option<User>, Error> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT id, email, name, is_email_verified, provider, created_at, settings
            FROM core.users
            WHERE name = $1
            "#,
        )
        .bind(user_name)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn update(&self, id: Uuid, name: &str) -> Result<User, Error> {
        let user = sqlx::query_as::<_, User>(
            r#"
            UPDATE core.users SET name = $1
            WHERE id = $2
            RETURNING id, email, name, is_email_verified, provider, created_at, settings
            "#,
        )
        .bind(name)
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<User>, Error> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT id, email, name, is_email_verified, provider, created_at, settings
            FROM core.users
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn get_by_email(&self, email: &str) -> Result<Option<User>, Error> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT id, email, name, is_email_verified, provider, created_at, settings
            FROM core.users
            WHERE email = $1
            "#,
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn get_by_emails(&self, emails: &[String]) -> Result<Vec<User>, Error> {
        if emails.is_empty() {
            return Ok(Vec::new());
        }

        let users = sqlx::query_as::<_, User>(
            r#"
            SELECT id, email, name, is_email_verified, provider, created_at, settings
            FROM core.users
            WHERE email = ANY($1)
            "#,
        )
        .bind(emails)
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }

    async fn get_settings(&self, id: Uuid) -> Result<Option<UserSettings>, Error> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT id, email, name, is_email_verified, provider, created_at, settings
            FROM core.users
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user.and_then(|u| u.settings))
    }

    async fn update_settings(
        &self,
        id: Uuid,
        settings: UserSettings,
    ) -> Result<Option<UserSettings>, Error> {
        let settings = serde_json::to_value(&settings).unwrap();
        let row = sqlx::query(
            r#"
            UPDATE core.users SET settings = COALESCE(settings, '{}'::jsonb) || $2::jsonb
            WHERE id = $1
            RETURNING settings
            "#,
        )
        .bind(id)
        .bind(settings)
        .fetch_optional(&self.pool)
        .await?;

        let Some(row) = row else { return Ok(None) };
        let json: Option<serde_json::Value> = row.try_get("settings")?;
        Ok(Some(
            json.and_then(|v| serde_json::from_value(v).ok())
                .unwrap_or_default(),
        ))
    }

    async fn verify_email(&self, id: Uuid) -> Result<(), Error> {
        sqlx::query(
            r#"
            UPDATE core.users SET is_email_verified = true WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_org_memberships(&self, user_id: Uuid) -> Result<Vec<(String, String)>, Error> {
        let rows = sqlx::query_as::<_, (String, String)>(
            r#"
            SELECT o.name, om.role::text
            FROM core.organization_members om
            JOIN core.organizations o ON o.id = om.organization_id
            WHERE om.user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    async fn is_name_taken(&self, name: &str) -> Result<bool, Error> {
        let exists = sqlx::query_scalar::<_, bool>(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM core.users WHERE name = $1
                UNION
                SELECT 1 FROM core.organizations WHERE name = $1
            )
            "#,
        )
        .bind(name)
        .fetch_one(&self.pool)
        .await?;

        Ok(exists)
    }

    async fn is_email_taken(&self, email: &str) -> Result<bool, Error> {
        let exists = sqlx::query_scalar::<_, bool>(
            r#"
            SELECT EXISTS(SELECT 1 FROM core.users WHERE email = $1)
            "#,
        )
        .bind(email)
        .fetch_one(&self.pool)
        .await?;

        Ok(exists)
    }
}
