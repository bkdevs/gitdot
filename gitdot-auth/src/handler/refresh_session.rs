use axum::extract::State;

use gitdot_api::resource::auth::AuthTokensResource;
use gitdot_core::dto::RefreshSessionRequest;

use crate::{
    app::{AppResponse, AppState, error::AppError},
    extract::{ClientIp, RefreshToken, UserAgent},
};

pub async fn refresh_session(
    State(state): State<AppState>,
    UserAgent(user_agent): UserAgent,
    ClientIp(ip_address): ClientIp,
    RefreshToken(refresh_token): RefreshToken,
) -> Result<AppResponse<AuthTokensResource>, AppError> {
    let request = RefreshSessionRequest::new(refresh_token, user_agent, ip_address.as_deref());
    state
        .authentication_service
        .refresh_session(request)
        .await
        .map_err(AppError::from)
        .map(AppResponse::auth)
}
