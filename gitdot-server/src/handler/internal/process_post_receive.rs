use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};

use gitdot_core::dto::{CreateCommitsRequest, PublishRepoPushRequest};

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
    let publish_request = PublishRepoPushRequest::new(
        &owner,
        &repo,
        request.ref_name.clone(),
        request.old_sha.clone(),
        request.new_sha.clone(),
        request.pusher_id,
    )?;
    let commit_request = CreateCommitsRequest::new(
        &owner,
        &repo,
        request.old_sha,
        request.new_sha,
        request.ref_name,
        None,
        Default::default(),
    )?;

    // execute in the background to avoid blocking push operation
    tokio::spawn(async move {
        if let Err(e) = state
            .webhook_service
            .publish_repo_push(publish_request)
            .await
        {
            tracing::error!("Failed to publish repo push event: {e}");
        }

        if let Err(e) = state.commit_service.create_commits(commit_request).await {
            tracing::error!("Failed to create commits in post-receive: {e}");
        }
    });

    Ok(AppResponse::new(StatusCode::OK, ()))
}
