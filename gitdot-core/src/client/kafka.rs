use std::time::Duration;

use async_trait::async_trait;
use rdkafka::{
    ClientConfig,
    producer::{FutureProducer, FutureRecord, Producer},
    util::Timeout,
};

use crate::{dto::RepoPushEvent, error::KafkaError};

const REPO_PUSHED_TOPIC: &str = "gitdot.repo.pushed";

const SEND_TIMEOUT: Duration = Duration::from_secs(5);

#[async_trait]
pub trait KafkaClient: Send + Sync + Clone + 'static {
    async fn publish_repo_push(&self, event: RepoPushEvent) -> Result<(), KafkaError>;
}

#[derive(Clone)]
pub struct KafkaClientImpl {
    producer: FutureProducer,
}

impl KafkaClientImpl {
    pub fn new(bootstrap_servers: &str) -> Result<Self, KafkaError> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", bootstrap_servers)
            .set("message.timeout.ms", "5000")
            .set("acks", "all")
            .set("enable.idempotence", "true")
            .create()?;

        Ok(Self { producer })
    }
}

impl std::fmt::Debug for KafkaClientImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KafkaClientImpl").finish_non_exhaustive()
    }
}

impl Drop for KafkaClientImpl {
    fn drop(&mut self) {
        // Block briefly so any in-flight messages get delivered before shutdown.
        self.producer.flush(Timeout::After(SEND_TIMEOUT)).ok();
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl KafkaClient for KafkaClientImpl {
    async fn publish_repo_push(&self, event: RepoPushEvent) -> Result<(), KafkaError> {
        let key = format!("{}/{}", event.owner, event.repo);
        let payload = serde_json::to_vec(&event)?;

        self.producer
            .send(
                FutureRecord::to(REPO_PUSHED_TOPIC)
                    .key(&key)
                    .payload(&payload),
                Timeout::After(SEND_TIMEOUT),
            )
            .await
            .map_err(|(e, _)| KafkaError::from(e))?;

        Ok(())
    }
}
