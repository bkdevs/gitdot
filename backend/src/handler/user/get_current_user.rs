use axum::{extract::State, http::StatusCode};

use api::user::UserApiResponse;
use gitdot_core::dto::GetCurrentUserRequest;

use crate::app::{AppError, AppResponse, AppState, AuthenticatedUser};
use crate::dto::UserResponseWrapper;

#[axum::debug_handler]
pub async fn get_current_user(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
) -> Result<AppResponse<UserApiResponse>, AppError> {
    let request = GetCurrentUserRequest::new(auth_user.id);
    state
        .user_service
        .get_current_user(request)
        .await
        .map_err(AppError::from)
        .map(|user| {
            let response: UserResponseWrapper = user.into();
            AppResponse::new(StatusCode::OK, response.0)
        })
}
