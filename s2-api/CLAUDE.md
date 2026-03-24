# s2-api

API types and protocol definitions for S2, a durable streams service. Acts as the shared contract layer between S2 clients and servers.

## Purpose

- Define request/response types for basins (containers) and streams (append-only logs)
- HTTP protocol extraction via Axum extractors (feature-gated)
- Binary S2S streaming protocol with compression
- Server-Sent Events (SSE) for resumable reads
- Format negotiation: JSON, Protobuf, Base64

## Module Structure

```
src/
  lib.rs              — exports data, mime, v1 modules
  data.rs             — Json<T>/Proto<T> response wrappers, Format enum, S2FormatHeader
  mime.rs             — Content-Type/Accept parsing, JsonOrProto enum
  v1/
    mod.rs            — path extractors, list request macro, S2RequestTokenHeader
    error.rs          — ErrorCode enum (22 codes), ErrorResponse, HTTP status mapping
    basin.rs          — ListBasins, CreateBasin, BasinInfo, BasinState, BasinScope
    config.rs         — StorageClass, RetentionPolicy, TimestampingConfig, StreamConfig, BasinConfig
    stream/
      mod.rs          — ReadRequest, AppendRequest, AppendRecord, ReadBatch, SequencedRecord
      extract.rs      — Axum FromRequest impls for AppendRequest and ReadRequest
      sse.rs          — LastEventId, ReadEvent (Batch/Error/Ping/Done), SSE protocol
      s2s.rs          — S2S binary framing, FrameDecoder, compression (Zstd/Gzip)
      proto/
        mod.rs        — Proto↔API type conversions
        s2.v1.rs      — prost-generated Protobuf types (do not edit)
```

## Key Patterns

- **Format negotiation:** Content-Type/Accept headers select JSON vs Protobuf; `data.rs` and `mime.rs` handle this
- **Axum feature:** Extractors and `IntoResponse` impls are behind `features = ["axum"]`
- **S2S framing:** 3-byte length + 1-byte flags + payload; max 2 MiB; terminal frame carries HTTP status + JSON body
- **Compression:** Zstd preferred, Gzip fallback; auto-skipped below 1 KiB threshold
- **SSE checkpointing:** `LastEventId` encodes `seq_num,count,bytes` for resumable reads
- **Conversions:** Extensive `From`/`TryFrom` between API types, proto types, and `s2_common::types`
- **Config merging:** `StreamConfig` and `BasinConfig` support partial updates; proptest covers round-trips

## Protobuf Codegen

The generated file `src/v1/stream/proto/s2.v1.rs` is checked in. To regenerate:

```bash
cargo build -p s2-api --features codegen
```

## Testing

Property-based tests live inline in `config.rs` and `s2s.rs` (proptest + rstest). Run with:

```bash
cargo test -p s2-api
```
