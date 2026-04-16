use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::update_review as api;
use gitdot_core::dto::{ReviewAuthorizationRequest, UpdateReviewRequest};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

use super::ReviewIdParam;

#[axum::debug_handler]
pub async fn update_review(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path((owner, repo, id)): Path<(String, String, ReviewIdParam)>,
    Json(request): Json<api::UpdateReviewRequest>,
) -> Result<AppResponse<api::UpdateReviewResponse>, AppError> {
    let auth_request = ReviewAuthorizationRequest::new(auth_user.id, &owner, &repo, id.0.clone())?;
    state
        .authorization_service
        .verify_authorized_for_review(auth_request)
        .await?;

    let request =
        UpdateReviewRequest::new(&owner, &repo, id.0, request.title, request.description)?;
    state
        .review_service
        .update_review(request)
        .await
        .map_err(AppError::from)
        .map(|r| AppResponse::new(StatusCode::OK, r.into_api()))
}
