use futures::StreamExt;

#[cfg(feature = "_hidden")]
use crate::client::Connect;
use crate::{
    api::{AccountClient, BaseClient, BasinClient},
    producer::{Producer, ProducerConfig},
    session::{self, AppendSession, AppendSessionConfig},
    types::{
        AppendAck, AppendInput, BasinInfo, BasinName, CreateBasinInput, CreateStreamInput,
        DeleteBasinInput, DeleteStreamInput, ListAllStreamsInput, ListStreamsInput, Page,
        ReadBatch, ReadInput, S2Config, S2Error, StreamInfo, StreamName, StreamPosition, Streaming,
    },
};

#[derive(Debug, Clone)]
/// An S2 account.
pub struct S2 {
    client: AccountClient,
}

impl S2 {
    /// Create a new [`S2`].
    pub fn new(config: S2Config) -> Result<Self, S2Error> {
        let base_client = BaseClient::init(&config)?;
        Ok(Self {
            client: AccountClient::init(config, base_client),
        })
    }

    /// Create a new [`S2`] using a single URL for both account and basin endpoints.
    pub fn from_url(url: &str) -> Result<Self, S2Error> {
        let endpoints = crate::types::S2Endpoints::from_url(url)?;
        let config = S2Config::new().with_endpoints(endpoints);
        Self::new(config)
    }

    /// Return a clone of this [`S2`] with a Bearer token injected into all requests.
    /// The underlying connection pool is shared; only headers are duplicated.
    pub fn with_auth(&self, token: impl AsRef<str>) -> Self {
        let mut cloned = self.clone();
        cloned.client.client = cloned.client.client.with_auth(token);
        cloned
    }

    /// Get an [`S2Basin`].
    pub fn basin(&self, name: BasinName) -> S2Basin {
        S2Basin {
            client: self.client.basin_client(name),
        }
    }

    /// Create a basin.
    pub async fn create_basin(&self, input: CreateBasinInput) -> Result<BasinInfo, S2Error> {
        let (request, idempotency_token) = input.into();
        let info = self.client.create_basin(request, idempotency_token).await?;
        Ok(info.into())
    }

    /// Delete a basin.
    pub async fn delete_basin(&self, input: DeleteBasinInput) -> Result<(), S2Error> {
        Ok(self
            .client
            .delete_basin(input.name, input.ignore_not_found)
            .await?)
    }
}

#[derive(Debug, Clone)]
/// A basin in an S2 account.
///
/// See [`S2::basin`].
pub struct S2Basin {
    client: BasinClient,
}

impl S2Basin {
    /// Get an [`S2Stream`].
    pub fn stream(&self, name: StreamName) -> S2Stream {
        S2Stream {
            client: self.client.clone(),
            name,
        }
    }

    /// List a page of streams.
    ///
    /// See [`list_all_streams`](crate::S2Basin::list_all_streams) for automatic pagination.
    pub async fn list_streams(&self, input: ListStreamsInput) -> Result<Page<StreamInfo>, S2Error> {
        let response = self.client.list_streams(input.into()).await?;
        Ok(Page::new(
            response
                .streams
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            response.has_more,
        ))
    }

    /// List all streams, paginating automatically.
    pub fn list_all_streams(&self, input: ListAllStreamsInput) -> Streaming<StreamInfo> {
        let basin = self.clone();
        let prefix = input.prefix;
        let start_after = input.start_after;
        let include_deleted = input.include_deleted;
        let mut input = ListStreamsInput::new()
            .with_prefix(prefix)
            .with_start_after(start_after);
        Box::pin(async_stream::try_stream! {
            loop {
                let page = basin.list_streams(input.clone()).await?;

                let start_after = page.values.last().map(|info| info.name.clone().into());
                for info in page.values {
                    if !include_deleted && info.deleted_at.is_some() {
                        continue;
                    }
                    yield info;
                }

                if page.has_more && let Some(start_after) = start_after {
                    input = input.with_start_after(start_after);
                } else {
                    break;
                }
            }
        })
    }

    /// Create a stream.
    pub async fn create_stream(&self, input: CreateStreamInput) -> Result<StreamInfo, S2Error> {
        let (request, idempotency_token) = input.into();
        let info = self
            .client
            .create_stream(request, idempotency_token)
            .await?;
        Ok(info.try_into()?)
    }

    /// Delete a stream.
    pub async fn delete_stream(&self, input: DeleteStreamInput) -> Result<(), S2Error> {
        Ok(self
            .client
            .delete_stream(input.name, input.ignore_not_found)
            .await?)
    }
}

#[derive(Debug, Clone)]
/// A stream in an S2 basin.
///
/// See [`S2Basin::stream`].
pub struct S2Stream {
    client: BasinClient,
    name: StreamName,
}

impl S2Stream {
    /// Check tail position.
    pub async fn check_tail(&self) -> Result<StreamPosition, S2Error> {
        let response = self.client.check_tail(&self.name).await?;
        Ok(response.tail.into())
    }

    /// Append records.
    pub async fn append(&self, input: AppendInput) -> Result<AppendAck, S2Error> {
        let retry_enabled = self
            .client
            .config
            .retry
            .append_retry_policy
            .is_compliant(&input);
        let ack = self
            .client
            .append(&self.name, input.into(), retry_enabled)
            .await?;
        Ok(ack.into())
    }

    /// Read records.
    pub async fn read(&self, input: ReadInput) -> Result<ReadBatch, S2Error> {
        let batch = self
            .client
            .read(&self.name, input.start.into(), input.stop.into())
            .await?;
        Ok(ReadBatch::from_api(batch, input.ignore_command_records))
    }

    /// Create an append session for submitting [`AppendInput`]s.
    pub fn append_session(&self, config: AppendSessionConfig) -> AppendSession {
        AppendSession::new(self.client.clone(), self.name.clone(), config)
    }

    /// Create a producer for submitting individual [`AppendRecord`](crate::types::AppendRecord)s.
    pub fn producer(&self, config: ProducerConfig) -> Producer {
        Producer::new(self.client.clone(), self.name.clone(), config)
    }

    /// Create a read session.
    pub async fn read_session(&self, input: ReadInput) -> Result<Streaming<ReadBatch>, S2Error> {
        let batches = session::read_session(
            self.client.clone(),
            self.name.clone(),
            input.start.into(),
            input.stop.into(),
            input.ignore_command_records,
        )
        .await?;
        Ok(Box::pin(batches.map(|res| match res {
            Ok(batch) => Ok(batch),
            Err(err) => Err(err.into()),
        })))
    }
}
