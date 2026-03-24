## gitdot-core

### Overview

`gitdot-core` is the business logic crate for the Gitdot platform. It defines all domain services, data-access repositories, external client integrations, DTOs, models, and error types. The Axum HTTP server (`gitdot-server`) depends entirely on this crate and delegates all logic to its services, keeping the handler layer thin.

The crate follows a strict layered architecture: handlers call services, services call repositories and clients, repositories execute SQL queries via `sqlx`, and clients wrap `git2`, `difftastic`, GitHub's API, and S2 streams. Every layer is expressed as a trait with a corresponding `Impl` struct, making each layer independently testable.

### Files

```
gitdot-core/
├── derive/                         # Proc macro crate — #[derive(ApiResource)]
│   ├── src/lib.rs
│   └── Cargo.toml
├── hooks/                          # Git hook scripts installed into bare repos
│   ├── post-receive
│   ├── pre-receive
│   └── proc-receive
├── migrations/                     # sqlx up/down migration SQL files
├── src/
│   ├── client/                     # External service clients (traits + impls)
│   │   ├── diff.rs                 # DiffClient / DifftClient (difftastic)
│   │   ├── git.rs                  # GitClient / Git2Client (libgit2)
│   │   ├── git_http.rs             # GitHttpClient / GitHttpClientImpl (git http-backend)
│   │   ├── github.rs               # GitHubClient / OctocrabClient
│   │   ├── s2.rs                   # S2Client / S2ClientImpl (durable streams)
│   │   └── secret.rs               # SecretClient / GoogleSecretClient
│   ├── dto/                        # Request/response DTOs, one dir per domain
│   │   ├── authentication/
│   │   ├── authorization/
│   │   ├── build/
│   │   ├── commit/
│   │   ├── git_http/
│   │   ├── migration/github/
│   │   ├── oauth/
│   │   ├── organization/
│   │   ├── question/
│   │   ├── repository/
│   │   ├── review/
│   │   ├── runner/
│   │   ├── task/
│   │   ├── user/
│   │   └── common.rs               # Validated types: OwnerName, RepositoryName, RunnerName
│   ├── error/                      # Domain error enums (thiserror), one file per domain
│   ├── model/                      # #[derive(FromRow)] DB model structs
│   ├── repository/                 # SQL data-access traits + impls (sqlx)
│   ├── service/                    # Business logic traits + impls, one file per domain
│   ├── util/                       # Internal helpers
│   │   ├── auth.rs                 # Reserved name checks
│   │   ├── code.rs
│   │   ├── git.rs                  # Hook scripts, ref helpers, DEFAULT_BRANCH constant
│   │   ├── github.rs               # GitHub clone URL helpers
│   │   ├── review.rs               # Magic-ref helpers (refs/for/*, refs/reviews/*)
│   │   └── token.rs                # Token generation and hashing
│   ├── client.rs
│   ├── dto.rs
│   ├── error.rs
│   ├── lib.rs
│   ├── model.rs
│   ├── repository.rs
│   ├── service.rs
│   └── util.rs
├── CLAUDE.md
└── Cargo.toml
```

### APIs

#### Services (`gitdot-core/src/service/`)

All services are `async_trait` traits. Concrete impls are generic over repository/client traits and wired up with `*Impl` types.

- **`UserService`** — [`src/service/user.rs`](gitdot-core/src/service/user.rs)
  - `get_current_user(request) -> UserResponse`
  - `update_current_user(request) -> UserResponse` — validates against reserved names / name conflicts
  - `has_user(request) -> ()`
  - `get_user(request) -> UserResponse`
  - `list_repositories(request) -> Vec<RepositoryResponse>` — filters private repos for non-owners
  - `list_organizations(request) -> Vec<OrganizationResponse>`
  - `list_reviews(request) -> Vec<ReviewResponse>`
  - `get_current_user_settings(request) -> UserSettingsResponse`
  - `update_current_user_settings(request) -> UserSettingsResponse`

- **`RepositoryService`** — [`src/service/repository.rs`](gitdot-core/src/service/repository.rs)
  - `create_repository(request) -> RepositoryResponse` — initializes bare git repo, installs hooks, then inserts DB record
  - `delete_repository(request) -> ()`
  - `get_repository_by_id(id) -> RepositoryResponse`
  - `get_repository_blob(request) -> RepositoryBlobResponse` — file or folder listing at a ref/path
  - `get_repository_blobs(request) -> RepositoryBlobsResponse`
  - `get_repository_paths(request) -> RepositoryPathsResponse` — full recursive file tree
  - `get_repository_file_commits(request) -> RepositoryCommitsResponse` — paginated, enriched with user info
  - `resolve_ref_sha(owner, repo, ref_name) -> String`
  - `get_repository_settings(request) -> RepositorySettingsResponse`
  - `update_repository_settings(request) -> RepositorySettingsResponse`

