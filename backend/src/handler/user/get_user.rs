use axum::{extract::State, http::StatusCode};

use api::user::UserEndpointResponse;
use api::user::get_user::GetUserEndpointRequest;
use gitdot_core::dto::GetUserRequest;

use crate::app::{AppError, AppResponse, AppState, Ext};
use crate::dto::GetUserResponse;

#[axum::debug_handler]
pub async fn get_user(
    State(state): State<AppState>,
    Ext(request): Ext<GetUserEndpointRequest>,
) -> Result<AppResponse<UserEndpointResponse>, AppError> {
    let request = GetUserRequest::new(&request.user_name)?;
    let user: GetUserResponse = state
        .user_service
        .get_user(request)
        .await
        .map_err(AppError::from)?
        .into();

    Ok(AppResponse::new(StatusCode::OK, user.0))
}
