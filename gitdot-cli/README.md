## gitdot-cli

`gitdot-cli` is the command-line interface for Gitdot, compiled to a binary named `gdot`. It provides user-facing commands for authentication, code review workflows, and CI pipeline management, as well as daemon-mode runner commands for executing CI tasks on self-hosted machines. The CLI is built with Clap and uses compile-time feature flags (`main`, `runner`) to gate entire subcommand trees.

Configuration is split between two TOML files: user config at `~/.config/gitdot/config.toml` (auth tokens, server URLs) and runner config at `/etc/gitdot/runner.toml` (runner token, executor count, S2 stream URL). All API calls go through `GitdotClient`, a thin reqwest wrapper that attaches JWT or Basic auth headers and deserializes responses into `gitdot-api` resource types.
### APIs

#### `GitdotClient` — `src/client.rs`

Reqwest wrapper with pluggable auth. Builder pattern for construction.

```rust
pub struct GitdotClient { /* ... */ }

impl GitdotClient {
    pub fn new(client_id: &str) -> Self
    pub fn with_web_url(self, web_url: &str) -> Self
    pub fn with_server_url(self, server_url: &str) -> Self
    pub fn with_token(self, token: String) -> Self      // Basic auth (runner)
    pub fn with_jwt(self, token: String) -> Self         // Bearer auth (user)

    #[cfg(feature = "main")]
    pub fn from_user_config(config: &UserConfig) -> Self

    #[cfg(feature = "runner")]
    pub fn from_runner_config(config: &RunnerConfig) -> Self

    pub fn get_client_id(&self) -> &str
    pub fn get_web_url(&self) -> &str
    pub fn get_server_url(&self) -> &str

    pub(crate) async fn get<T: ApiRequest, R: ApiResource>(path, request) -> Result<R>
    pub(crate) async fn head<T: ApiRequest>(path, request) -> Result<()>
    pub(crate) async fn post<T: ApiRequest, R: ApiResource>(path, request) -> Result<R>
    pub(crate) async fn patch<T: ApiRequest, R: ApiResource>(path, request) -> Result<R>
    pub(crate) async fn delete<T: ApiRequest, R: ApiResource>(path, request) -> Result<R>
}
```

```rust
let client = GitdotClient::from_user_config(&config).with_jwt(token);
let user = client.get_current_user(GetCurrentUserRequest {}).await?;
```

#### OAuth methods — `src/client/methods/oauth.rs`

Device flow login. POSTs to `oauth/device` and `oauth/token`.

```rust
impl GitdotClient {
    pub async fn create_device_code(&self) -> Result<CreateDeviceCodeResponse>
    pub async fn poll_token(&self, device_code: &str) -> Result<PollTokenResponse>
}
```

#### User methods — `src/client/methods/user.rs`

```rust
impl GitdotClient {
    pub async fn get_current_user(&self, request: GetCurrentUserRequest) -> Result<GetCurrentUserResponse>
    pub async fn get_user(&self, user_name: &str, request: GetUserRequest) -> Result<GetUserResponse>
    pub async fn has_user(&self, user_name: &str, request: HasUserApiRequest) -> Result<()>
    pub async fn list_user_repositories(&self, user_name: &str, request: ListUserRepositoriesRequest) -> Result<ListUserRepositoriesResponse>
    pub async fn list_user_organizations(&self, user_name: &str, request: ListUserOrganizationsRequest) -> Result<ListUserOrganizationsResponse>
    pub async fn list_user_reviews(&self, user_name: &str, request: ListUserReviewsRequest) -> Result<ListUserReviewsResponse>
    pub async fn update_current_user(&self, request: UpdateCurrentUserRequest) -> Result<UpdateCurrentUserResponse>
}
```

#### Task methods — `src/client/methods/task.rs`

```rust
impl GitdotClient {
    pub async fn poll_task(&self, request: PollTaskRequest) -> Result<PollTaskResponse>
    pub async fn update_task(&self, id: Uuid, status: &str) -> Result<UpdateTaskResponse>
}
```

#### Runner methods — `src/client/methods/runner.rs`

```rust
impl GitdotClient {
    pub async fn verify_runner(&self) -> Result<()>
}
```

---

#### `UserConfig` — `src/config/user.rs`

Async load/save for `~/.config/gitdot/config.toml`.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserConfig {
    pub gitdot_server_url: String,  // default: "https://api.gitdot.io"
    pub gitdot_web_url: String,     // default: "https://www.gitdot.io"
    pub user_name: String,
    pub user_email: String,
}

impl UserConfig {
    pub async fn load() -> anyhow::Result<Self>   // returns Default if file absent
    pub async fn save(&self) -> anyhow::Result<()>
}
```

#### `RunnerConfig` — `src/config/runner.rs`

Sync load/save for `/etc/gitdot/runner.toml`. Writes via `sudo cp` from a temp file.

```rust
pub const SYSTEM_USER: &str = "gitdot";
pub const RUNNER_CONFIG_PATH: &str = "/etc/gitdot/runner.toml";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunnerConfig {
    pub gitdot_server_url: String,  // default: "https://api.gitdot.io"
    pub gitdot_web_url: String,     // default: "https://www.gitdot.io"
    pub s2_server_url: String,      // default: "https://s2.gitdot.io"
    pub runner_token: Option<String>,
    pub num_executors: i8,          // default: 4
}

