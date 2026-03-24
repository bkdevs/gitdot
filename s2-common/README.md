## s2-common

### Overview

`s2-common` is the shared library crate for the S2 durable streams system, providing the core record model, binary encoding, naming types, and configuration primitives used across `s2-api`, `s2-sdk`, and `s2-server`. It defines the canonical on-wire format for records, the type-safe naming conventions for basins and streams, and the three-tier configuration system that supports both full resolution and PATCH-style partial updates.

The crate is intentionally dependency-light and feature-gated: the `axum` feature enables Axum extractor implementations for HTTP header types, `clap` enables CLI parsing support, and `rkyv` enables binary serialization. Everything else is available unconditionally, keeping SDK and server consumers from pulling in unnecessary dependencies.

### Files

```
s2-common/
├── Cargo.toml
└── src
    ├── lib.rs
    ├── bash.rs            # BLAKE3 hashing wrapper (Bash)
    ├── caps.rs            # System limit constants
    ├── deep_size.rs       # DeepSize trait for heap billing
    ├── http.rs            # ParseableHeader trait + optional Axum extractors
    ├── maybe.rs           # Maybe<T> — PATCH-semantics optional
    ├── read_extent.rs     # ReadLimit / ReadUntil — read throttling
    ├── record
    │   ├── mod.rs         # Record, SequencedRecord, Encodable, MagicByte, StreamPosition
    │   ├── batcher.rs     # RecordBatch, RecordBatcher<I, E>
    │   ├── command.rs     # CommandRecord (trim, fence)
    │   ├── envelope.rs    # EnvelopeRecord (headers + body)
    │   ├── fencing.rs     # FencingToken
    │   └── metering.rs    # Metered<T>, MeteredSize
    └── types
        ├── mod.rs         # ValidationError
        ├── access.rs      # Operation enum, ResourceSet
        ├── basin.rs       # BasinName, BasinNamePrefix, BasinInfo, BasinState
        ├── config.rs      # StreamConfig, BasinConfig, reconfiguration types
        ├── resources.rs   # Page<T>, ListLimit, ListItemsRequest, RequestToken, CreateMode
        ├── stream.rs      # StreamName, StreamNamePrefix, StreamInfo
        └── strings.rs     # StrProps marker traits for name validation
```

### APIs

- **`record`** ([s2-common/src/record/mod.rs](s2-common/src/record/mod.rs))
  - `Record` — top-level record enum
    - `Record::Command(CommandRecord)` — system command (trim, fence)
    - `Record::Envelope(EnvelopeRecord)` — user data with typed headers and body
    - `Record::try_from_parts(headers: Vec<Header>, body: Bytes) -> Result<Self, PublicRecordError>` — construct from wire parts; empty header name signals a command record
    - `Record::into_parts(self) -> (Vec<Header>, Bytes)` — decompose back to wire parts
    - `Record::sequenced(self, position: StreamPosition) -> SequencedRecord`
    ```rust
    let record = Record::try_from_parts(headers, body)?;
    let sequenced = record.sequenced(StreamPosition { seq_num: 1, timestamp: 1000 });
    ```
  - `SequencedRecord` — record bound to a `StreamPosition`
    - Fields: `position: StreamPosition`, `record: Record`
  - `StreamPosition` — `{ seq_num: u64, timestamp: u64 }`; `StreamPosition::MIN` is the zero position
  - `Encodable` trait ([s2-common/src/record/mod.rs](s2-common/src/record/mod.rs))
    - `fn encoded_size(&self) -> usize`
    - `fn encode_into(&self, buf: &mut impl BufMut)`
    - `fn to_bytes(&self) -> Bytes` — default impl via `encode_into`
  - `FencingToken` ([s2-common/src/record/fencing.rs](s2-common/src/record/fencing.rs)) — string token ≤36 bytes for atomic append sequencing

