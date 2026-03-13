use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::add_reviewer as api;
use gitdot_core::dto::{AddReviewerRequest, ReviewAuthorizationRequest};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn add_reviewer(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path((owner, repo, number)): Path<(String, String, i32)>,
    Json(request): Json<api::AddReviewerRequest>,
) -> Result<AppResponse<api::AddReviewerResponse>, AppError> {
    let auth_request = ReviewAuthorizationRequest::new(auth_user.id, &owner, &repo, number)?;
    state
        .authorization_service
        .verify_authorized_for_review(auth_request)
        .await?;

    let request = AddReviewerRequest::new(&owner, &repo, number, &request.user_name)?;
    state
        .review_service
        .add_reviewer(request)
        .await
        .map_err(AppError::from)
        .map(|response| AppResponse::new(StatusCode::CREATED, response.into_api()))
}
