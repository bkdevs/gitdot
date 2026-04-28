use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::create_review_comments as api;
use gitdot_core::dto::{CreateReviewCommentsRequest, ReviewingAuthorizationRequest};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn create_review_comments(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path((owner, repo, number, position)): Path<(String, String, i32, i32)>,
    Json(request): Json<api::CreateReviewCommentsRequest>,
) -> Result<AppResponse<api::CreateReviewCommentsResponse>, AppError> {
    let auth_request = ReviewingAuthorizationRequest::new(auth_user.id, &owner, &repo, number)?;
    state
        .authorization_service
        .verify_authorized_for_reviewing(auth_request)
        .await?;

    let comments = request
        .comments
        .into_iter()
        .map(|c| {
            (
                c.revision_id,
                c.body,
                c.file_path,
                c.line_number_start,
                c.line_number_end,
                c.start_character,
                c.end_character,
                c.side,
            )
        })
        .collect();

    let request = CreateReviewCommentsRequest::new(
        &owner,
        &repo,
        number,
        position,
        auth_user.id,
        comments,
    )?;

    state
        .review_service
        .create_review_comments(request)
        .await
        .map_err(AppError::from)
        .map(|response| AppResponse::new(StatusCode::OK, response.into_api()))
}
