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
    dto::{FromApi, IntoApi},
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

    let commit_filters = <Option<Vec<CommitFilter>>>::from_api(body.commit_filters);
    let request = UpdateRepositorySettingsRequest::new(&owner, &repo, commit_filters)?;
    state
        .repo_service
        .update_repository_settings(request)
        .await
        .map_err(AppError::from)
        .map(|resp| AppResponse::new(StatusCode::OK, resp.into_api()))
}
