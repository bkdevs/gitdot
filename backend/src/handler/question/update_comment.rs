use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use gitdot_core::dto::{CommentAuthorizationRequest, UpdateCommentRequest};

use crate::app::{AppError, AppResponse, AppState, AuthenticatedUser};
use crate::dto::{CommentServerResponse, UpdateCommentServerRequest};

#[axum::debug_handler]
pub async fn update_comment(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Path((_owner, _repo, _number, comment_id)): Path<(String, String, i32, Uuid)>,
    Json(request): Json<UpdateCommentServerRequest>,
) -> Result<AppResponse<CommentServerResponse>, AppError> {
    let auth_request = CommentAuthorizationRequest::new(auth_user.id, comment_id);
    state
        .auth_service
        .verify_authorized_for_comment(auth_request)
        .await?;

    let request = UpdateCommentRequest::new(comment_id, request.body);
    state
        .question_service
        .update_comment(request)
        .await
        .map_err(AppError::from)
        .map(|c| AppResponse::new(StatusCode::OK, c.into()))
}
