use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::add_review_reviewer as api;
use gitdot_core::dto::{AddReviewReviewerReqeuest, ReviewAuthorizationRequest};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

use super::ReviewIdParam;

#[axum::debug_handler]
pub async fn add_review_reviewer(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path((owner, repo, id)): Path<(String, String, ReviewIdParam)>,
    Json(request): Json<api::AddReviewReviewerRequest>,
) -> Result<AppResponse<api::AddReviewReviewerResponse>, AppError> {
    let auth_request = ReviewAuthorizationRequest::new(auth_user.id, &owner, &repo, id.0.clone())?;
    state
        .authorization_service
        .verify_authorized_for_review(auth_request)
        .await?;

    let request = AddReviewReviewerReqeuest::new(&owner, &repo, id.0, &request.user_name)?;
    state
        .review_service
        .add_review_reviewer(request)
        .await
        .map_err(AppError::from)
        .map(|response| AppResponse::new(StatusCode::CREATED, response.into_api()))
}
