use axum::{extract::State, http::StatusCode};

use gitdot_api::endpoint::user::get_current_user_settings as api;
use gitdot_core::dto::GetCurrentUserSettingsRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn get_current_user_settings(
    auth_user: Principal<User>,
    State(state): State<AppState>,
) -> Result<AppResponse<api::GetCurrentUserSettingsResponse>, AppError> {
    let request = GetCurrentUserSettingsRequest::new(auth_user.id);
    state
        .user_service
        .get_current_user_settings(request)
        .await
        .map_err(AppError::from)
        .map(|resp| AppResponse::new(StatusCode::OK, resp.into_api()))
}
