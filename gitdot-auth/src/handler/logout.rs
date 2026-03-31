use axum::extract::State;

use gitdot_core::dto::LogoutRequest;

use crate::{
    app::{AppResponse, AppState, error::AppError},
    extract::RefreshToken,
};

pub async fn logout(
    State(state): State<AppState>,
    RefreshToken(refresh_token): RefreshToken,
) -> Result<AppResponse<()>, AppError> {
    let request = LogoutRequest { refresh_token };
    state
        .authentication_service
        .logout(request)
        .await
        .map_err(AppError::from)
        .map(|_| AppResponse::clear_auth_cookies())
}
