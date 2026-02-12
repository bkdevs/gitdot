use axum::{Json, extract::State, http::StatusCode};

use api::user::UserEndpointResponse;
use api::user::update_current_user::UpdateCurrentUserEndpointRequest;
use gitdot_core::dto::UpdateCurrentUserRequest;

use crate::app::{AppError, AppResponse, AppState, AuthenticatedUser};
use crate::dto::UserResponseWrapper;

#[axum::debug_handler]
pub async fn update_current_user(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Json(request): Json<UpdateCurrentUserEndpointRequest>,
) -> Result<AppResponse<UserEndpointResponse>, AppError> {
    let request = UpdateCurrentUserRequest::new(auth_user.id, &request.name)?;
    state
        .user_service
        .update_current_user(request)
        .await
        .map_err(AppError::from)
        .map(|user| {
            let response: UserResponseWrapper = user.into();
            AppResponse::new(StatusCode::OK, response.0)
        })
}
