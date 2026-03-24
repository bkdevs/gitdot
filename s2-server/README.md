## s2-server

### Overview

`s2-server` is a lightweight HTTP server implementing the [S2](https://s2.dev) durable streams API, backed by object storage via SlateDB. It manages **basins** (logical containers) and **streams** (append-only logs) with a pluggable storage backend layer supporting S3, GCP, local filesystem, or in-memory storage.

The server follows a clean layered architecture: HTTP handlers delegate to backend service methods, which execute reads and writes against a KV schema built on top of SlateDB. Append operations flow through per-stream **streamer** actors that handle sequencing, batching, and durability tracking, while background tasks manage stream trimming, delete-on-empty enforcement, and basin deletion cleanup.

### Files

```
s2-server/
├── CLAUDE.md
├── Cargo.toml
├── Dockerfile
├── LICENSE
├── src
│   ├── auth.rs
│   ├── backend
│   │   ├── append.rs
│   │   ├── basins.rs
│   │   ├── bgtasks
│   │   │   ├── basin_deletion.rs
│   │   │   ├── mod.rs
│   │   │   ├── stream_doe.rs
│   │   │   └── stream_trim.rs
│   │   ├── core.rs
│   │   ├── error.rs
│   │   ├── kv
│   │   │   ├── basin_deletion_pending.rs
│   │   │   ├── basin_meta.rs
│   │   │   ├── mod.rs
│   │   │   ├── stream_doe_deadline.rs
│   │   │   ├── stream_fencing_token.rs
│   │   │   ├── stream_id_mapping.rs
│   │   │   ├── stream_meta.rs
│   │   │   ├── stream_record_data.rs
│   │   │   ├── stream_record_timestamp.rs
│   │   │   ├── stream_tail_position.rs
│   │   │   ├── stream_trim_point.rs
│   │   │   └── timestamp.rs
│   │   ├── mod.rs
│   │   ├── read.rs
│   │   ├── store.rs
│   │   ├── stream_id.rs
│   │   ├── streamer.rs
│   │   └── streams.rs
│   ├── bin
│   │   └── main.rs
│   ├── handlers
│   │   ├── mod.rs
│   │   └── v1
│   │       ├── basins.rs
│   │       ├── error.rs
│   │       ├── mod.rs
│   │       ├── paths.rs
│   │       ├── records.rs
│   │       └── streams.rs
│   ├── init.rs
│   ├── lib.rs
│   └── server.rs
└── tests
    ├── backend
    │   ├── common.rs
    │   ├── control_plane
    │   │   ├── basin.rs
    │   │   ├── mod.rs
    │   │   └── stream.rs
    │   └── data_plane
    │       ├── append.rs
    │       ├── auto_create.rs
    │       ├── mixed.rs
    │       ├── mod.rs
    │       ├── read.rs
    │       └── read_follow.rs
    └── backend_tests.rs
```

### APIs

- **[`src/auth.rs`](s2-server/src/auth.rs)** — JWT authentication
  - `trait Authenticator` — async auth provider
    - `async fn authenticate(parts: &Parts, backend: &Backend) -> Result<Principal<Self>, AuthError>`
  - `struct Principal<A: Authenticator>` — authenticated identity wrapper
    - `fn new(id: Uuid) -> Self`
  - `struct Internal` — GitDot server-to-server authenticator; validates JWT with `sub = "gitdot-server"`
  - `struct TaskJwt` — task-scoped JWT authenticator; parses UUID from `sub` field
  - `enum AuthError` — `MissingHeader | InvalidHeaderFormat | InvalidToken | InvalidPublicKey`; implements `IntoResponse` with `401 Unauthorized`

- **[`src/server.rs`](s2-server/src/server.rs)** — server initialization
  - `struct LiteArgs` — CLI args: `local_root`, `port`, `tls`, `no_cors`, `init_file`
  - `enum StoreType` — storage backend selector: `S3Bucket | GcpBucket | LocalFileSystem | InMemory`
    - `fn default_flush_interval(&self) -> Duration`
  - `async fn run(args: LiteArgs) -> eyre::Result<()>` — main server entry point; detects storage backend, initializes SlateDB, starts Axum

- **[`src/init.rs`](s2-server/src/init.rs)** — declarative JSON initialization
  - `struct ResourcesSpec` — root spec: `basins: Vec<BasinSpec>`
  - `struct BasinSpec` — `name`, `config: Option<BasinConfigSpec>`, `streams: Vec<StreamSpec>`
  - `struct StreamSpec` — `name`, `config: Option<StreamConfigSpec>`
  - `fn load(path: &Path) -> eyre::Result<ResourcesSpec>` — parse JSON spec file
  - `fn validate(spec: &ResourcesSpec) -> eyre::Result<()>` — validate names and uniqueness
  - `async fn apply(backend: &Backend, spec: ResourcesSpec) -> eyre::Result<()>` — apply spec to live backend
  - Example init file:
    ```json
    {
      "basins": [{
        "name": "my-basin",
        "config": { "create_stream_on_append": true },
        "streams": [{ "name": "events" }]
      }]
    }
    ```

- **[`src/backend/core.rs`](s2-server/src/backend/core.rs)** — main `Backend` struct
  - `struct Backend` — holds SlateDB, streamer slots, auth key, bgtask channel
    - `fn new(db: slatedb::Db, append_inflight_max: ByteSize, gitdot_public_key: String) -> Self`
    - `async fn streamer_client(&self, basin, stream) -> Result<StreamerClient, StreamerError>` — get or init streamer
    - `fn streamer_client_if_active(&self, basin, stream) -> Option<StreamerClient>` — non-blocking check
    - `async fn streamer_client_with_auto_create<E>(&self, basin, stream, should_auto_create) -> Result<StreamerClient, E>`
    - `fn bgtask_trigger(&self, trigger: BgtaskTrigger)` — fire background task
  - `enum StreamerClientSlot` — `Initializing { init_id, future } | Ready { client }`

- **[`src/backend/basins.rs`](s2-server/src/backend/basins.rs)** — basin lifecycle
  - `async fn list_basins(&self, request: ListBasinsRequest) -> Result<Page<BasinInfo>, ListBasinsError>`
  - `async fn create_basin(&self, basin, config, mode: CreateMode) -> Result<CreatedOrReconfigured<BasinInfo>, CreateBasinError>`
  - `async fn get_basin_config(&self, basin) -> Result<BasinConfig, GetBasinConfigError>`
  - `async fn reconfigure_basin(&self, basin, reconfig: BasinReconfiguration) -> Result<BasinConfig, ReconfigureBasinError>`
  - `async fn delete_basin(&self, basin) -> Result<(), DeleteBasinError>`

- **[`src/backend/streams.rs`](s2-server/src/backend/streams.rs)** — stream lifecycle
  - `async fn list_streams(&self, basin, request: ListStreamsRequest) -> Result<Page<StreamInfo>, ListStreamsError>`
  - `async fn create_stream(&self, basin, stream, config, mode) -> Result<CreatedOrReconfigured<StreamInfo>, CreateStreamError>`
  - `async fn get_stream_config(&self, basin, stream) -> Result<OptionalStreamConfig, GetStreamConfigError>`
  - `async fn reconfigure_stream(&self, basin, stream, reconfig) -> Result<OptionalStreamConfig, ReconfigureStreamError>`
  - `async fn delete_stream(&self, basin, stream) -> Result<(), DeleteStreamError>` — terminal trim then mark deleted

- **[`src/backend/append.rs`](s2-server/src/backend/append.rs)** — record append
  - `async fn append(&self, basin, stream, input: AppendInput) -> Result<AppendAck, AppendError>`
  - `async fn append_session(self, basin, stream, inputs: impl Stream<Item = AppendInput>) -> Result<impl Stream<Item = Result<AppendAck, AppendError>>, AppendError>`
  - `struct PendingAppends` — durability queue
    - `fn accept(&mut self, ticket: Ticket, ack_range: Range<StreamPosition>)`
    - `fn on_stable(&mut self, stable_pos: StreamPosition)` — complete appends when durable
    - `fn on_durability_failed(self, err: slatedb::Error)` — fail all pending
  - `struct Ticket` — permission slot to enqueue an append
    - `fn accept(self, ack_range) -> BlockedReplySender`
    - `fn reject(self, err, stable_pos) -> Option<BlockedReplySender>`

- **[`src/backend/read.rs`](s2-server/src/backend/read.rs)** — record read
  - `async fn check_tail(&self, basin, stream) -> Result<StreamPosition, CheckTailError>`
  - `async fn read(&self, basin, stream, start: ReadStart, end: ReadEnd) -> Result<impl Stream<Item = Result<ReadSessionOutput, ReadError>>, ReadError>`
  - `async fn resolve_timestamp(&self, stream_id, timestamp) -> Result<Option<StreamPosition>, StorageError>`

- **[`src/backend/streamer.rs`](s2-server/src/backend/streamer.rs)** — per-stream sequencing actor
  - `struct Spawner` — configures and launches a streamer background task
    - `fn spawn(self, on_exit: impl FnOnce(StreamerId)) -> StreamerClient`
  - `struct Streamer` — runtime instance managing sequencing, batching, fencing, and DOE deadlines
    - `fn next_assignable_pos(&self) -> StreamPosition`
    - `fn sequence_records(&self, input: AppendInput) -> Result<Vec<Metered<SequencedRecord>>, AppendErrorInternal>`
  - `struct CommandState<T>` — tracks when a command (trim point, fencing token) has been applied to the log
    - `fn is_applied_in(&self, seq_num_range: &Range<SeqNum>) -> bool`
  - Constants: `DORMANT_TIMEOUT = 60s`, `DOE_DEADLINE_REFRESH_PERIOD = 600s`

- **[`src/backend/store.rs`](s2-server/src/backend/store.rs)** — DB access helpers
  - `async fn db_status(&self) -> Result<(), slatedb::Error>` — health check
  - `async fn db_get<K, V>(&self, key, deser) -> Result<Option<V>, StorageError>` — get with remote durability filter
  - `async fn db_txn_get<K, V>(txn, key, deser) -> Result<Option<V>, StorageError>` — transactional get

- **[`src/backend/error.rs`](s2-server/src/backend/error.rs)** — domain errors
  - `enum StorageError` — `Deserialization | Database`
  - `enum AppendError` — covers storage, fencing, condition failures, stream-not-found, deletion-pending, etc.
  - `enum ReadError` — covers storage, unwritten, stream-not-found, etc.
  - `enum AppendConditionFailedError` — `FencingTokenMismatch | SeqNumMismatch`
  - Per-operation error enums for all CRUD operations: `CreateBasinError`, `DeleteBasinError`, `ListBasinsError`, `ReconfigureBasinError`, `CreateStreamError`, `DeleteStreamError`, etc.

- **[`src/handlers/v1/basins.rs`](s2-server/src/handlers/v1/basins.rs)** — basin HTTP handlers
  - `fn router() -> axum::Router<Backend>`
  - `async fn list_basins(auth, State(backend), ListArgs) -> Result<Json<ListBasinsResponse>, ServiceError>`
  - `async fn create_basin(auth, State(backend), CreateArgs) -> Result<(StatusCode, Json<BasinInfo>), ServiceError>` — `201 Created`
  - `async fn delete_basin(auth, State(backend), DeleteArgs) -> Result<StatusCode, ServiceError>` — `202 Accepted`

- **[`src/handlers/v1/streams.rs`](s2-server/src/handlers/v1/streams.rs)** — stream HTTP handlers
  - `fn router() -> axum::Router<Backend>`
  - `async fn list_streams`, `async fn create_stream`, `async fn delete_stream`

- **[`src/handlers/v1/records.rs`](s2-server/src/handlers/v1/records.rs)** — record HTTP handlers
  - `fn router() -> axum::Router<Backend>`
  - `async fn check_tail(auth: Principal<TaskJwt>, State(backend), CheckTailArgs) -> Result<Json<TailResponse>, ServiceError>`
  - `async fn read(auth: Principal<TaskJwt>, State(backend), ReadArgs) -> Result<Response, ServiceError>` — supports SSE streaming and unary modes
  - `async fn append(auth: Principal<TaskJwt>, State(backend), AppendArgs) -> Result<Response, ServiceError>`

- **[`src/handlers/v1/paths.rs`](s2-server/src/handlers/v1/paths.rs)** — URL path constants
  - `basins::LIST = "/basins"`, `basins::CREATE = "/basins"`, `basins::DELETE = "/basins/{basin}"`
  - `streams::LIST = "/streams"`, `streams::CREATE = "/streams"`, `streams::DELETE = "/streams/{stream}"`
  - `streams::records::CHECK_TAIL = "/streams/{stream}/records/tail"`
  - `streams::records::READ = "/streams/{stream}/records"`
  - `streams::records::APPEND = "/streams/{stream}/records"`

- **[`src/handlers/v1/error.rs`](s2-server/src/handlers/v1/error.rs)** — HTTP error mapping
  - `enum ServiceError` — maps all domain errors to HTTP responses; implements `IntoResponse`
    - `fn to_response(&self) -> ErrorResponse`
