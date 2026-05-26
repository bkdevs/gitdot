use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::{
    dto::{Cursor, UserResponse},
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

    async fn get(
        &self,
        owner: &str,
        repo: &str,
        viewer_id: Option<Uuid>,
    ) -> Result<Option<Repository>, DatabaseError>;

    async fn get_by_id(
        &self,
        id: Uuid,
        viewer_id: Option<Uuid>,
    ) -> Result<Option<Repository>, DatabaseError>;

    async fn list_by_owner(
        &self,
        owner_name: &str,
        viewer_id: Option<Uuid>,
        cursor: Option<Cursor>,
        limit: i64,
    ) -> Result<(Vec<Repository>, Option<Cursor>), DatabaseError>;

    async fn list_latest(&self, limit: i64) -> Result<Vec<Repository>, DatabaseError>;

    async fn list_trending(&self, limit: i64) -> Result<Vec<Repository>, DatabaseError>;

    async fn delete(&self, id: Uuid) -> Result<(), DatabaseError>;

    async fn disable_readonly(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Option<Repository>, DatabaseError>;

    async fn star(&self, id: Uuid, user_id: Uuid) -> Result<Option<RepositoryStar>, DatabaseError>;

    async fn unstar(&self, id: Uuid, user_id: Uuid) -> Result<bool, DatabaseError>;

    async fn list_recent_stars(
        &self,
        repository_id: Uuid,
        limit: i64,
    ) -> Result<Vec<(UserResponse, DateTime<Utc>)>, DatabaseError>;

    async fn list_commit_filters(
        &self,
        repository_id: Uuid,
        cursor: Option<Cursor>,
        limit: i64,
    ) -> Result<(Vec<CommitFilter>, Option<Cursor>), DatabaseError>;

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

    async fn delete_commit_filter(&self, filter_id: Uuid) -> Result<bool, DatabaseError>;
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
                   i.owner_type, i.visibility, i.description, i.stars, i.readonly, i.created_at,
                   false AS user_star
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

    async fn get(
        &self,
        owner: &str,
        repo: &str,
        viewer_id: Option<Uuid>,
    ) -> Result<Option<Repository>, DatabaseError> {
        let repository = sqlx::query_as::<_, Repository>(
            r#"
            SELECT r.id, r.name, r.owner_id, COALESCE(u.name, o.name) AS owner_name,
                   r.owner_type, r.visibility, r.description, r.stars, r.readonly, r.created_at,
                   EXISTS(SELECT 1 FROM core.stars s WHERE s.repository_id = r.id AND s.user_id = $3) AS user_star
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
        .bind(viewer_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(repository)
    }

    async fn get_by_id(
        &self,
        id: Uuid,
        viewer_id: Option<Uuid>,
    ) -> Result<Option<Repository>, DatabaseError> {
        let repository = sqlx::query_as::<_, Repository>(
            r#"
            SELECT r.id, r.name, r.owner_id, COALESCE(u.name, o.name) AS owner_name,
                   r.owner_type, r.visibility, r.description, r.stars, r.readonly, r.created_at,
                   EXISTS(SELECT 1 FROM core.stars s WHERE s.repository_id = r.id AND s.user_id = $2) AS user_star
            FROM core.repositories r
            LEFT JOIN core.users u
              ON r.owner_id = u.id AND r.owner_type = 'user'
            LEFT JOIN core.organizations o
              ON r.owner_id = o.id AND r.owner_type = 'organization'
            WHERE r.id = $1
            "#,
        )
        .bind(id)
        .bind(viewer_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(repository)
    }

    async fn list_by_owner(
        &self,
        owner_name: &str,
        viewer_id: Option<Uuid>,
        cursor: Option<Cursor>,
        limit: i64,
    ) -> Result<(Vec<Repository>, Option<Cursor>), DatabaseError> {
        let cursor_created_at = cursor.as_ref().map(|c| c.created_at);
        let cursor_id = cursor.as_ref().map(|c| c.id);

        let mut repositories = sqlx::query_as::<_, Repository>(
            r#"
            SELECT r.id, r.name, r.owner_id, COALESCE(u.name, o.name) AS owner_name,
                   r.owner_type, r.visibility, r.description, r.stars, r.readonly, r.created_at,
                   EXISTS(SELECT 1 FROM core.stars s WHERE s.repository_id = r.id AND s.user_id = $5) AS user_star
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
              AND ($2::timestamptz IS NULL OR (r.created_at, r.id) < ($2, $3))
            ORDER BY r.created_at DESC, r.id DESC
            LIMIT $4
            "#,
        )
        .bind(owner_name)
        .bind(cursor_created_at)
        .bind(cursor_id)
        .bind(limit + 1)
        .bind(viewer_id)
        .fetch_all(&self.pool)
        .await?;

        let next_cursor = if repositories.len() as i64 > limit {
            repositories.pop();
            repositories.last().map(|last| Cursor {
                created_at: last.created_at,
                id: last.id,
            })
        } else {
            None
        };

        Ok((repositories, next_cursor))
    }

    async fn list_latest(&self, limit: i64) -> Result<Vec<Repository>, DatabaseError> {
        let repositories = sqlx::query_as::<_, Repository>(
            r#"
            SELECT r.id, r.name, r.owner_id, COALESCE(u.name, o.name) AS owner_name,
                   r.owner_type, r.visibility, r.description, r.stars, r.readonly, r.created_at,
                   FALSE AS user_star
            FROM core.repositories r
            LEFT JOIN core.users u
              ON r.owner_id = u.id AND r.owner_type = 'user'
            LEFT JOIN core.organizations o
              ON r.owner_id = o.id AND r.owner_type = 'organization'
            WHERE r.visibility = 'public'
            ORDER BY r.created_at DESC, r.id DESC
            LIMIT $1
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(repositories)
    }

    async fn list_trending(&self, limit: i64) -> Result<Vec<Repository>, DatabaseError> {
        let repositories = sqlx::query_as::<_, Repository>(
            r#"
            SELECT r.id, r.name, r.owner_id, COALESCE(u.name, o.name) AS owner_name,
                   r.owner_type, r.visibility, r.description, r.stars, r.readonly, r.created_at,
                   FALSE AS user_star
            FROM core.repositories r
            LEFT JOIN core.users u
              ON r.owner_id = u.id AND r.owner_type = 'user'
            LEFT JOIN core.organizations o
              ON r.owner_id = o.id AND r.owner_type = 'organization'
            WHERE r.visibility = 'public'
            ORDER BY r.stars DESC, r.created_at DESC, r.id DESC
            LIMIT $1
            "#,
        )
        .bind(limit)
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

    async fn disable_readonly(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Option<Repository>, DatabaseError> {
        let repository = sqlx::query_as::<_, Repository>(
            r#"
            WITH updated AS (
                UPDATE core.repositories r
                SET readonly = false
                WHERE r.name = $2
                  AND r.owner_id IN (
                    SELECT id FROM core.users         WHERE name = $1
                    UNION ALL
                    SELECT id FROM core.organizations WHERE name = $1
                  )
                RETURNING r.*
            )
            SELECT r.id, r.name, r.owner_id, COALESCE(u.name, o.name) AS owner_name,
                   r.owner_type, r.visibility, r.description, r.stars, r.readonly, r.created_at,
                   FALSE AS user_star
            FROM updated r
            LEFT JOIN core.users u
              ON r.owner_id = u.id AND r.owner_type = 'user'
            LEFT JOIN core.organizations o
              ON r.owner_id = o.id AND r.owner_type = 'organization'
            "#,
        )
        .bind(owner)
        .bind(repo)
        .fetch_optional(&self.pool)
        .await?;

        Ok(repository)
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

    async fn list_recent_stars(
        &self,
        repository_id: Uuid,
        limit: i64,
    ) -> Result<Vec<(UserResponse, DateTime<Utc>)>, DatabaseError> {
        let rows = sqlx::query(
            r#"
            SELECT u.id, u.name, ue.email, u.created_at, u.display_name,
                   s.created_at AS starred_at
            FROM core.stars s
            JOIN core.users u ON s.user_id = u.id
            JOIN core.user_emails ue ON ue.user_id = u.id AND ue.is_primary
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
        cursor: Option<Cursor>,
        limit: i64,
    ) -> Result<(Vec<CommitFilter>, Option<Cursor>), DatabaseError> {
        let cursor_created_at = cursor.as_ref().map(|c| c.created_at);
        let cursor_id = cursor.as_ref().map(|c| c.id);

        let mut filters = sqlx::query_as::<_, CommitFilter>(
            r#"
            SELECT id, repository_id, name, authors, tags, paths, created_at, updated_at
            FROM core.commit_filters
            WHERE repository_id = $1
              AND ($2::timestamptz IS NULL OR (created_at, id) < ($2, $3))
            ORDER BY created_at DESC, id DESC
            LIMIT $4
            "#,
        )
        .bind(repository_id)
        .bind(cursor_created_at)
        .bind(cursor_id)
        .bind(limit + 1)
        .fetch_all(&self.pool)
        .await?;

        let next_cursor = if filters.len() as i64 > limit {
            filters.pop();
            filters.last().map(|last| Cursor {
                created_at: last.created_at,
                id: last.id,
            })
        } else {
            None
        };

        Ok((filters, next_cursor))
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

    async fn delete_commit_filter(&self, filter_id: Uuid) -> Result<bool, DatabaseError> {
        let result = sqlx::query("DELETE FROM core.commit_filters WHERE id = $1")
            .bind(filter_id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}
