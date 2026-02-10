use axum::extract::{Json, State};
use chrono::Utc;
use http::StatusCode;
use uuid::Uuid;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::{CreateRunnerServerRequest, RunnerServerResponse},
};

#[axum::debug_handler]
pub async fn create_runner(
    State(state): State<AppState>,
    Json(request): Json<CreateRunnerServerRequest>,
) -> Result<AppResponse<RunnerServerResponse>, AppError> {
    let response = RunnerServerResponse {
        id: Uuid::new_v4(),
        name: request.name,
        owner_id: request.owner_id,
        owner_type: request.owner_type,
        created_at: Utc::now(),
    };

    Ok(AppResponse::new(StatusCode::CREATED, response))
}
