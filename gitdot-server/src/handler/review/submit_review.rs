use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::submit_review as api;
use gitdot_core::dto::{ReviewingAuthorizationRequest, SubmitComment, SubmitReviewRequest};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn submit_review(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path((owner, repo, number, position)): Path<(String, String, i32, i32)>,
    Json(request): Json<api::SubmitReviewRequest>,
) -> Result<AppResponse<api::SubmitReviewResponse>, AppError> {
    let auth_request = ReviewingAuthorizationRequest::new(auth_user.id, &owner, &repo, number)?;
    state
        .authorization_service
        .verify_authorized_for_reviewing(auth_request)
        .await?;

    let comments = request
        .comments
        .into_iter()
        .map(|c| {
            SubmitComment::new(
                c.body,
                c.parent_id,
                c.file_path,
                c.line_number_start,
                c.line_number_end,
                c.side.as_deref(),
            )
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(AppError::from)?;

    let request = SubmitReviewRequest::new(
        &owner,
        &repo,
        number,
        position,
        auth_user.id,
        &request.action,
        comments,
    )?;

    state
        .review_service
        .submit_review(request)
        .await
        .map_err(AppError::from)
        .map(|response| AppResponse::new(StatusCode::OK, response.into_api()))
}
