# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
cargo check -p gitdot-cli            # Type check
cargo build -p gitdot-cli            # Build
cargo run -p gitdot-cli -- <cmd>     # Run (e.g., `-- auth login`)
cargo test -p gitdot-cli             # Run tests
cargo +nightly fmt -p gitdot-cli     # Format
cargo build -p gitdot-cli --release  # Release build (binary: gdot)
```

## Architecture

The binary is named `gdot`. Entry point: `src/bin/main.rs` → initializes rustls (`bootstrap::load_rustls()`) → parses CLI args → routes to command.

### Feature Flags

Two compile-time features (both default):
- **`main`** — user-facing commands: `auth`, `review`, `ci`
- **`runner`** — daemon commands: `runner` (install/run/start/stop/config/verify), `executor`, OS service management

`src/command.rs` uses `#[cfg(feature = "...")]` on clap variants to gate entire subcommands.

### Command Flow

Each command follows: `load config → build client → execute`

- `auth`/`review` → `UserConfig::load()` (`~/.config/gitdot/config.toml`) + `GitdotClient` with JWT
- `runner` → `RunnerConfig::load()` (`/etc/gitdot/runner.toml`) + `GitdotClient` with Basic auth

### Key Modules

| Module | Role |
|--------|------|
| `client.rs` + `client/methods/` | `GitdotClient`: reqwest wrapper with JWT or Basic auth; methods grouped by domain (oauth, runner, task, user) |
| `config/user.rs` | `UserConfig`: async load/save, `~/.config/gitdot/config.toml` |
| `config/runner.rs` | `RunnerConfig`: sync load/save, `/etc/gitdot/runner.toml` |
| `git.rs` | `GitWrapper`: async wrapper over git CLI via `tokio::process::Command` |
| `store.rs` | Credential storage via `git credential approve` |
| `executor/local.rs` | `LocalExecutor`: clones repo into `/tmp/gitdot/tasks/{id}`, runs `sh -c` command, streams output to S2 |
| `os/` | `Service` trait with `launchd` (macOS) and `systemd` (Linux) implementations via `#[cfg(target_os)]` |
| `util/ci.rs` | Finds `.gitdot-ci.toml` by walking up from cwd |

### Config Defaults

```
Web:    https://www.gitdot.io
API:    https://api.gitdot.io
S2:     https://s2.gitdot.io
```

Overridden by `gitdot_server_url` / `gitdot_web_url` / `s2_server_url` in config files.
