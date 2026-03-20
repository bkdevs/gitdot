use std::collections::HashMap;

use axum::{Json, extract::State, http::StatusCode};

use gitdot_api::endpoint::user::update_current_user_settings as api;
use gitdot_core::{dto::UpdateCurrentUserSettingsRequest, model::UserRepoSettings};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::{FromApi, IntoApi},
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn update_current_user_settings(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Json(body): Json<api::UpdateCurrentUserSettingsRequest>,
) -> Result<AppResponse<api::UpdateCurrentUserSettingsResponse>, AppError> {
    let repos = body.repos.map(|r| {
        r.into_iter()
            .map(|(k, v)| (k, UserRepoSettings::from_api(v)))
            .collect::<HashMap<_, _>>()
    });
    let request = UpdateCurrentUserSettingsRequest::new(auth_user.id, repos);
    state
        .user_service
        .update_current_user_settings(request)
        .await
        .map_err(AppError::from)
        .map(|resp| AppResponse::new(StatusCode::OK, resp.into_api()))
}
