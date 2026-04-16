use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};

use gitdot_core::dto::{ProcessReviewRequest, ReviewAuthorizationRequest, ReviewId};

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

    let (action, review_number, review_id) = if review_request.is_new() {
        let review = state.review_service.create_review(review_request).await?;
        let id = review.id.simple().to_string()[..8].to_string();
        (ReviewAction::Created, review.number, id)
    } else {
        let review_number = review_request.review_number.unwrap() as i32;
        let auth_request = ReviewAuthorizationRequest::new(
            request.pusher_id,
            &owner,
            &repo,
            ReviewId::Number(review_number),
        )?;
        state
            .authorization_service
            .verify_authorized_for_review(auth_request)
            .await?;

        let review = state
            .review_service
            .process_review_update(review_request)
            .await?;
        let id = review.id.simple().to_string()[..8].to_string();
        (ReviewAction::Updated, review.number, id)
    };

    Ok(AppResponse::new(
        StatusCode::OK,
        ProcessReviewServerResponse {
            review_number,
            review_id,
            action,
        },
    ))
}
