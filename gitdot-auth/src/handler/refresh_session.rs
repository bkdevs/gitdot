use axum::{Json, extract::State, http::StatusCode};

use gitdot_api::{endpoint::auth::refresh as api, resource::auth::AuthTokensResource};
use gitdot_core::dto::RefreshSessionRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{ClientIp, UserAgent},
};

pub async fn refresh_session(
    State(state): State<AppState>,
    UserAgent(user_agent): UserAgent,
    ClientIp(ip_address): ClientIp,
    Json(body): Json<api::RefreshSessionRequest>,
) -> Result<AppResponse<AuthTokensResource>, AppError> {
    let request = RefreshSessionRequest::new(body.refresh_token, user_agent, ip_address.as_deref());
    state
        .authentication_service
        .refresh_session(request)
        .await
        .map_err(AppError::from)
        .map(|r| AppResponse::new(StatusCode::OK, r.into_api()))
}
