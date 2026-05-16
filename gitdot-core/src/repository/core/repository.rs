use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::{
    dto::UserResponse,
    error::DatabaseError,
    model::{CommitFilter, Repository, RepositoryOwnerType, RepositoryStar, RepositoryVisibility},
};

#[async_trait]
pub trait RepositoryRepository: Send + Sync + Clone + 'static {
    async fn create(
        &self,
        name: &str,
        owner_id: Uuid,
        owner_type: &RepositoryOwnerType,
        visibility: &RepositoryVisibility,
        description: Option<String>,
        readonly: bool,
        created_at: Option<DateTime<Utc>>,
    ) -> Result<Repository, DatabaseError>;

    async fn get(&self, owner: &str, repo: &str) -> Result<Option<Repository>, DatabaseError>;

    async fn get_by_id(&self, id: Uuid) -> Result<Option<Repository>, DatabaseError>;

    async fn list_by_owner(&self, owner_name: &str) -> Result<Vec<Repository>, DatabaseError>;

    async fn delete(&self, id: Uuid) -> Result<(), DatabaseError>;

    async fn star(&self, id: Uuid, user_id: Uuid) -> Result<Option<RepositoryStar>, DatabaseError>;

    async fn unstar(&self, id: Uuid, user_id: Uuid) -> Result<bool, DatabaseError>;

    async fn is_starred(&self, id: Uuid, user_id: Uuid) -> Result<bool, DatabaseError>;

    async fn list_recent_stars(
        &self,
        repository_id: Uuid,
        limit: i64,
    ) -> Result<Vec<(UserResponse, DateTime<Utc>)>, DatabaseError>;

    async fn list_commit_filters(
        &self,
        repository_id: Uuid,
    ) -> Result<Vec<CommitFilter>, DatabaseError>;

    async fn create_commit_filter(
        &self,
        repository_id: Uuid,
        name: &str,
        authors: Option<Vec<String>>,
        tags: Option<Vec<String>>,
        paths: Option<Vec<String>>,
    ) -> Result<CommitFilter, DatabaseError>;

    async fn update_commit_filter(
        &self,
        filter_id: Uuid,
        name: &str,
        authors: Option<Vec<String>>,
        tags: Option<Vec<String>>,
        paths: Option<Vec<String>>,
    ) -> Result<Option<CommitFilter>, DatabaseError>;
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

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl RepositoryRepository for RepositoryRepositoryImpl {
    async fn create(
        &self,
        name: &str,
        owner_id: Uuid,
        owner_type: &RepositoryOwnerType,
        visibility: &RepositoryVisibility,
        description: Option<String>,
        readonly: bool,
        created_at: Option<DateTime<Utc>>,
    ) -> Result<Repository, DatabaseError> {
        let repository = sqlx::query_as::<_, Repository>(
            r#"
            WITH inserted AS (
                INSERT INTO core.repositories (name, owner_id, owner_type, visibility, description, readonly, created_at)
                VALUES ($1, $2, $3, $4, $5, $6, COALESCE($7, NOW()))
                RETURNING id, name, owner_id, owner_type, visibility, description, stars, readonly, created_at
            )
            SELECT i.id, i.name, i.owner_id, COALESCE(u.name, o.name) AS owner_name,
                   i.owner_type, i.visibility, i.description, i.stars, i.readonly, i.created_at
            FROM inserted i
            LEFT JOIN core.users u
              ON i.owner_id = u.id AND i.owner_type = 'user'
            LEFT JOIN core.organizations o
              ON i.owner_id = o.id AND i.owner_type = 'organization'
            "#,
        )
        .bind(name)
        .bind(owner_id)
        .bind(owner_type)
        .bind(visibility)
        .bind(description)
        .bind(readonly)
        .bind(created_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(repository)
    }

    async fn get(&self, owner: &str, repo: &str) -> Result<Option<Repository>, DatabaseError> {
        let repository = sqlx::query_as::<_, Repository>(
            r#"
            SELECT r.id, r.name, r.owner_id, COALESCE(u.name, o.name) AS owner_name,
                   r.owner_type, r.visibility, r.description, r.stars, r.readonly, r.created_at
            FROM core.repositories r
            LEFT JOIN core.users u
              ON r.owner_id = u.id AND r.owner_type = 'user'
            LEFT JOIN core.organizations o
              ON r.owner_id = o.id AND r.owner_type = 'organization'
            WHERE r.name = $2
              AND r.owner_id IN (
                SELECT id FROM core.users         WHERE name = $1
                UNION ALL
                SELECT id FROM core.organizations WHERE name = $1
              )
            "#,
        )
        .bind(owner)
        .bind(repo)
        .fetch_optional(&self.pool)
        .await?;

        Ok(repository)
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<Repository>, DatabaseError> {
        let repository = sqlx::query_as::<_, Repository>(
            r#"
            SELECT r.id, r.name, r.owner_id, COALESCE(u.name, o.name) AS owner_name,
                   r.owner_type, r.visibility, r.description, r.stars, r.readonly, r.created_at
            FROM core.repositories r
            LEFT JOIN core.users u
              ON r.owner_id = u.id AND r.owner_type = 'user'
            LEFT JOIN core.organizations o
              ON r.owner_id = o.id AND r.owner_type = 'organization'
            WHERE r.id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(repository)
    }

    async fn list_by_owner(&self, owner_name: &str) -> Result<Vec<Repository>, DatabaseError> {
        let repositories = sqlx::query_as::<_, Repository>(
            r#"
            SELECT r.id, r.name, r.owner_id, COALESCE(u.name, o.name) AS owner_name,
                   r.owner_type, r.visibility, r.description, r.stars, r.readonly, r.created_at
            FROM core.repositories r
            LEFT JOIN core.users u
              ON r.owner_id = u.id AND r.owner_type = 'user'
            LEFT JOIN core.organizations o
              ON r.owner_id = o.id AND r.owner_type = 'organization'
            WHERE r.owner_id IN (
                SELECT id FROM core.users         WHERE name = $1
                UNION ALL
                SELECT id FROM core.organizations WHERE name = $1
            )
            ORDER BY r.created_at DESC
            "#,
        )
        .bind(owner_name)
        .fetch_all(&self.pool)
        .await?;

        Ok(repositories)
    }

    async fn delete(&self, id: Uuid) -> Result<(), DatabaseError> {
        sqlx::query("DELETE FROM core.repositories WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn star(&self, id: Uuid, user_id: Uuid) -> Result<Option<RepositoryStar>, DatabaseError> {
        let mut tx = self.pool.begin().await?;

        let star = sqlx::query_as::<_, RepositoryStar>(
            r#"
            INSERT INTO core.stars (user_id, repository_id)
            VALUES ($1, $2)
            ON CONFLICT (user_id, repository_id) DO NOTHING
            RETURNING id, user_id, repository_id, created_at
            "#,
        )
        .bind(user_id)
        .bind(id)
        .fetch_optional(&mut *tx)
        .await?;

        if star.is_some() {
            sqlx::query("UPDATE core.repositories SET stars = stars + 1 WHERE id = $1")
                .bind(id)
                .execute(&mut *tx)
                .await?;
        }

        tx.commit().await?;
        Ok(star)
    }

    async fn unstar(&self, id: Uuid, user_id: Uuid) -> Result<bool, DatabaseError> {
        let mut tx = self.pool.begin().await?;

        let deleted =
            sqlx::query("DELETE FROM core.stars WHERE user_id = $1 AND repository_id = $2")
                .bind(user_id)
                .bind(id)
                .execute(&mut *tx)
                .await?
                .rows_affected();

        if deleted > 0 {
            sqlx::query(
                "UPDATE core.repositories SET stars = GREATEST(stars - 1, 0) WHERE id = $1",
            )
            .bind(id)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(deleted > 0)
    }

    async fn is_starred(&self, id: Uuid, user_id: Uuid) -> Result<bool, DatabaseError> {
        let row = sqlx::query(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM core.stars
                WHERE repository_id = $1 AND user_id = $2
            ) AS "starred"
            "#,
        )
        .bind(id)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.try_get("starred")?)
    }

    async fn list_recent_stars(
        &self,
        repository_id: Uuid,
        limit: i64,
    ) -> Result<Vec<(UserResponse, DateTime<Utc>)>, DatabaseError> {
        let rows = sqlx::query(
            r#"
            SELECT u.id, u.name, u.email, u.created_at, u.display_name,
                   s.created_at AS starred_at
            FROM core.stars s
            JOIN core.users u ON s.user_id = u.id
            WHERE s.repository_id = $1
            ORDER BY s.created_at DESC
            LIMIT $2
            "#,
        )
        .bind(repository_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter()
            .map(|row| {
                let user = UserResponse {
                    id: row.try_get("id")?,
                    name: row.try_get("name")?,
                    email: row.try_get("email")?,
                    created_at: row.try_get("created_at")?,
                    display_name: row.try_get("display_name")?,
                    location: None,
                    readme: None,
                    links: vec![],
                };
                let starred_at: DateTime<Utc> = row.try_get("starred_at")?;
                Ok((user, starred_at))
            })
            .collect()
    }

    async fn list_commit_filters(
        &self,
        repository_id: Uuid,
    ) -> Result<Vec<CommitFilter>, DatabaseError> {
        let filters = sqlx::query_as::<_, CommitFilter>(
            r#"
            SELECT id, repository_id, name, authors, tags, paths, created_at, updated_at
            FROM core.commit_filters
            WHERE repository_id = $1
            ORDER BY created_at ASC
            "#,
        )
        .bind(repository_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(filters)
    }

    async fn create_commit_filter(
        &self,
        repository_id: Uuid,
        name: &str,
        authors: Option<Vec<String>>,
        tags: Option<Vec<String>>,
        paths: Option<Vec<String>>,
    ) -> Result<CommitFilter, DatabaseError> {
        let filter = sqlx::query_as::<_, CommitFilter>(
            r#"
            INSERT INTO core.commit_filters (repository_id, name, authors, tags, paths)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, repository_id, name, authors, tags, paths, created_at, updated_at
            "#,
        )
        .bind(repository_id)
        .bind(name)
        .bind(authors)
        .bind(tags)
        .bind(paths)
        .fetch_one(&self.pool)
        .await?;

        Ok(filter)
    }

    async fn update_commit_filter(
        &self,
        filter_id: Uuid,
        name: &str,
        authors: Option<Vec<String>>,
        tags: Option<Vec<String>>,
        paths: Option<Vec<String>>,
    ) -> Result<Option<CommitFilter>, DatabaseError> {
        let filter = sqlx::query_as::<_, CommitFilter>(
            r#"
            UPDATE core.commit_filters
            SET name = $2,
                authors = $3,
                tags = $4,
                paths = $5,
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, repository_id, name, authors, tags, paths, created_at, updated_at
            "#,
        )
        .bind(filter_id)
        .bind(name)
        .bind(authors)
        .bind(tags)
        .bind(paths)
        .fetch_optional(&self.pool)
        .await?;

        Ok(filter)
    }
}
