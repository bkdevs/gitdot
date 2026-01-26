use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use gitdot_core::dto::UpdateCommentRequest;

use crate::app::{AppError, AppResponse, AppState, AuthenticatedUser};
use crate::dto::{CommentServerResponse, UpdateCommentServerRequest};

#[axum::debug_handler]
pub async fn update_comment(
    _auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Path(comment_id): Path<Uuid>,
    Json(request): Json<UpdateCommentServerRequest>,
) -> Result<AppResponse<CommentServerResponse>, AppError> {
    let request = UpdateCommentRequest::new(comment_id, request.body);
    state
        .question_service
        .update_comment(request)
        .await
        .map_err(AppError::from)
        .map(|c| AppResponse::new(StatusCode::OK, c.into()))
}
