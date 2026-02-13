use axum::{extract::State, http::StatusCode};

use crate::{
    app::{AppError, AppResponse, AppState, AuthenticatedUser},
    dto::IntoApi,
};
use gitdot_core::dto::GetCurrentUserRequest;

#[axum::debug_handler]
pub async fn get_current_user(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
) -> Result<AppResponse<api::GetCurrentUserResponse>, AppError> {
    let request = GetCurrentUserRequest::new(auth_user.id);
    state
        .user_service
        .get_current_user(request)
        .await
        .map_err(AppError::from)
        .map(|user| AppResponse::new(StatusCode::OK, user.into_api()))
}
