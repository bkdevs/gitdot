use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::judge_review_diff as api;
use gitdot_core::dto::{JudgeReviewDiffRequest, ReviewingAuthorizationRequest};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn judge_review_diff(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path((owner, repo, number, position)): Path<(String, String, i32, i32)>,
    Json(request): Json<api::JudgeReviewDiffRequest>,
) -> Result<AppResponse<api::JudgeReviewDiffResponse>, AppError> {
    let auth_request =
        ReviewingAuthorizationRequest::new(auth_user.id, &owner, &repo, number)?;
    state
        .authorization_service
        .verify_authorized_for_reviewing(auth_request)
        .await?;

    let request = JudgeReviewDiffRequest::new(
        &owner,
        &repo,
        number,
        position,
        auth_user.id,
        &request.verdict,
    )?;

    state
        .review_service
        .judge_review_diff(request)
        .await
        .map_err(AppError::from)
        .map(|response| AppResponse::new(StatusCode::OK, response.into_api()))
}
