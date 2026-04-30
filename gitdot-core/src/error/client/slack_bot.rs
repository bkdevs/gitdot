use thiserror::Error;

#[derive(Debug, Error)]
pub enum SlackBotError {
    #[error("Slack bot request error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("Slack bot returned non-success status {status}: {body}")]
    NonSuccessStatus { status: u16, body: String },

    #[error("Failed to serialize Slack bot request body: {0}")]
    SerializationError(#[from] serde_json::Error),
}
