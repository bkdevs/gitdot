use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use gitdot_api::endpoint::review::resolve_review_comment as api;
use gitdot_core::dto::ResolveReviewCommentRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn resolve_review_comment(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path((owner, repo, number, comment_id)): Path<(String, String, i32, Uuid)>,
    Json(request): Json<api::ResolveReviewCommentRequest>,
) -> Result<AppResponse<api::ResolveReviewCommentResponse>, AppError> {
    let request = ResolveReviewCommentRequest::new(
        &owner,
        &repo,
        number,
        comment_id,
        auth_user.id,
        request.resolved,
    )?;

    state
        .review_service
        .resolve_review_comment(request)
        .await
        .map_err(AppError::from)
        .map(|response| AppResponse::new(StatusCode::OK, response.into_api()))
}
