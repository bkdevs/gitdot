use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::repository::update_repository_settings as api;
use gitdot_core::{
    dto::{RepositoryAuthorizationRequest, RepositoryPermission, UpdateRepositorySettingsRequest},
    model::CommitFilter,
};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn update_repository_settings(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
    Json(body): Json<api::UpdateRepositorySettingsRequest>,
) -> Result<AppResponse<api::UpdateRepositorySettingsResponse>, AppError> {
    let auth_request = RepositoryAuthorizationRequest::new(
        Some(auth_user.id),
        &owner,
        &repo,
        RepositoryPermission::Admin,
    )?;
    state
        .authorization_service
        .verify_authorized_for_repository(auth_request)
        .await?;

    let commit_filters: Option<Vec<CommitFilter>> = body.commit_filters.map(|filters| {
        filters
            .into_iter()
            .map(|f| CommitFilter {
                name: f.name,
                authors: f.authors,
                tags: f.tags,
                included_paths: f.included_paths,
                excluded_paths: f.excluded_paths,
                created_at: f.created_at,
                updated_at: f.updated_at,
            })
            .collect()
    });

    let request = UpdateRepositorySettingsRequest::new(&owner, &repo, commit_filters)?;
    state
        .repo_service
        .update_repository_settings(request)
        .await
        .map_err(AppError::from)
        .map(|resp| AppResponse::new(StatusCode::OK, resp.into_api()))
}
