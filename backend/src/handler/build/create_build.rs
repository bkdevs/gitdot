use axum::extract::{Json, State};
use http::StatusCode;

use gitdot_api::endpoint::create_build as api;
use gitdot_core::dto::{BuildTrigger, CreateBuildRequest};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
};

#[axum::debug_handler]
pub async fn create_build(
    State(state): State<AppState>,
    Json(request): Json<api::CreateBuildRequest>,
) -> Result<AppResponse<api::CreateBuildResponse>, AppError> {
    let trigger = match request.trigger.as_str() {
        "pull_request" => BuildTrigger::PullRequest,
        "push_to_main" => BuildTrigger::PushToMain,
        other => {
            return Err(AppError::Build(
                gitdot_core::error::BuildError::InvalidTrigger(other.to_string()),
            ));
        }
    };

    let request = CreateBuildRequest::new(
        &request.repo_owner,
        &request.repo_name,
        trigger,
        request.commit_sha,
    )?;

    state
        .build_service
        .create_build(request)
        .await
        .map_err(AppError::from)
        .map(|d| AppResponse::new(StatusCode::CREATED, d.into_api()))
}
