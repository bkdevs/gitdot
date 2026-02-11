use axum::extract::{Query, State};
use chrono::Utc;
use gitdot_core::dto::TaskResponse;
use gitdot_core::model::TaskStatus;
use http::StatusCode;
use uuid::Uuid;

use crate::{
    app::{AppResponse, AppState},
    dto::{PollTaskQuery, TaskServerResponse},
};

#[axum::debug_handler]
pub async fn poll_task(
    State(_state): State<AppState>,
    Query(_query): Query<PollTaskQuery>,
) -> AppResponse<TaskServerResponse> {
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

    AppResponse::new(StatusCode::OK, response.into())
}
