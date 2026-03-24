# s2-common

Shared library crate for S2 durable streams. Provides common types, serialization primitives, and utilities consumed by `s2-api`, `s2-sdk`, and `s2-server`.

## Build & Test

```bash
cargo check -p s2-common
cargo test -p s2-common
cargo +nightly fmt
```

## Module Overview

| Module | Purpose |
|--------|---------|
| `record` | Core record types, binary encoding, batching |
| `types` | Basin/stream naming, config, access control, pagination |
| `maybe` | `Maybe<T>` — distinguishes "not provided" from explicit `None` (used for PATCH semantics) |
| `read_extent` | `ReadLimit` and `ReadUntil` — read throttling types |
| `bash` | BLAKE3 hashing wrapper with unambiguous component encoding |
| `deep_size` | `DeepSize` trait — heap size calculation for billing |
| `http` | `ParseableHeader` trait + optional Axum header extractors |
| `caps` | System limit constants |

## Key Types

### Records (`record/`)
- `Record` — enum: `Command(CommandRecord)` or `Envelope(EnvelopeRecord)`
- `SequencedRecord` — record + `StreamPosition` (seq_num: u64, timestamp: u64)
- `RecordBatch` — collection of sequenced records with `is_terminal` flag
- `RecordBatcher<I, E>` — iterator adapter that groups records into batches (max 1000 records or 1 MiB)
- `Metered<T>` — caches billable size; separate from encoded size
- `FencingToken` — string token ≤36 bytes for atomic write sequencing
- `Encodable` trait — `encode_into(&mut buf)` + `encoded_size()`
- `MeteredSize` trait — `metered_size()` for billing

### Types (`types/`)
- `BasinName` / `BasinNamePrefix` — validated names (8–48 bytes, lowercase + digits + `_./- `)
- `StreamName` / `StreamNamePrefix` — validated names (1–512 bytes)
- `StreamConfig` / `StreamReconfiguration` — resolved vs PATCH config (uses `Maybe<T>` fields)
- `BasinConfig` / `BasinReconfiguration` — basin-level config
- `StorageClass` — `Standard` | `Express`
- `RetentionPolicy` — `Age(Duration)` | `Infinite`
- `TimestampingMode` — `ClientPrefer` | `ClientRequire` | `Arrival`
- `Operation` — 16-variant enum, `EnumSet`-compatible for ACL bitmasks
- `Page<T>` — paginated response with `has_more`
- `ListLimit` — `NonZeroUsize` capped at 1000
- `RequestToken` — idempotency token ≤36 bytes

## Configuration Patterns

Three layers for stream/basin config:
1. **Resolved** (`StreamConfig`) — concrete values, used at runtime
2. **Optional** (`OptionalStreamConfig`) — `Option<T>` fields, internal storage
3. **Reconfiguration** (`StreamReconfiguration`) — `Maybe<T>` fields, PATCH semantics

`Maybe<T>` distinguishes "field not sent" (`Unspecified`) from "field explicitly set" (`Specified(v)`).

## Features

- `axum` — enables Axum `FromRequestParts` header extractors (`Header<T>`, `HeaderOpt<T>`)
- `clap` — enables CLI arg parsing support
- `rkyv` — enables rkyv serialization

## Dependencies on this Crate

```
s2-common ← s2-api ← s2-server
s2-common ← s2-sdk
s2-common ← s2-server (direct, with clap + axum features)
```

## Testing

Uses `proptest` for property-based roundtrip tests (encode/decode) and `rstest` for parametrized cases. Tests are inline (`#[cfg(test)]`) within each module.
