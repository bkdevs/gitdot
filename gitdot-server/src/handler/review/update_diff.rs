use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::update_diff as api;
use gitdot_core::dto::{ReviewAuthorizationRequest, UpdateDiffRequest};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn update_diff(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path((owner, repo, number, position)): Path<(String, String, i32, i32)>,
    Json(request): Json<api::UpdateDiffRequest>,
) -> Result<AppResponse<api::UpdateDiffResponse>, AppError> {
    let auth_request = ReviewAuthorizationRequest::new(auth_user.id, &owner, &repo, number)?;
    state
        .authorization_service
        .verify_authorized_for_review(auth_request)
        .await?;

    let request = UpdateDiffRequest::new(
        &owner,
        &repo,
        number,
        position,
        request.title,
        request.description,
    )?;
    state
        .review_service
        .update_diff(request)
        .await
        .map_err(AppError::from)
        .map(|r| AppResponse::new(StatusCode::OK, r.into_api()))
}
