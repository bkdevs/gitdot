use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::migration::migrate_github_repositories as api;
use gitdot_core::dto::{MigrateGitHubRepositoriesRequest, MigrationAuthorizationRequest};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn migrate_github_repositories(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path(installation_id): Path<i64>,
    Json(request): Json<api::MigrateGitHubRepositoriesRequest>,
) -> Result<AppResponse<api::MigrateGitHubRepositoriesResponse>, AppError> {
    let auth_request =
        MigrationAuthorizationRequest::new(auth_user.id, &request.owner, &request.owner_type)?;
    state
        .auth_service
        .verify_authorized_for_migration(auth_request)
        .await?;

    let request = MigrateGitHubRepositoriesRequest::new(
        installation_id,
        &request.owner,
        &request.owner_type,
        request.repositories,
        auth_user.id,
    )?;
    state
        .migration_service
        .migrate_github_repositories(request)
        .await
        .map_err(AppError::from)
        .map(|migration| AppResponse::new(StatusCode::CREATED, migration.into_api()))
}
