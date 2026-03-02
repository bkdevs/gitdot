use std::env;

use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use reqwest::Client;
use serde_json::json;

use gitdot_core::{
    dto::{CreateBuildRequest, CreateCommitsRequest},
    error::BuildError,
};

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
    let commit_request = CreateCommitsRequest::new(
        &owner,
        &repo,
        request.old_sha.clone(),
        request.new_sha.clone(),
        request.ref_name.clone(),
    )?;
    let build_request = CreateBuildRequest::new(
        &owner,
        &repo,
        request.ref_name.clone(),
        request.new_sha.clone(),
    )?;

    // execute in the background to avoid blocking push operation
    tokio::spawn(async move {
        if let Err(e) = state.commit_service.create_commits(commit_request).await {
            tracing::error!("Failed to create commits in post-receive: {e}");
        }
    });
    tokio::spawn(async move {
        if let Err(e) = state.build_service.create_build(build_request).await {
            if !matches!(e, BuildError::ConfigNotFound(_)) {
                tracing::error!("Failed to create build in post-receive: {e}");
            }
        }
    });

    // TEMP: Slack debug payload for post-receive.
    let slack_webhook_url = env::var("SLACK_WEBHOOK_URL").ok();
    let text = format!(
        "https://www.gitdot.io/{}/{}/commits/{}",
        owner,
        repo,
        request.new_sha.chars().take(7).collect::<String>()
    );

    tokio::spawn(async move {
        let Some(webhook_url) = slack_webhook_url else {
            return;
        };
        let body = json!({ "text": text }).to_string();

        let client = Client::new();
        if let Err(e) = client
            .post(webhook_url)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
        {
            tracing::error!("Failed to send Slack debug message: {e}");
        }
    });

    Ok(AppResponse::new(StatusCode::OK, ()))
}
