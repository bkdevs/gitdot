## s2-sdk

### Overview

`s2-sdk` is an async Rust SDK for [S2](https://s2.dev/), a durable streams platform. It provides ergonomic bindings for managing basins and streams — creating, deleting, listing, and performing high-throughput record I/O — built on top of hyper HTTP/2 with rustls for transport and prost for protobuf serialization.

The SDK exposes three top-level handle types — `S2`, `S2Basin`, and `S2Stream` — that reflect the S2 resource hierarchy. Append workloads are served at multiple abstraction levels: a single `append` call for simple use cases, an `AppendSession` for pipelined HTTP/2 batches with backpressure, and a `Producer` that further abstracts individual record submission with automatic batching and per-record acknowledgement tickets.

### APIs

- **`S2`** — account-level handle ([s2-sdk/src/ops.rs](s2-sdk/src/ops.rs))
  - `S2::new(config: S2Config) -> Result<Self, S2Error>` — construct from config
  - `S2::from_url(url: &str) -> Result<Self, S2Error>` — single URL for account + basin
  - `S2::with_auth(token) -> Self` — clone handle with a Bearer token injected
  - `S2::basin(name: BasinName) -> S2Basin` — get a basin handle
  - `async S2::create_basin(input: CreateBasinInput) -> Result<BasinInfo, S2Error>`
  - `async S2::delete_basin(input: DeleteBasinInput) -> Result<(), S2Error>`

  ```rust
  let s2 = S2::new(S2Config::new().with_endpoints(S2Endpoints::from_env()?))?;
  let s2 = s2.with_auth("my-token");
  let basin = s2.basin("my-basin".parse()?);
  ```

- **`S2Basin`** — basin-level handle ([s2-sdk/src/ops.rs](s2-sdk/src/ops.rs))
  - `S2Basin::stream(name: StreamName) -> S2Stream` — get a stream handle
  - `async S2Basin::list_streams(input: ListStreamsInput) -> Result<Page<StreamInfo>, S2Error>`
  - `S2Basin::list_all_streams(input: ListAllStreamsInput) -> Streaming<StreamInfo>` — auto-paginating stream
  - `async S2Basin::create_stream(input: CreateStreamInput) -> Result<StreamInfo, S2Error>`
  - `async S2Basin::delete_stream(input: DeleteStreamInput) -> Result<(), S2Error>`

  ```rust
  let mut streams = basin.list_all_streams(ListAllStreamsInput::new());
  while let Some(info) = streams.next().await {
      println!("{}", info?.name);
  }
  ```

- **`S2Stream`** — stream-level handle ([s2-sdk/src/ops.rs](s2-sdk/src/ops.rs))
  - `async S2Stream::check_tail() -> Result<StreamPosition, S2Error>`
  - `async S2Stream::append(input: AppendInput) -> Result<AppendAck, S2Error>`
  - `async S2Stream::read(input: ReadInput) -> Result<ReadBatch, S2Error>`
  - `S2Stream::append_session(config: AppendSessionConfig) -> AppendSession`
  - `S2Stream::producer(config: ProducerConfig) -> Producer`
  - `async S2Stream::read_session(input: ReadInput) -> Result<Streaming<ReadBatch>, S2Error>`

- **`S2Config`** — client configuration ([s2-sdk/src/types.rs](s2-sdk/src/types.rs))
  - `S2Config::new()` — defaults to AWS endpoints, 3s connection timeout, 5s request timeout
  - `.with_endpoints(S2Endpoints)` — override account + basin endpoints
  - `.with_connection_timeout(Duration)`
  - `.with_request_timeout(Duration)`
  - `.with_retry(RetryConfig)`
  - `.with_compression(Compression)` — `None` | `Gzip` | `Zstd`
  - `.with_insecure_skip_cert_verification(bool)` — dev/test only

- **`S2Endpoints`** — endpoint configuration ([s2-sdk/src/types.rs](s2-sdk/src/types.rs))
  - `S2Endpoints::from_env()` — reads `S2_ACCOUNT_ENDPOINT` + `S2_BASIN_ENDPOINT`
  - `S2Endpoints::from_url(url)` — single URL for both endpoints
  - `S2Endpoints::new(account, basin)` — explicit account + basin endpoints

- **`RetryConfig`** — retry policy ([s2-sdk/src/types.rs](s2-sdk/src/types.rs))
  - Fields: `max_attempts` (default 3), `min_base_delay` (100ms), `max_base_delay` (1s), `append_retry_policy`
  - `.with_max_attempts(NonZeroU32)`
  - `.with_append_retry_policy(AppendRetryPolicy)` — `All` | `NoSideEffects`

- **`AppendSession`** — pipelined append session ([s2-sdk/src/session/append.rs](s2-sdk/src/session/append.rs))
  - Created via `S2Stream::append_session(config)`
  - `async AppendSession::submit(input: AppendInput) -> Result<BatchSubmitTicket, S2Error>` — backpressure-aware submit
  - `async AppendSession::reserve(bytes: u32) -> Result<BatchSubmitPermit, S2Error>` — explicit permit for `select!` loops
  - `async AppendSession::close(self) -> Result<(), S2Error>` — flush and close
  - `BatchSubmitTicket` — future resolving to `AppendAck` once the batch is confirmed

  ```rust
  let session = stream.append_session(AppendSessionConfig::new());
  let ticket = session.submit(AppendInput::new(records)).await?;
  session.close().await?;
  let ack = ticket.await?;
  ```

- **`AppendSessionConfig`** — session configuration ([s2-sdk/src/session/append.rs](s2-sdk/src/session/append.rs))
  - `.with_max_unacked_bytes(u32)` — default 5MiB, minimum 1MiB
  - `.with_max_unacked_batches(NonZeroU32)` — default unlimited

- **`Producer`** — record-level producer with auto-batching ([s2-sdk/src/producer.rs](s2-sdk/src/producer.rs))
  - Created via `S2Stream::producer(config)`
  - `async Producer::submit(record: AppendRecord) -> Result<RecordSubmitTicket, S2Error>`
  - `async Producer::reserve(bytes: u32) -> Result<RecordSubmitPermit, S2Error>` — cancel-safe, for `select!` loops
  - `async Producer::close(self) -> Result<(), S2Error>` — flush and close
  - `RecordSubmitTicket` — future resolving to `IndexedAppendAck` with per-record `seq_num`

  ```rust
  let producer = stream.producer(ProducerConfig::new());
  let ticket = producer.submit(AppendRecord::new(b"hello")?).await?;
  producer.close().await?;
  let ack = ticket.await?;
  println!("assigned seq_num: {}", ack.seq_num);
  ```

- **`ProducerConfig`** — producer configuration ([s2-sdk/src/producer.rs](s2-sdk/src/producer.rs))
  - `.with_max_unacked_bytes(u32)` — default 5MiB
  - `.with_batching(BatchingConfig)`
  - `.with_fencing_token(FencingToken)`
  - `.with_match_seq_num(u64)` — starting seq num, auto-incremented per batch

- **`BatchingConfig`** — record batching policy ([s2-sdk/src/batching.rs](s2-sdk/src/batching.rs))
  - `.with_linger(Duration)` — wait for more records before flushing; default 5ms
  - `.with_max_batch_bytes(usize)` — default 1MiB, min 8B
  - `.with_max_batch_records(usize)` — default 1000, min 1
