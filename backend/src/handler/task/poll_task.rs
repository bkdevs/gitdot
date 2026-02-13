use axum::{
    extract::{Query, State},
    http::StatusCode,
};
use chrono::Utc;
use uuid::Uuid;

use gitdot_api::endpoint::poll_task as api;
use gitdot_core::{dto::TaskResponse, model::TaskStatus};

use crate::{
    app::{AppResponse, AppState},
    dto::IntoApi,
};

#[axum::debug_handler]
pub async fn poll_task(
    State(_state): State<AppState>,
    Query(_query): Query<api::PollTaskRequest>,
) -> AppResponse<api::PollTaskResponse> {
    let now = Utc::now();
    let response = TaskResponse {
        id: Uuid::new_v4(),
        repo_owner: String::new(),
        repo_name: String::new(),
        script: String::new(),
        status: TaskStatus::Pending,
        created_at: now,
        updated_at: now,
    };

    AppResponse::new(StatusCode::OK, response.into_api())
}
