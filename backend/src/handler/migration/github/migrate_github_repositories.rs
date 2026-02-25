use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::migration::github::migrate_github_repositories as api;
use gitdot_core::dto::{
    CreateCommitsRequest, CreateGitHubMigrationRequest, MigrateGitHubRepositoriesRequest,
    MigrationAuthorizationRequest, MigrationResponse,
};

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
    let auth_request = MigrationAuthorizationRequest::new(
        auth_user.id,
        &request.destination,
        &request.destination_type,
    )?;
    state
        .auth_service
        .verify_authorized_for_migration(auth_request)
        .await?;

    let request = CreateGitHubMigrationRequest::new(
        auth_user.id,
        installation_id,
        &request.origin,
        &request.origin_type,
        &request.destination,
        &request.destination_type,
        request.repositories,
    )?;
    let response = state
        .migration_service
        .create_github_migration(request)
        .await?;

    let api_response = MigrationResponse::from_parts(
        response.migration.clone(),
        response.migration_repositories.clone(),
    )
    .into_api();

    let migration_service = state.migration_service.clone();
    let commit_service = state.commit_service.clone();
    tokio::spawn(async move {
        let request = MigrateGitHubRepositoriesRequest {
            migration_id: response.migration.id,
            installation_id,
            owner_id: response.owner_id,
            owner_name: response.owner_name,
            owner_type: response.owner_type,
            migration_repositories: response.migration_repositories,
        };
        let response = migration_service.migrate_github_repositories(request).await;

        // Create commits, starting from the initial commit
        let zero_sha = "0000000000000000000000000000000000000000".to_string();
        for info in response
            .map(|r| r.migrated_repositories)
            .unwrap_or_default()
        {
            if let Some(head_sha) = info.head_sha {
                // Only create commits for HEAD ref for migrated repositories
                if let Ok(req) = CreateCommitsRequest::new(
                    &info.owner_name,
                    &info.repo_name,
                    zero_sha.clone(),
                    head_sha,
                    "refs/heads/main".to_string(),
                ) {
                    let _ = commit_service.create_commits(req).await;
                }
            }
        }
    });

    Ok(AppResponse::new(StatusCode::CREATED, api_response))
}