- **`record::metering`** ([s2-common/src/record/metering.rs](s2-common/src/record/metering.rs))
  - `MeteredSize` trait — `fn metered_size(&self) -> usize`; implemented for `Record`, `SequencedRecord`, slices, and `Vec`
  - `Metered<T>` — caches billable size alongside the inner value; implements `Deref<Target = T>`
    - `Metered::from(inner: T) -> Self` — computes and caches metered size
    - `Metered::as_ref(&self) -> Metered<&T>`
    - `Metered::into_inner(self) -> T`
    - `Metered<Vec<T>>::push(&mut self, item: Metered<T>)` — appends and accumulates size
    ```rust
    let metered: Metered<Record> = record.into();
    let encoded = metered.as_ref().to_bytes(); // encode with magic byte + metered size prefix
    ```

- **`record::batcher`** ([s2-common/src/record/batcher.rs](s2-common/src/record/batcher.rs))
  - `RecordBatch` — `{ records: Metered<Vec<SequencedRecord>>, is_terminal: bool }`
  - `RecordBatcher<I, E>` — iterator adapter that groups raw `(StreamPosition, Bytes)` pairs into batches respecting `ReadLimit` and `ReadUntil`; capped at 1000 records or 1 MiB per batch
    - `RecordBatcher::new(record_iterator: I, read_limit: ReadLimit, until: ReadUntil) -> Self`
    - `impl Iterator<Item = Result<RecordBatch, InternalRecordError>>`
    ```rust
    let batcher = RecordBatcher::new(iter, ReadLimit::Count(500), ReadUntil::Unbounded);
    for batch in batcher {
        let batch = batch?;
        // batch.is_terminal signals read limit exhausted
    }
    ```

- **`maybe`** ([s2-common/src/maybe.rs](s2-common/src/maybe.rs))
  - `Maybe<T>` — three-state enum for PATCH semantics
    - `Maybe::Unspecified` — field absent; `Default::default()` produces this via `#[serde(default)]`
    - `Maybe::Specified(T)` — field explicitly set (including `null` → `Specified(None)`)
    - `Maybe::map`, `Maybe::unwrap_or_default`, `Maybe::is_unspecified`
    - `Maybe<Option<T>>::map_opt`, `try_map_opt`, `opt_or_default_mut`
    ```rust
    #[derive(Deserialize)]
    struct Patch {
        #[serde(default)]
        pub retention: Maybe<Option<RetentionPolicy>>,
    }
    // "{}" → Unspecified (no change); "{"retention": null}" → Specified(None) (clear)
    ```

- **`read_extent`** ([s2-common/src/read_extent.rs](s2-common/src/read_extent.rs))
  - `ReadLimit` — `Unbounded | Count(usize) | Bytes(usize) | CountOrBytes(CountOrBytes)`
    - `ReadLimit::from_count_and_bytes(count, bytes) -> Self`
    - `ReadLimit::allow(count, bytes) -> bool` / `deny(count, bytes) -> bool`
    - `ReadLimit::remaining(consumed_count, consumed_bytes) -> EvaluatedReadLimit`
  - `ReadUntil` — `Unbounded | Timestamp(u64)`
    - `ReadUntil::allow(timestamp) -> bool` / `deny(timestamp) -> bool`

- **`types::config`** ([s2-common/src/types/config.rs](s2-common/src/types/config.rs))
  - Three-tier config pattern (stream and timestamping/delete-on-empty sub-configs each follow this):
    - `StreamConfig` — resolved, all fields concrete; produced by `OptionalStreamConfig::merge(basin_defaults)`
    - `OptionalStreamConfig` — internal storage with `Option<T>` fields; mutated via `reconfigure()`
    - `StreamReconfiguration` — PATCH update with `Maybe<Option<T>>` fields
  - `StorageClass` — `Standard | Express`
  - `RetentionPolicy` — `Age(Duration) | Infinite()`
  - `TimestampingMode` — `ClientPrefer | ClientRequire | Arrival`
  - `BasinConfig` — `{ default_stream_config: OptionalStreamConfig, create_stream_on_append: bool, create_stream_on_read: bool }`
    - `BasinConfig::reconfigure(self, reconfiguration: BasinReconfiguration) -> Self`
  - `BasinReconfiguration` — PATCH update for `BasinConfig` using `Maybe<T>` fields

