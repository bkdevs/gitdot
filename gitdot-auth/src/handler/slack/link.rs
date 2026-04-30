use axum::{Json, extract::State};

use gitdot_api::{endpoint::auth::slack::link as api, resource::slack::SlackAccountResource};

use crate::{
    app::{AppError, AppResponse, AppState},
    extract::Principal,
};

pub async fn link_slack_account(
    _principal: Principal,
    State(_state): State<AppState>,
    Json(_body): Json<api::LinkSlackAccountRequest>,
) -> Result<AppResponse<SlackAccountResource>, AppError> {
    todo!()
}
