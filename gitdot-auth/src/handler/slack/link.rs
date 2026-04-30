use axum::{Json, extract::State, http::StatusCode};

use gitdot_api::{endpoint::auth::slack::link as api, resource::slack::SlackAccountResource};
use gitdot_core::dto::LinkSlackAccountRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::Principal,
};

pub async fn link_slack_account(
    principal: Principal,
    State(state): State<AppState>,
    Json(body): Json<api::LinkSlackAccountRequest>,
) -> Result<AppResponse<SlackAccountResource>, AppError> {
    let request = LinkSlackAccountRequest::new(principal.id, body.state);
    state
        .authentication_service
        .link_slack_account(request)
        .await
        .map_err(AppError::from)
        .map(|r| AppResponse::new(StatusCode::CREATED, r.into_api()))
}
