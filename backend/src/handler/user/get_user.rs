use axum::extract::Path;
use axum::{extract::State, http::StatusCode};

use api::user::UserEndpointResponse;
use gitdot_core::dto::GetUserRequest;

use crate::app::{AppError, AppResponse, AppState};
use crate::dto::UserResponseWrapper;

#[axum::debug_handler]
pub async fn get_user(
    State(state): State<AppState>,
    Path(user_name): Path<String>,
) -> Result<AppResponse<UserEndpointResponse>, AppError> {
    let request = GetUserRequest::new(&user_name)?;
    let user: UserResponseWrapper = state
        .user_service
        .get_user(request)
        .await
        .map_err(AppError::from)?
        .into();

    Ok(AppResponse::new(StatusCode::OK, user.0))
}
