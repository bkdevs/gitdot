# gitdot-config

Shared configuration types for the Gitdot platform. Currently contains CI pipeline config parsing and validation.

## Purpose

Parses and validates `.gitdot.toml` CI config files (user-authored). Consumed by `gitdot-core` when executing CI pipelines.

## Structure

- `src/ci.rs` — `CiConfig`, `BuildConfig`, `TaskConfig`, `BuildTrigger` types; TOML parsing via `toml::from_str`
- `src/validate.rs` — Validation logic; all checks run together, errors collected and returned as `CiConfigError::Validation(Vec<String>)`
- `src/error.rs` — `CiConfigError` enum (`Parse`, `Validation`, `NoMatchingBuild`, `InvalidTrigger`)

## Config Schema

```toml
[[builds]]
trigger = "pull_request"   # or "push_to_main"
tasks = ["lint", "test"]

[[tasks]]
name = "lint"
command = "cargo clippy"

[[tasks]]
name = "test"
command = "cargo test"
waits_for = ["lint"]       # optional; declares dependency ordering
```

## Validation Rules

All errors are collected before returning (not fail-fast):

- builds and tasks lists must be non-empty
- no duplicate task names
- no duplicate build triggers
- no build with an empty task list
- no task with an empty/whitespace command
- build task references must name defined tasks
- `waits_for` references must name defined tasks
- no task referenced more than once within a single build
- no orphaned tasks (every task must appear in at least one build)
- `waits_for` graph must be a DAG (cycle detection via DFS coloring)

## Commands

```bash
cargo test -p gitdot-config    # run tests (all in src/validate.rs)
cargo check -p gitdot-config   # type check
```
