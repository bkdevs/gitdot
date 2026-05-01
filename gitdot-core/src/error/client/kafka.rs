use thiserror::Error;

#[derive(Debug, Error)]
pub enum KafkaError {
    #[error("Kafka error: {0}")]
    KafkaError(#[from] rdkafka::error::KafkaError),

    #[error("Failed to serialize Kafka payload: {0}")]
    SerializationError(#[from] serde_json::Error),
}
