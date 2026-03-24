## gitdot-cli

### Overview

`gitdot-cli` is the command-line interface for Gitdot, compiled to a binary named `gdot`. It provides user-facing commands for authentication, code review workflows, and CI pipeline management, as well as daemon-mode runner commands for executing CI tasks on self-hosted machines. The CLI is built with Clap and uses compile-time feature flags (`main`, `runner`) to gate entire subcommand trees.

Configuration is split between two TOML files: user config at `~/.config/gitdot/config.toml` (auth tokens, server URLs) and runner config at `/etc/gitdot/runner.toml` (runner token, executor count, S2 stream URL). All API calls go through `GitdotClient`, a thin reqwest wrapper that attaches JWT or Basic auth headers and deserializes responses into `gitdot-api` resource types.

### Files

```
gitdot-cli/
├── Cargo.toml
└── src
    ├── bin
    │   └── main.rs            # Entry point: loads rustls, parses CLI args
    ├── bootstrap.rs           # rustls initialization
    ├── cli.rs                 # Top-level Clap struct
    ├── client.rs              # GitdotClient: reqwest wrapper with auth
    ├── client
    │   └── methods
    │       ├── oauth.rs       # create_device_code, poll_token
    │       ├── runner.rs      # verify_runner
    │       ├── task.rs        # poll_task, update_task
    │       └── user.rs        # get_user, list_user_*, update_current_user
    ├── command.rs             # Clap subcommand enum (feature-gated)
    ├── command
    │   ├── auth.rs            # auth subcommand dispatch
    │   ├── auth
    │   │   ├── login.rs       # OAuth device flow login
    │   │   └── status.rs      # Print current auth status
    │   ├── ci.rs              # ci subcommand dispatch
    │   ├── ci
    │   │   ├── format.rs      # Run formatter
    │   │   ├── init.rs        # Initialize .gitdot-ci.toml
    │   │   ├── lint.rs        # Run linter
    │   │   └── run.rs         # Run CI pipeline locally
    │   ├── review.rs          # review subcommand dispatch
    │   ├── review
    │   │   ├── amend.rs       # Amend + force-push current review
    │   │   ├── checkout.rs    # Checkout a review branch
    │   │   ├── create.rs      # Create a new review
    │   │   └── update.rs      # Update an existing review
    │   ├── runner.rs          # runner subcommand dispatch
    │   └── runner
    │       ├── config.rs      # Print/edit runner config
    │       ├── install.rs     # Install runner as OS service
    │       ├── run.rs         # Run runner in foreground
    │       ├── start.rs       # Start runner service
    │       ├── stop.rs        # Stop runner service
    │       └── verify.rs      # Verify runner token with server
    ├── config.rs              # Re-exports UserConfig / RunnerConfig
    ├── config
    │   ├── default.rs         # Default URL constants
    │   ├── runner.rs          # RunnerConfig (sync load/save via sudo)
    │   └── user.rs            # UserConfig (async load/save)
    ├── executor.rs            # Executor trait
    ├── executor
    │   └── local.rs           # LocalExecutor: clone → sh -c → stream to S2
    ├── git.rs                 # GitWrapper: async tokio::process::Command helpers
    ├── lib.rs
    ├── os.rs
    ├── os
    │   ├── install_service.rs # OS-agnostic service install logic
    │   ├── service.rs         # Service trait + ServiceManager
    │   └── service
    │       ├── launchd.rs     # macOS launchd implementation
    │       └── systemd.rs     # Linux systemd implementation
    ├── store.rs               # GitCredentialStore: git credential approve
    └── util
        ├── ci.rs              # Find .gitdot-ci.toml by walking up from cwd
        ├── command.rs         # run_command helper
        ├── review.rs          # Review branch/refspec utilities
        └── s2.rs              # Parse s2:// URIs
```

### APIs

- **`GitdotClient`** ([gitdot-cli/src/client.rs](gitdot-cli/src/client.rs))
  - `GitdotClient::new(client_id)` — construct with default URLs
  - `.with_web_url(url)` / `.with_server_url(url)` — override endpoints
  - `.with_jwt(token)` / `.with_token(token)` — attach Bearer or Basic auth
  - `::from_user_config(config)` — build from `UserConfig` *(feature = "main")*
  - `::from_runner_config(config)` — build from `RunnerConfig` *(feature = "runner")*
  - `async get/post/patch/delete<T, R>(path, request)` — typed HTTP methods

  ```rust
  let client = GitdotClient::from_user_config(&config)
      .with_jwt(token);
  let user = client.get_current_user(GetCurrentUserRequest {}).await?;
  ```