impl RunnerConfig {
    pub fn load() -> anyhow::Result<Self>
    pub fn save(&self) -> anyhow::Result<()>
}
```

---

#### `GitWrapper` — `src/git.rs`

Async wrapper over `git` CLI via `tokio::process::Command`.

```rust
pub struct GitWrapper;

impl GitWrapper {
    pub fn new() -> Self

    // Primitives — capture stdout
    pub async fn remote_url(&self, name: &str) -> anyhow::Result<String>
    pub async fn default_branch(&self) -> anyhow::Result<String>
    pub async fn git_dir(&self) -> anyhow::Result<PathBuf>
    pub async fn rev_parse(&self, rev: &str) -> anyhow::Result<String>
    pub async fn current_branch(&self) -> anyhow::Result<String>
    pub async fn log_oneline(&self, range: &str) -> anyhow::Result<Vec<(String, String)>>

    // Operations — inherit or suppress stdio
    pub async fn pull_rebase(&self, branch: &str) -> anyhow::Result<()>
    pub async fn push_refspec(&self, refspec: &str) -> anyhow::Result<String>
    pub async fn add_all(&self) -> anyhow::Result<()>
    pub async fn commit_amend_no_edit(&self) -> anyhow::Result<()>
    pub async fn rebase_onto(&self, new_base: &str, old_base: &str, branch: &str) -> anyhow::Result<()>
    pub async fn checkout(&self, rev: &str) -> anyhow::Result<()>
}
```

```rust
let git = GitWrapper::new();
let branch = git.current_branch().await?;
git.push_refspec(&format!("HEAD:refs/for/{}", branch)).await?;
```

---

#### `Executor` trait + `LocalExecutor` — `src/executor.rs`, `src/executor/local.rs`

```rust
pub trait Executor: Sized {
    async fn initialize(config: &RunnerConfig, task: &PollTaskResource) -> Result<Self>;
    async fn execute(&self) -> Result<()>;
    async fn cleanup(self) -> Result<()>;
}

pub struct LocalExecutor {
    pub working_directory: PathBuf,
    pub task: PollTaskResource,
    pub s2: S2,
}
```

`initialize` clones the repo into `/tmp/gitdot/tasks/{id}` via `git2` and opens an S2 producer. `execute` spawns `sh -c <command>`, streams stdout/stderr as `AppendRecord`s with a `stream` header, and appends a `task-finished` record on exit. `cleanup` removes the working directory.

```rust
let executor = LocalExecutor::initialize(&runner_config, &task).await?;
executor.execute().await?;
executor.cleanup().await?;
```

---

#### `Service` trait + `ServiceManager` — `src/os/service.rs`

```rust
pub trait Service {
    fn install(&self) -> Result<()>;
    fn uninstall(&self) -> Result<()>;
    fn start(&self) -> Result<()>;
    fn stop(&self) -> Result<()>;
}

pub struct ServiceManager {
    binary_path: String,
}

impl ServiceManager {
    pub fn new() -> Result<Self>   // detects current exe path
}
```

`ServiceManager` dispatches to `launchd` (macOS) or `systemd` (Linux) via `#[cfg(target_os)]`.

---

#### `GitCredentialStore` — `src/store.rs`

```rust
pub struct GitCredentialStore;

impl GitCredentialStore {
    pub fn store(url: &str, username: &str, password: &str) -> anyhow::Result<()>
}
```

Pipes credentials into `git credential approve`, integrating with whatever credential helper is configured (osxkeychain, manager-core, etc.).

---

#### Utilities — `src/util/`

```rust
// src/util/ci.rs
pub async fn find_config() -> anyhow::Result<PathBuf>
// Returns path to .gitdot-ci.toml at git repo root (does not check existence)

// src/util/s2.rs
pub fn parse_s2_uri(uri: &str) -> Result<(BasinName, StreamName)>
// Parses "s2://<basin>/<stream>", replacing '.' with '-' in basin name

// src/util/review.rs
pub async fn get_remote_owner_repo(git: &GitWrapper) -> anyhow::Result<(String, String)>
pub async fn save_review_branch(git: &GitWrapper, branch: &str) -> anyhow::Result<()>
pub async fn load_review_branch(git: &GitWrapper) -> anyhow::Result<Option<String>>
pub async fn clear_review_branch(git: &GitWrapper) -> anyhow::Result<()>
pub async fn push_for_review(git: &GitWrapper, branch: &str, review_number: Option<i32>) -> anyhow::Result<Option<String>>
// Pushes HEAD to refs/for/<branch>[/<number>], returns review URL from git push stderr

// src/util/command.rs
pub fn run_command(program: &str, args: &[&str]) -> anyhow::Result<()>
```
