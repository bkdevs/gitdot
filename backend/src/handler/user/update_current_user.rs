use axum::{Json, extract::State, http::StatusCode};

use gitdot_core::dto::UpdateCurrentUserRequest;

use crate::app::{AppError, AppResponse, AppState, AuthenticatedUser};
use crate::dto::{UpdateCurrentUserServerRequest, UserServerResponse};

#[axum::debug_handler]
pub async fn update_current_user(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Json(request): Json<UpdateCurrentUserServerRequest>,
) -> Result<AppResponse<UserServerResponse>, AppError> {
    let request = UpdateCurrentUserRequest::new(auth_user.id, &request.name)?;
    state
        .user_service
        .update_current_user(request)
        .await
        .map_err(AppError::from)
        .map(|user| AppResponse::new(StatusCode::OK, user.into()))
}
