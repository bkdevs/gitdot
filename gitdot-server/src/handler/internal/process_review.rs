use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};

use gitdot_core::dto::ProcessReviewRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::{ProcessReviewServerRequest, ProcessReviewServerResponse, ReviewAction},
};

#[axum::debug_handler]
pub async fn process_review(
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
    Json(request): Json<ProcessReviewServerRequest>,
) -> Result<AppResponse<ProcessReviewServerResponse>, AppError> {
    let review_request = ProcessReviewRequest::new(
        &owner,
        &repo,
        &request.ref_name,
        request.new_sha,
        request.pusher_id,
    )?;

    let (action, review_number) = if review_request.is_new() {
        let review = state.review_service.create_review(review_request).await?;
        (ReviewAction::Created, review.number)
    } else {
        let review = state.review_service.update_review(review_request).await?;
        (ReviewAction::Updated, review.number)
    };

    Ok(AppResponse::new(
        StatusCode::OK,
        ProcessReviewServerResponse {
            review_number,
            action,
        },
    ))
}
