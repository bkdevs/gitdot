use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};

use gitdot_core::dto::CreateCommitsRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::ProcessPostReceiveServerRequest,
};

#[axum::debug_handler]
pub async fn process_post_receive(
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
    Json(request): Json<ProcessPostReceiveServerRequest>,
) -> Result<AppResponse<()>, AppError> {
    let request = CreateCommitsRequest::new(
        &owner,
        &repo,
        request.old_sha,
        request.new_sha,
        request.ref_name,
    )?;

    // execute in the background to avoid blocking push operation
    tokio::spawn(async move {
        if let Err(e) = state.commit_service.create_commits(request).await {
            tracing::error!("Failed to process post-receive hook: {e}");
        }
    });

    Ok(AppResponse::new(StatusCode::OK, ()))
}