- **`ReviewService`** — [`src/service/review.rs`](gitdot-core/src/service/review.rs)
  - `get_review(request) -> ReviewResponse`
  - `list_reviews(request) -> ReviewsResponse`
  - `create_review(request) -> ReviewResponse` — triggered by `refs/for/<branch>` push; creates diffs + revisions + git refs
    ```rust
    // Push flow: proc-receive hook → ReviewService::create_review
    // Creates: refs/reviews/<N>/diffs/<pos>/revisions/1
    //          refs/reviews/<N>/diffs/<pos>/current
    //          refs/reviews/<N>/head
    ```
  - `process_review_update(request) -> ReviewResponse` — handles re-push to `refs/for/<branch>/<N>`; detects rebase-only vs. content change via patch IDs
  - `publish_review(request) -> ReviewResponse` — transitions draft → in_progress
  - `update_review(request) -> ReviewResponse`
  - `get_review_diff(request) -> ReviewDiffResponse` — fetches file pairs and runs difftastic
  - `submit_review(request) -> ReviewResponse` — records verdict (approve/request-changes/comment)
  - `merge_diff(request) -> ReviewResponse` — fast-forward or cherry-pick onto target branch
  - `update_diff(request) -> ReviewResponse`
  - `add_reviewer(request) -> ReviewerResponse`
  - `remove_reviewer(request) -> ()`
  - `update_review_comment(request) -> ReviewCommentResponse`
  - `resolve_review_comment(request) -> ReviewCommentResponse`

- **`BuildService`** — [`src/service/build.rs`](gitdot-core/src/service/build.rs)
  - `create_build(request) -> BuildResponse` — reads CI config from repo, creates build + tasks, publishes to S2 stream
  - `list_builds(request) -> BuildsResponse`
  - `get_build(owner, repo, number) -> BuildResponse`
  - `list_build_tasks(owner, repo, number) -> Vec<TaskResponse>`

- **`CommitService`** — [`src/service/commit.rs`](gitdot-core/src/service/commit.rs)
  - `get_commit(request) -> CommitResponse`
  - `get_commit_diff(request) -> CommitDiffResponse` — diff via git + difftastic
  - `get_commits(request) -> CommitsResponse`
  - `create_commits(request) -> Vec<CommitResponse>` — stores commit metadata in DB from push hook

- **`OrganizationService`** — [`src/service/organization.rs`](gitdot-core/src/service/organization.rs)
  - `create_organization(request) -> OrganizationResponse`
  - `get_organization(request) -> OrganizationResponse`
  - `add_member(request) -> OrganizationMemberResponse`
  - `list_repositories(request) -> Vec<RepositoryResponse>`
  - `list_organizations() -> Vec<OrganizationResponse>`
  - `list_members(request) -> Vec<OrganizationMemberResponse>`

- **`QuestionService`** — [`src/service/question.rs`](gitdot-core/src/service/question.rs)
  - `create_question / update_question / get_question / list_questions`
  - `create_answer / update_answer`
  - `create_question_comment / create_answer_comment / update_comment`
  - `vote_question / vote_answer / vote_comment -> VoteResponse`

- **`RunnerService`** — [`src/service/runner.rs`](gitdot-core/src/service/runner.rs)
  - `create_runner(request) -> CreateRunnerResponse`
  - `verify_runner(request) -> ()` — validates hashed token
  - `get_runner(request) -> GetRunnerResponse`
  - `delete_runner(request) -> ()`
  - `refresh_runner_token(request) -> CreateRunnerTokenResponse`
  - `list_runners(request) -> ListRunnersResponse`

- **`MigrationService`** — [`src/service/migration.rs`](gitdot-core/src/service/migration.rs)
  - `get_migration / list_migrations`
  - `create_github_installation / list_github_installations / list_github_installation_repositories`
  - `create_github_migration -> CreateGitHubMigrationResponse`
  - `migrate_github_repositories -> MigrateGitHubRepositoriesResponse` — mirrors repos via `git clone --mirror`, installs hooks

- **`AuthenticationService`** — [`src/service/authentication.rs`](gitdot-core/src/service/authentication.rs)
  - `issue_task_jwt(request) -> String` — mints a short-lived JWT for runner tasks

