use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::model::OrganizationRole;

pub async fn insert_user(pool: &PgPool, id: Uuid, name: &str) {
    sqlx::query("INSERT INTO core.users (id, name) VALUES ($1, $2)")
        .bind(id)
        .bind(name)
        .execute(pool)
        .await
        .unwrap();
}

pub async fn insert_secondary_email(pool: &PgPool, user_id: Uuid, email: &str) {
    sqlx::query(
        "INSERT INTO core.user_emails (user_id, email, is_primary, is_verified)
         VALUES ($1, $2, FALSE, FALSE)",
    )
    .bind(user_id)
    .bind(email)
    .execute(pool)
    .await
    .unwrap();
}

pub async fn insert_org(pool: &PgPool, id: Uuid, name: &str) {
    sqlx::query("INSERT INTO core.organizations (id, name) VALUES ($1, $2)")
        .bind(id)
        .bind(name)
        .execute(pool)
        .await
        .unwrap();
}

pub async fn insert_org_at(pool: &PgPool, id: Uuid, name: &str, created_at: DateTime<Utc>) {
    sqlx::query("INSERT INTO core.organizations (id, name, created_at) VALUES ($1, $2, $3)")
        .bind(id)
        .bind(name)
        .bind(created_at)
        .execute(pool)
        .await
        .unwrap();
}

pub async fn insert_user_repo(
    pool: &PgPool,
    id: Uuid,
    name: &str,
    owner_id: Uuid,
    visibility: &str,
) {
    sqlx::query(
        "INSERT INTO core.repositories (id, name, owner_id, owner_type, visibility)
         VALUES ($1, $2, $3, 'user', $4::core.repository_visibility)",
    )
    .bind(id)
    .bind(name)
    .bind(owner_id)
    .bind(visibility)
    .execute(pool)
    .await
    .unwrap();
}

pub async fn insert_org_repo(pool: &PgPool, id: Uuid, name: &str, org_id: Uuid, visibility: &str) {
    sqlx::query(
        "INSERT INTO core.repositories (id, name, owner_id, owner_type, visibility)
         VALUES ($1, $2, $3, 'organization', $4::core.repository_visibility)",
    )
    .bind(id)
    .bind(name)
    .bind(org_id)
    .bind(visibility)
    .execute(pool)
    .await
    .unwrap();
}

pub async fn insert_commit(
    pool: &PgPool,
    repo_id: Uuid,
    author_id: Uuid,
    sha: &str,
    created_at: DateTime<Utc>,
) {
    sqlx::query(
        "INSERT INTO core.commits (repo_id, author_id, sha, ref_name, message, created_at)
         VALUES ($1, $2, $3, 'refs/heads/main', 'msg', $4)",
    )
    .bind(repo_id)
    .bind(author_id)
    .bind(sha)
    .bind(created_at)
    .execute(pool)
    .await
    .unwrap();
}

pub async fn insert_commit_on(
    pool: &PgPool,
    repo_id: Uuid,
    author_id: Option<Uuid>,
    sha: &str,
    ref_name: &str,
    created_at: DateTime<Utc>,
) {
    sqlx::query(
        "INSERT INTO core.commits (repo_id, author_id, sha, ref_name, message, created_at)
         VALUES ($1, $2, $3, $4, 'msg', $5)",
    )
    .bind(repo_id)
    .bind(author_id)
    .bind(sha)
    .bind(ref_name)
    .bind(created_at)
    .execute(pool)
    .await
    .unwrap();
}

pub async fn insert_star(pool: &PgPool, user_id: Uuid, repo_id: Uuid) {
    sqlx::query("INSERT INTO core.stars (user_id, repository_id) VALUES ($1, $2)")
        .bind(user_id)
        .bind(repo_id)
        .execute(pool)
        .await
        .unwrap();
}

pub async fn insert_star_at(
    pool: &PgPool,
    user_id: Uuid,
    repo_id: Uuid,
    created_at: DateTime<Utc>,
) {
    sqlx::query("INSERT INTO core.stars (user_id, repository_id, created_at) VALUES ($1, $2, $3)")
        .bind(user_id)
        .bind(repo_id)
        .bind(created_at)
        .execute(pool)
        .await
        .unwrap();
}

pub async fn insert_membership_at(
    pool: &PgPool,
    user_id: Uuid,
    org_id: Uuid,
    role: OrganizationRole,
    created_at: DateTime<Utc>,
) {
    sqlx::query(
        "INSERT INTO core.organization_members (user_id, organization_id, role, created_at)
         VALUES ($1, $2, $3, $4)",
    )
    .bind(user_id)
    .bind(org_id)
    .bind(role)
    .bind(created_at)
    .execute(pool)
    .await
    .unwrap();
}

pub async fn insert_filter_at(pool: &PgPool, repo_id: Uuid, name: &str, created_at: DateTime<Utc>) {
    sqlx::query(
        "INSERT INTO core.commit_filters (repository_id, name, created_at) VALUES ($1, $2, $3)",
    )
    .bind(repo_id)
    .bind(name)
    .bind(created_at)
    .execute(pool)
    .await
    .unwrap();
}

pub async fn insert_migration_at(
    pool: &PgPool,
    author_id: Uuid,
    number: i32,
    created_at: DateTime<Utc>,
) -> Uuid {
    sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO migration.migrations
            (number, author_id, origin_service, origin, origin_type, destination, destination_type, created_at)
         VALUES ($1, $2, 'github', 'o', 'user', 'd', 'user', $3)
         RETURNING id",
    )
    .bind(number)
    .bind(author_id)
    .bind(created_at)
    .fetch_one(pool)
    .await
    .unwrap()
}

pub async fn insert_installation_at(
    pool: &PgPool,
    installation_id: i64,
    owner_id: Uuid,
    login: &str,
    created_at: DateTime<Utc>,
) {
    sqlx::query(
        "INSERT INTO migration.github_installations
            (installation_id, owner_id, type, github_login, created_at)
         VALUES ($1, $2, 'user', $3, $4)",
    )
    .bind(installation_id)
    .bind(owner_id)
    .bind(login)
    .bind(created_at)
    .execute(pool)
    .await
    .unwrap();
}
