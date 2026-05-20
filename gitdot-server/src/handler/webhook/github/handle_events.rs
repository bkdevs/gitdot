use axum::extract::State;
use http::StatusCode;
use uuid::Uuid;

use gitdot_core::dto::{
    CreateCommitsRequest, ProcessGithubInstallationRequest, ProcessGithubPushRequest,
};

use crate::{
    app::{AppError, AppResponse, AppState},
    extract::{GithubEvent, GithubSigned},
};

#[axum::debug_handler]
pub async fn handle_events(
    State(state): State<AppState>,
    GithubSigned {
        event,
        delivery,
        body,
    }: GithubSigned,
) -> Result<AppResponse<()>, AppError> {
    match event {
        GithubEvent::Ping => {
            tracing::info!(%delivery, "github webhook ping acknowledged");
        }
        GithubEvent::Push => handle_push(state, delivery, body)?,
        GithubEvent::Installation => handle_installation(state, body).await?,
    }

    Ok(AppResponse::new(StatusCode::OK, ()))
}

fn handle_push(state: AppState, delivery: Uuid, body: Vec<u8>) -> Result<(), AppError> {
    let request = ProcessGithubPushRequest::new(&body)?;

    // run sync in the background so we ack the webhook within github's
    // 10s timeout window even for large pushes
    tokio::spawn(async move {
        let response = match state
            .github_webhook_service
            .process_github_push(request.clone())
            .await
        {
            Ok(r) => r,
            Err(e) => {
                tracing::error!(?e, %delivery, "github push processing failed");
                return;
            }
        };
        // TODO: publish push events
        for info in response.synced_repositories {
            if let Ok(req) = CreateCommitsRequest::new(
                &info.owner_name,
                &info.repo_name,
                request.before.clone(),
                info.head_sha,
                request.ref_name.clone(),
                None,
                Default::default(),
            ) {
                let _ = state.commit_service.create_commits(req).await;
            }
        }
    });
    Ok(())
}

async fn handle_installation(state: AppState, body: Vec<u8>) -> Result<(), AppError> {
    let request = ProcessGithubInstallationRequest::new(&body)?;
    state
        .github_webhook_service
        .process_github_installation(request)
        .await?;
    Ok(())
}
