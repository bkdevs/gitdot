use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::publish_review as api;
use gitdot_core::dto::{DiffUpdateRequest, PublishReviewRequest, ReviewAuthorizationRequest};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn publish_review(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path((owner, repo, number)): Path<(String, String, i32)>,
    Json(request): Json<api::PublishReviewRequest>,
) -> Result<AppResponse<api::PublishReviewResponse>, AppError> {
    let auth_request = ReviewAuthorizationRequest::new(auth_user.id, &owner, &repo, number)?;
    state
        .authorization_service
        .verify_authorized_for_review(auth_request)
        .await?;

    let diffs = request
        .diffs
        .unwrap_or_default()
        .into_iter()
        .map(|d| DiffUpdateRequest {
            position: d.position,
            title: d.title,
            description: d.description,
        })
        .collect();
    let request = PublishReviewRequest::new(
        &owner,
        &repo,
        number,
        request.title,
        request.description,
        diffs,
    )?;
    state
        .review_service
        .publish_review(request)
        .await
        .map_err(AppError::from)
        .map(|response| AppResponse::new(StatusCode::OK, response.into_api()))
}
