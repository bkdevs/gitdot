use anyhow::Context;
use futures::StreamExt;
use rdkafka::{
    Message,
    consumer::{CommitMode, Consumer, ConsumerContext, StreamConsumer},
    message::BorrowedMessage,
};

use gitdot_core::{
    dto::{ListSlackWebhooksRequest, NotifyRepoPushRequest, RepoPushEvent},
    model::WebhookEventType,
};

use super::ConsumerState;

pub async fn run<C>(state: ConsumerState, kafka: StreamConsumer<C>) -> anyhow::Result<()>
where
    C: ConsumerContext + 'static,
{
    let mut stream = kafka.stream();
    let shutdown = shutdown_signal();
    tokio::pin!(shutdown);

    loop {
        tokio::select! {
            _ = &mut shutdown => {
                tracing::info!("shutdown signal received; draining offsets");
                break;
            }
            msg = stream.next() => {
                let Some(msg) = msg else {
                    tracing::warn!("kafka stream ended unexpectedly");
                    break;
                };
                match msg {
                    Ok(msg) => {
                        if let Err(e) = handle_message(&state, &msg).await {
                            tracing::error!(?e, "handler failed; not committing offset");
                            continue;
                        }
                        if let Err(e) = kafka.commit_message(&msg, CommitMode::Async) {
                            tracing::error!(?e, "failed to commit kafka offset");
                        }
                    }
                    Err(e) => tracing::error!(?e, "kafka receive error"),
                }
            }
        }
    }

    Ok(())
}

async fn handle_message(state: &ConsumerState, msg: &BorrowedMessage<'_>) -> anyhow::Result<()> {
    let payload = msg
        .payload()
        .ok_or_else(|| anyhow::anyhow!("message has no payload"))?;

    let event: RepoPushEvent =
        serde_json::from_slice(payload).context("deserialize RepoPushEvent")?;

    let list_request =
        ListSlackWebhooksRequest::new(&event.owner, &event.repo, WebhookEventType::Push)?;
    let subscriptions = state
        .slack_webhook_service
        .list_slack_webhooks(list_request)
        .await?;

    if subscriptions.is_empty() {
        tracing::debug!(
            owner = %event.owner,
            repo = %event.repo,
            "no slack subscriptions for repo; skipping",
        );
        return Ok(());
    }

    tracing::info!(
        owner = %event.owner,
        repo = %event.repo,
        subscribers = subscriptions.len(),
        "fanning out repo push notification",
    );

    // TODO: parallelize fan-out and add per-channel retry.
    for sub in subscriptions {
        let channel_id = sub.slack_channel_id.clone();
        let body = NotifyRepoPushRequest {
            channel_id: sub.slack_channel_id,
            owner: event.owner.clone(),
            repo: event.repo.clone(),
            ref_name: event.ref_name.clone(),
            pusher_name: event.pusher_name.clone(),
            commits: event.commits.clone(),
        };
        if let Err(e) = state
            .slack_webhook_service
            .notify_slack_of_repo_push(body)
            .await
        {
            tracing::error!(
                ?e,
                channel_id = %channel_id,
                "failed to notify slack channel; aborting batch",
            );
            return Err(e.into());
        }
    }

    Ok(())
}

async fn shutdown_signal() {
    use tokio::signal::unix::{SignalKind, signal};

    let mut term = match signal(SignalKind::terminate()) {
        Ok(s) => s,
        Err(e) => {
            tracing::warn!(?e, "failed to install SIGTERM handler");
            return;
        }
    };
    let mut int = match signal(SignalKind::interrupt()) {
        Ok(s) => s,
        Err(e) => {
            tracing::warn!(?e, "failed to install SIGINT handler");
            return;
        }
    };
    tokio::select! {
        _ = term.recv() => tracing::info!("SIGTERM"),
        _ = int.recv() => tracing::info!("SIGINT"),
    }
}
