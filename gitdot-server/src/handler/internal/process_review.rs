use axum::{
    extract::{Json, Path},
    http::StatusCode,
};

use crate::{
    app::{AppError, AppResponse},
    dto::{ProcessReviewServerRequest, ProcessReviewServerResponse},
};

#[axum::debug_handler]
pub async fn process_review(
    Path((owner, repo)): Path<(String, String)>,
    Json(request): Json<ProcessReviewServerRequest>,
) -> Result<AppResponse<ProcessReviewServerResponse>, AppError> {
    tracing::info!(?request, %owner, %repo, "processing review");

    Ok(AppResponse::new(
        StatusCode::OK,
        ProcessReviewServerResponse { review_number: 1 },
    ))
}
