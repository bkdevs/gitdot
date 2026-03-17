use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::list_user_reviews as api;
use gitdot_core::dto::ListUserReviewsRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
};

#[axum::debug_handler]
pub async fn list_user_reviews(
    State(state): State<AppState>,
    Path(user_name): Path<String>,
) -> Result<AppResponse<api::ListUserReviewsResponse>, AppError> {
    let request = ListUserReviewsRequest::new(&user_name)?;
    state
        .user_service
        .list_reviews(request)
        .await
        .map_err(AppError::from)
        .map(|reviews| AppResponse::new(StatusCode::OK, reviews.into_api()))
}
