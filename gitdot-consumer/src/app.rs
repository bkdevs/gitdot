mod bootstrap;
mod runner;
mod settings;
mod state;

use anyhow::Context;
use rdkafka::consumer::Consumer;
use sqlx::PgPool;

pub use settings::Settings;
pub use state::{ConsumerHandle, ConsumerState};

const REPO_PUSHED_TOPIC: &str = "gitdot.repo.pushed";

pub struct GitdotConsumer {
    state: ConsumerState,
    kafka: ConsumerHandle,
}

impl GitdotConsumer {
    pub async fn new() -> anyhow::Result<Self> {
        bootstrap::bootstrap()?;

        let settings = Settings::new()?;
        let pool = PgPool::connect(&settings.database_url).await?;
        let state = ConsumerState::new(settings, pool).await?;
        let kafka = state::build_consumer(&state.settings).await?;
        match &kafka {
            ConsumerHandle::Plain(c) => c
                .subscribe(&[REPO_PUSHED_TOPIC])
                .context("subscribe to topic")?,
            ConsumerHandle::Gcp(c) => c
                .subscribe(&[REPO_PUSHED_TOPIC])
                .context("subscribe to topic")?,
        }

        Ok(Self { state, kafka })
    }

    pub async fn run(self) -> anyhow::Result<()> {
        tracing::info!(
            topic = REPO_PUSHED_TOPIC,
            group_id = %self.state.settings.kafka_consumer_group_id,
            "starting consumer",
        );
        match self.kafka {
            ConsumerHandle::Plain(c) => runner::run(self.state, c).await,
            ConsumerHandle::Gcp(c) => runner::run(self.state, c).await,
        }
    }
}