- **OAuth methods** ([gitdot-cli/src/client/methods/oauth.rs](gitdot-cli/src/client/methods/oauth.rs))
  - `async create_device_code()` → `CreateDeviceCodeResponse`
  - `async poll_token(device_code)` → `PollTokenResponse`

- **User methods** ([gitdot-cli/src/client/methods/user.rs](gitdot-cli/src/client/methods/user.rs))
  - `async get_current_user(request)` → `GetCurrentUserResponse`
  - `async get_user(user_name, request)` → `GetUserResponse`
  - `async has_user(user_name, request)` — HEAD check
  - `async list_user_repositories(user_name, request)` → `ListUserRepositoriesResponse`
  - `async list_user_organizations(user_name, request)` → `ListUserOrganizationsResponse`
  - `async list_user_reviews(user_name, request)` → `ListUserReviewsResponse`
  - `async update_current_user(request)` → `UpdateCurrentUserResponse`

- **Task methods** ([gitdot-cli/src/client/methods/task.rs](gitdot-cli/src/client/methods/task.rs))
  - `async poll_task(request)` → `PollTaskResponse`
  - `async update_task(id, status)` → `UpdateTaskResponse`

- **Runner methods** ([gitdot-cli/src/client/methods/runner.rs](gitdot-cli/src/client/methods/runner.rs))
  - `async verify_runner()` — validates runner token with server

- **`UserConfig`** ([gitdot-cli/src/config/user.rs](gitdot-cli/src/config/user.rs))
  - `async UserConfig::load()` — reads `~/.config/gitdot/config.toml`, returns `Default` if absent
  - `async save(&self)` — writes config, creating parent dirs as needed
  - Fields: `gitdot_server_url`, `gitdot_web_url`, `user_name`, `user_email`

- **`RunnerConfig`** ([gitdot-cli/src/config/runner.rs](gitdot-cli/src/config/runner.rs))
  - `RunnerConfig::load()` — sync read from `/etc/gitdot/runner.toml`
  - `save(&self)` — writes via `sudo cp` from a temp file
  - Fields: `gitdot_server_url`, `gitdot_web_url`, `s2_server_url`, `runner_token`, `num_executors`

- **`GitWrapper`** ([gitdot-cli/src/git.rs](gitdot-cli/src/git.rs))
  - `async remote_url(name)` / `async default_branch()` / `async current_branch()`
  - `async rev_parse(rev)` / `async log_oneline(range)` → `Vec<(hash, subject)>`
  - `async pull_rebase(branch)` / `async push_refspec(refspec)`
  - `async add_all()` / `async commit_amend_no_edit()`
  - `async rebase_onto(new_base, old_base, branch)` / `async checkout(rev)`

  ```rust
  let git = GitWrapper::new();
  let branch = git.current_branch().await?;
  git.push_refspec(&format!("{}:refs/heads/{}", branch, branch)).await?;
  ```

- **`LocalExecutor`** ([gitdot-cli/src/executor/local.rs](gitdot-cli/src/executor/local.rs)) — implements `Executor`
  - `async initialize(config, task)` — clones repo into `/tmp/gitdot/tasks/{id}`, connects to S2
  - `async execute()` — runs `sh -c <command>`, streams stdout/stderr as S2 `AppendRecord`s with `stream` headers; appends `task-finished` record on completion
  - `async cleanup()` — removes working directory

  ```rust
  let executor = LocalExecutor::initialize(&runner_config, &task).await?;
  executor.execute().await?;
  executor.cleanup().await?;
  ```

- **`Service` trait** ([gitdot-cli/src/os/service.rs](gitdot-cli/src/os/service.rs))
  - `install()` / `uninstall()` / `start()` / `stop()`
  - `ServiceManager::new()` — detects current executable path; dispatches to `launchd` (macOS) or `systemd` (Linux) via `#[cfg(target_os)]`

- **`GitCredentialStore`** ([gitdot-cli/src/store.rs](gitdot-cli/src/store.rs))
  - `store(url, username, password)` — pipes credentials into `git credential approve`, integrating with the user's configured credential helper (osxkeychain, manager-core, etc.)
