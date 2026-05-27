use axum::{Json, extract::State, http::StatusCode};

use gitdot_api::{endpoint::auth::refresh_session as api, resource::auth::AuthTokensResource};
use gitdot_axum::extract::{ClientIp, UserAgent};
use gitdot_core::dto::RefreshSessionRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
};

pub async fn refresh_session(
    State(state): State<AppState>,
    UserAgent(user_agent): UserAgent,
    ClientIp(ip_address): ClientIp,
    Json(body): Json<api::RefreshSessionRequest>,
) -> Result<AppResponse<AuthTokensResource>, AppError> {
    let request = RefreshSessionRequest::new(body.refresh_token, user_agent, ip_address.as_deref());
    state
        .session_service
        .refresh_session(request)
        .await
        .map_err(AppError::from)
        .map(|r| AppResponse::new(StatusCode::OK, r.into_api()))
}