- **`types::basin`** ([s2-common/src/types/basin.rs](s2-common/src/types/basin.rs))
  - `BasinName` — validated basin identifier (8–48 bytes, lowercase + digits + `-_.`)
  - `BasinNamePrefix` — relaxed prefix variant for list filtering
  - `BasinInfo` — `{ name: BasinName, scope: Option<BasinScope>, state: BasinState }`
  - `BasinState` — `Active | Creating | Deleting`
  - `BasinScope` — `AwsUsEast1`
  - `ListBasinsRequest` — alias for `ListItemsRequest<BasinNamePrefix, BasinNameStartAfter>`

- **`types::stream`** ([s2-common/src/types/stream.rs](s2-common/src/types/stream.rs))
  - `StreamName` — validated stream name (1–512 bytes)
  - `StreamNamePrefix` — relaxed prefix variant
  - `ListStreamsRequest` — alias for `ListItemsRequest<StreamNamePrefix, StreamNameStartAfter>`

- **`types::resources`** ([s2-common/src/types/resources.rs](s2-common/src/types/resources.rs))
  - `Page<T>` — `{ values: Vec<T>, has_more: bool }`; `Page::new(values, has_more)`, `Page::new_empty()`
  - `ListLimit` — `NonZeroUsize` capped at 1000; `ListLimit::MAX`
  - `ListItemsRequest<P, S>` — validated list request with prefix, start_after, and limit
  - `RequestToken` — idempotency token ≤36 bytes
  - `CreateMode` — `CreateOnly(Option<RequestToken>) | CreateOrReconfigure`

- **`types::access`** ([s2-common/src/types/access.rs](s2-common/src/types/access.rs))
  - `Operation` — 16-variant `EnumSetType`-compatible enum for ACL bitmasks: `ListBasins`, `CreateBasin`, `Append`, `Read`, `Trim`, `Fence`, etc.
  - `ResourceSet<E, P>` — `None | Exact(E) | Prefix(P)`; aliased as `BasinResourceSet`, `StreamResourceSet`

- **`bash`** ([s2-common/src/bash.rs](s2-common/src/bash.rs))
  - `Bash` — BLAKE3 hash (32 bytes) with unambiguous multi-component encoding
    - `Bash::delimited(components: &[&[u8]], delimiter: u8) -> Self`
    - `Bash::length_prefixed(components: &[&[u8]]) -> Self` — avoids separator-ambiguity attacks
    - `Bash::as_bytes(&self) -> &[u8; 32]`; serde uses hex representation

- **`http`** ([s2-common/src/http.rs](s2-common/src/http.rs))
  - `ParseableHeader` trait — `fn name() -> &'static HeaderName`; implemented by `BasinName`, `RequestToken`, `FencingToken`
  - `axum` feature: `Header<T>` and `HeaderOpt<T>` — Axum `FromRequestParts` extractors for any `ParseableHeader`
    - `parse_header<T>(headers: &HeaderMap) -> Result<T, HeaderRejection>`

- **`deep_size`** ([s2-common/src/deep_size.rs](s2-common/src/deep_size.rs))
  - `DeepSize` trait — `fn deep_size(&self) -> usize`; implemented on record and position types for heap billing

- **`caps`** ([s2-common/src/caps.rs](s2-common/src/caps.rs))
  - System-wide limit constants: `RECORD_BATCH_MAX` (1000 records / 1 MiB), `MAX_BASIN_NAME_LEN` (48), `MIN_BASIN_NAME_LEN` (8), etc.