- **`AuthorizationService`** — [`src/service/authorization.rs`](gitdot-core/src/service/authorization.rs)
  - `validate_token / verify_authorized_for_repository / verify_authorized_for_review / ...` — permission checks per resource type

- **`OAuthService`** — [`src/service/oauth.rs`](gitdot-core/src/service/oauth.rs)
  - `request_device_code / poll_token / authorize_device` — device flow OAuth

- **`GitHttpService`** — [`src/service/git_http.rs`](gitdot-core/src/service/git_http.rs)
  - `info_refs / upload_pack / receive_pack` — proxies smart HTTP git protocol to `git http-backend`

- **`TaskService`** — [`src/service/task.rs`](gitdot-core/src/service/task.rs)
  - `update_task(request) -> TaskResponse` — runner reports task status/output

#### Clients (`gitdot-core/src/client/`)

- **`GitClient` / `Git2Client`** — [`src/client/git.rs`](gitdot-core/src/client/git.rs)
  - Wraps `git2` (libgit2). Blocking calls run inside `tokio::task::spawn_blocking`.
  - `create_repo / delete_repo / mirror_repo`
  - `create_ref / update_ref`
  - `get_repo_blob / get_repo_blobs / get_repo_paths / get_repo_commit / get_repo_file_commits`
  - `get_repo_diff_files / get_repo_diff_stats`
  - `rev_list(old_sha, new_sha) -> Vec<RepositoryCommitResponse>`
  - `resolve_ref_sha / get_commit_patch_id / cherry_pick_commit`
  - `install_hook / empty_hooks`

- **`DiffClient` / `DifftClient`** — [`src/client/diff.rs`](gitdot-core/src/client/diff.rs)
  - `diff_files(left, right) -> RepositoryDiffFileResponse` — shells out to `difft` for syntax-aware diffs

- **`GitHttpClient` / `GitHttpClientImpl`** — [`src/client/git_http.rs`](gitdot-core/src/client/git_http.rs)
  - `info_refs / upload_pack / receive_pack` — CGI bridge to `git http-backend`

- **`GitHubClient` / `OctocrabClient`** — [`src/client/github.rs`](gitdot-core/src/client/github.rs)
  - Wraps `octocrab` for GitHub App API calls (installations, repos)

- **`S2Client` / `S2ClientImpl`** — [`src/client/s2.rs`](gitdot-core/src/client/s2.rs)
  - `create_stream(owner, repo, task_id) -> String` — provisions an S2 durable stream for build task logs

- **`SecretClient` / `GoogleSecretClient`** — [`src/client/secret.rs`](gitdot-core/src/client/secret.rs)
  - Fetches secrets (e.g., GitHub App private key) from Google Secret Manager

#### DTOs — Validated Types (`gitdot-core/src/dto/common.rs`)

Key request types use `nutype`-derived wrappers that auto-sanitize and validate on construction:

```rust
// OwnerName: trim, lowercase, strip ".git", validate slug (2–32 chars, [a-z0-9_-])
let owner = OwnerName::try_new("MyOrg").unwrap(); // → "myorg"

// RepositoryName: same rules
// RunnerName: same rules
```

#### Repositories (`gitdot-core/src/repository/`)

Each domain exposes a `*Repository` trait and a `*RepositoryImpl` backed by a `PgPool`. Methods return `Result<T, sqlx::Error>`. Key ones:

- **`UserRepository`** — `get_by_id / get / get_by_emails / is_name_taken / update / get_settings / update_settings`
- **`RepositoryRepository`** — `create / get / get_by_id / list_by_owner / delete / get_settings / update_settings`
- **`ReviewRepository`** — `create_review / get_review / list_reviews / create_diff / create_revision / update_revision_sha / update_diff / update_review / create_comment / get_comment / update_comment / resolve_comment / create_verdict / add_reviewer / remove_reviewer / get_reviews_by_user`
- **`BuildRepository`** — `create / get / list / get_by_number`
- **`CommitRepository`** — `create / get / list`
- **`TaskRepository`** — `create / update / get_by_build`
- **`TokenRepository`** — `create / get_by_hash / revoke`
- **`OrganizationRepository`** — `create / get / list_by_user_id / add_member / list_members / get_member_role`
- **`QuestionRepository`** — full CRUD + voting for questions, answers, and comments
- **`RunnerRepository`** — `create / get / list / delete`
- **`MigrationRepository`** / **`GitHubRepository`** — migration and GitHub installation records
- **`CodeRepository`** — stores file-level code snapshots for search/indexing
