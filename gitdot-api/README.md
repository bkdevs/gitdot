## gitdot-api

### Overview

`gitdot-api` defines the shared API contract between the Gitdot backend and its clients. It provides typed resource structs (response shapes) and endpoint definitions (request/response pairs with HTTP method and path metadata), forming a single source of truth that both the Axum server and TypeScript frontend mirror.

The crate is split into two layers: `resource/` contains plain data structs annotated with `#[derive(ApiResource)]`, and `endpoint/` contains zero-sized-type (ZST) structs implementing the `Endpoint` trait that associate a path, method, request type, and response type for each API call. A companion proc-macro crate (`gitdot-api/derive`) provides the `#[derive(ApiResource)]` and `#[derive(ApiRequest)]` derives.

---

### Table of Contents

- [Core Traits](#core-traits)
- [Resource Types](#resource-types)
  - [OAuth](#oauth-resources)
  - [Users](#user-resources)
  - [Organizations](#organization-resources)
  - [Repositories](#repository-resources)
  - [Reviews](#review-resources)
  - [Questions](#question-resources)
  - [Builds](#build-resources)
  - [Tasks](#task-resources)
  - [Runners](#runner-resources)
  - [Settings](#settings-resources)
  - [Migrations](#migration-resources)
- [Endpoints](#endpoints)
  - [Authentication](#authentication)
  - [Users](#users)
  - [Organizations](#organizations)
  - [Repositories](#repositories)
  - [Reviews](#reviews)
  - [Questions](#questions)
  - [Builds](#builds)
  - [Tasks](#tasks)
  - [Runners](#runners)
  - [Migrations](#migrations)

---

### Core Traits

Defined in [`src/resource.rs`](src/resource.rs) and [`src/endpoint.rs`](src/endpoint.rs).

| Trait | Bounds | Description |
|-------|--------|-------------|
| `ApiResource` | `Serialize + PartialEq + DeserializeOwned` | Marker for all response types. Blanket impls for `Vec<T>`, `Option<T>`, `()`. |
| `ApiRequest` | `Serialize + DeserializeOwned + Send` | Marker for all request types. Blanket impl for `()`. |
| `Endpoint` | — | Associates `PATH: &str`, `METHOD: Method`, `Request: ApiRequest`, `Response: ApiResource`. |

Proc macros from [`derive/src/lib.rs`](derive/src/lib.rs):
- `#[derive(ApiResource)]` — implements `ApiResource` for any struct or enum
- `#[derive(ApiRequest)]` — implements `ApiRequest` for any struct or enum

---

### Resource Types

#### OAuth Resources

##### `DeviceCodeResource`

Returned when initiating a device authorization flow.

| Field | Type | Description |
|-------|------|-------------|
| `device_code` | `string` | Opaque device code used to poll for a token |
| `user_code` | `string` | Short code the user enters at the verification URI |
| `verification_uri` | `string` | URL the user visits to authorize the device |
| `expires_in` | `u64` | Seconds until the device code expires |
| `interval` | `u64` | Minimum polling interval in seconds |

##### `TokenResource`

Returned after a device code is authorized.

| Field | Type | Description |
|-------|------|-------------|
| `access_token` | `string` | Bearer token for subsequent API calls |
| `user_name` | `string` | Name of the authenticated user |
| `user_email` | `string` | Email of the authenticated user |

---

#### User Resources

##### `UserResource`

| Field | Type | Description |
|-------|------|-------------|
| `id` | `uuid` | User ID |
| `name` | `string` | Username (login handle) |
| `email` | `string` | Email address |
| `created_at` | `datetime` | Account creation timestamp |

##### `UserSettingsResource`

| Field | Type | Description |
|-------|------|-------------|
| `repos` | `map<string, UserRepoSettingsResource>` | Per-repository settings keyed by `owner/repo` |

##### `UserRepoSettingsResource`

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `commit_filters` | `CommitFilterResource[]` | No | Named commit filter sets for this repo |

---

#### Organization Resources

##### `OrganizationResource`

| Field | Type | Description |
|-------|------|-------------|
| `id` | `uuid` | Organization ID |
| `name` | `string` | Organization name (slug) |
| `created_at` | `datetime` | Creation timestamp |

##### `OrganizationMemberResource`

| Field | Type | Description |
|-------|------|-------------|
| `id` | `uuid` | Membership record ID |
| `user_id` | `uuid` | Member's user ID |
| `organization_id` | `uuid` | Organization ID |
| `role` | `string` | Member role (e.g. `owner`, `member`) |
| `created_at` | `datetime` | When the user joined the organization |

---

#### Repository Resources

##### `RepositoryResource`

| Field | Type | Description |
|-------|------|-------------|
| `id` | `uuid` | Repository ID |
| `name` | `string` | Repository name |
| `owner` | `string` | Owner name (user or organization) |
| `visibility` | `string` | `public` or `private` |
| `created_at` | `datetime` | Creation timestamp |

##### `RepositoryPathsResource`

Top-level listing of a ref's tree entries.

| Field | Type | Description |
|-------|------|-------------|
| `ref_name` | `string` | Ref used for the listing (e.g. `HEAD`, `main`) |
| `commit_sha` | `string` | Resolved commit SHA for the ref |
| `entries` | `RepositoryPathResource[]` | Flat list of all paths in the tree |

##### `RepositoryPathResource`

A single entry in a repository tree.

| Field | Type | Description |
|-------|------|-------------|
| `path` | `string` | Full path from repo root |
| `name` | `string` | Filename or directory name |
| `path_type` | `PathType` | Entry kind |
| `sha` | `string` | Object SHA |

**`PathType`** (enum): `Blob` · `Tree` · `Commit` · `Unknown`

##### `RepositoryFileResource`

Contents of a single file blob.

| Field | Type | Description |
|-------|------|-------------|
| `path` | `string` | File path |
| `sha` | `string` | Blob SHA |
| `content` | `string` | File contents |
| `encoding` | `string` | Content encoding (e.g. `utf-8`, `base64`) |

##### `RepositoryFolderResource`

Contents of a directory.

| Field | Type | Description |
|-------|------|-------------|
| `path` | `string` | Directory path |
| `entries` | `RepositoryPathResource[]` | Direct children of the directory |

##### `RepositoryBlobResource`

Tagged union — either a file or a folder.

| Variant | Payload |
|---------|---------|
| `File` | `RepositoryFileResource` |
| `Folder` | `RepositoryFolderResource` |

##### `RepositoryBlobsResource`

Batch blob response.

| Field | Type | Description |
|-------|------|-------------|
| `ref_name` | `string` | Ref used for the lookup |
| `commit_sha` | `string` | Resolved commit SHA |
| `blobs` | `RepositoryBlobResource[]` | Requested blobs in order |

##### `RepositoryCommitsResource`

| Field | Type | Description |
|-------|------|-------------|
| `commits` | `RepositoryCommitResource[]` | Ordered list of commits |

##### `RepositoryCommitResource`

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `sha` | `string` | Yes | Commit SHA |
| `parent_sha` | `string` | Yes | Parent commit SHA |
| `message` | `string` | Yes | Commit message |
| `date` | `datetime` | Yes | Commit timestamp |
| `author` | `CommitAuthorResource` | Yes | Commit author |
| `review_number` | `i32` | No | Review number this commit belongs to |
| `diff_position` | `i32` | No | Diff position within the review |
| `diffs` | `RepositoryDiffStatResource[]` | Yes | Per-file diff statistics |

##### `CommitAuthorResource`

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | `uuid` | No | User ID if the author has a Gitdot account |
| `name` | `string` | Yes | Author name |
| `email` | `string` | Yes | Author email |

##### `RepositoryDiffStatResource`

Line-count summary for a single file in a commit.

| Field | Type | Description |
|-------|------|-------------|
| `path` | `string` | File path |
| `lines_added` | `u32` | Lines added |
| `lines_removed` | `u32` | Lines removed |

##### `RepositoryCommitDiffResource`

Full diff for a single commit.

| Field | Type | Description |
|-------|------|-------------|
| `sha` | `string` | Commit SHA |
| `parent_sha` | `string` | Parent commit SHA |
| `files` | `RepositoryDiffFileResource[]` | Per-file diffs |

##### `RepositoryDiffFileResource`

Rich diff for one file.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `path` | `string` | Yes | File path |
| `lines_added` | `u32` | Yes | Lines added |
| `lines_removed` | `u32` | Yes | Lines removed |
| `hunks` | `DiffHunkResource[]` | Yes | Diff hunks (each is a list of line pairs) |
| `left_content` | `string` | No | Full original file content |
| `right_content` | `string` | No | Full modified file content |

`DiffHunkResource` = `DiffPairResource[]`

##### `DiffPairResource`

One row in a side-by-side diff view.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `lhs` | `DiffLineResource` | No | Left-hand (old) line |
| `rhs` | `DiffLineResource` | No | Right-hand (new) line |

##### `DiffLineResource`

| Field | Type | Description |
|-------|------|-------------|
| `line_number` | `u32` | 1-based line number |
| `changes` | `DiffChangeResource[]` | Syntax-highlighted change spans |

##### `DiffChangeResource`

An inline character span with syntax highlighting.

| Field | Type | Description |
|-------|------|-------------|
| `start` | `u32` | Start byte offset within the line |
| `end` | `u32` | End byte offset within the line |
| `content` | `string` | Text content of the span |
| `highlight` | `SyntaxHighlight` | Highlight category |

**`SyntaxHighlight`** (enum): `Delimiter` · `Normal` · `String` · `Type` · `Comment` · `Keyword` · `TreeSitterError`

##### `RepositorySettingsResource`

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `commit_filters` | `CommitFilterResource[]` | No | Repository-level commit filters |

##### `RepositoryResourcesResource`

Composite payload returned by `GetRepositoryResources` — bundles multiple resource types in one request.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `last_commit` | `string` | Yes | SHA of the latest commit seen by the server |
| `last_updated` | `datetime` | No | When the repo was last updated |
| `paths` | `RepositoryPathsResource` | No | File tree (included when changed) |
| `commits` | `RepositoryCommitsResource` | No | Recent commits (included when changed) |
| `blobs` | `RepositoryBlobsResource` | No | Requested file contents |
| `questions` | `RepositoryQuestionsResource` | No | Open questions |
| `reviews` | `RepositoryReviewsResource` | No | Open reviews |
| `builds` | `RepositoryBuildsResource` | No | Recent builds |

##### `RepositoryQuestionsResource`

| Field | Type |
|-------|------|
| `questions` | `QuestionResource[]` |

##### `RepositoryReviewsResource`

| Field | Type |
|-------|------|
| `reviews` | `ReviewResource[]` |

##### `RepositoryBuildsResource`

| Field | Type |
|-------|------|
| `builds` | `BuildResource[]` |

---

#### Review Resources

##### `ReviewResource`

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | `uuid` | Yes | Review ID |
| `number` | `i32` | Yes | Human-readable review number within the repo |
| `author_id` | `uuid` | Yes | Author's user ID |
| `repository_id` | `uuid` | Yes | Repository ID |
| `title` | `string` | Yes | Review title |
| `description` | `string` | Yes | Review description |
| `target_branch` | `string` | Yes | Branch this review targets |
| `status` | `string` | Yes | `draft`, `open`, `merged`, `closed` |
| `created_at` | `datetime` | Yes | — |
| `updated_at` | `datetime` | Yes | — |
| `author` | `ReviewAuthorResource` | No | Embedded author info |
| `diffs` | `DiffResource[]` | Yes | Constituent diffs |
| `reviewers` | `ReviewerResource[]` | Yes | Assigned reviewers |
| `comments` | `ReviewCommentResource[]` | Yes | All comments on the review |

##### `ReviewAuthorResource`

| Field | Type | Description |
|-------|------|-------------|
| `id` | `uuid` | User ID |
| `name` | `string` | Username |

##### `DiffResource`

A single diff within a review (corresponds to one commit range being reviewed).

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | `uuid` | Yes | Diff ID |
| `review_id` | `uuid` | Yes | Parent review ID |
| `position` | `i32` | Yes | Ordering position within the review |
| `title` | `string` | Yes | Diff title |
| `description` | `string` | Yes | Diff description |
| `status` | `string` | Yes | `open`, `merged` |
| `created_at` | `datetime` | Yes | — |
| `updated_at` | `datetime` | Yes | — |
| `revisions` | `RevisionResource[]` | Yes | Ordered list of revisions |

##### `RevisionResource`

A snapshot of a diff at a point in time.

| Field | Type | Description |
|-------|------|-------------|
| `id` | `uuid` | Revision ID |
| `diff_id` | `uuid` | Parent diff ID |
| `number` | `i32` | Revision number (1-based) |
| `commit_hash` | `string` | Commit SHA for this revision's tip |
| `parent_hash` | `string` | Parent commit SHA |
| `created_at` | `datetime` | — |
| `verdicts` | `ReviewVerdictResource[]` | Reviewer verdicts on this revision |

##### `ReviewVerdictResource`

| Field | Type | Description |
|-------|------|-------------|
| `id` | `uuid` | Verdict ID |
| `diff_id` | `uuid` | Diff being reviewed |
| `revision_id` | `uuid` | Specific revision being judged |
| `reviewer_id` | `uuid` | Reviewer's user ID |
| `verdict` | `string` | `approved`, `rejected`, `pending` |
| `created_at` | `datetime` | — |

##### `ReviewerResource`

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | `uuid` | Yes | Record ID |
| `review_id` | `uuid` | Yes | Review ID |
| `reviewer_id` | `uuid` | Yes | User ID of the reviewer |
| `created_at` | `datetime` | Yes | When they were added |
| `user` | `ReviewAuthorResource` | No | Embedded reviewer info |

##### `ReviewCommentResource`

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | `uuid` | Yes | Comment ID |
| `review_id` | `uuid` | Yes | Parent review |
| `diff_id` | `uuid` | Yes | Diff the comment is on |
| `revision_id` | `uuid` | Yes | Revision the comment is anchored to |
| `author_id` | `uuid` | Yes | Author's user ID |
| `parent_id` | `uuid` | No | Parent comment ID (for threads) |
| `body` | `string` | Yes | Comment text (Markdown) |
| `file_path` | `string` | No | File path for inline comments |
| `line_number_start` | `i32` | No | Start line for inline range |
| `line_number_end` | `i32` | No | End line for inline range |
| `side` | `string` | No | `left` or `right` for side-by-side diffs |
| `resolved` | `bool` | Yes | Whether the comment thread is resolved |
| `created_at` | `datetime` | Yes | — |
| `updated_at` | `datetime` | Yes | — |
| `author` | `ReviewAuthorResource` | No | Embedded author info |

---

#### Question Resources

##### `QuestionResource`

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | `uuid` | Yes | Question ID |
| `number` | `i32` | Yes | Human-readable number within the repo |
| `author_id` | `uuid` | Yes | Author's user ID |
| `repository_id` | `uuid` | Yes | Repository ID |
| `title` | `string` | Yes | Question title |
| `body` | `string` | Yes | Question body (Markdown) |
| `upvote` | `i32` | Yes | Net upvote count |
| `impression` | `i32` | Yes | View count |
| `created_at` | `datetime` | Yes | — |
| `updated_at` | `datetime` | Yes | — |
| `user_vote` | `i16` | No | Requesting user's vote: `1`, `-1`, or `0` |
| `author` | `AuthorResource` | No | Embedded author info |
| `comments` | `CommentResource[]` | Yes | Top-level comments |
| `answers` | `AnswerResource[]` | Yes | Answers |

##### `AnswerResource`

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | `uuid` | Yes | Answer ID |
| `question_id` | `uuid` | Yes | Parent question ID |
| `author_id` | `uuid` | Yes | Author's user ID |
| `body` | `string` | Yes | Answer body (Markdown) |
| `upvote` | `i32` | Yes | Net upvote count |
| `created_at` | `datetime` | Yes | — |
| `updated_at` | `datetime` | Yes | — |
| `user_vote` | `i16` | No | Requesting user's vote |
| `author` | `AuthorResource` | No | Embedded author info |
| `comments` | `CommentResource[]` | Yes | Comments on this answer |

##### `CommentResource`

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | `uuid` | Yes | Comment ID |
| `parent_id` | `uuid` | Yes | ID of parent (question or answer) |
| `author_id` | `uuid` | Yes | Author's user ID |
| `body` | `string` | Yes | Comment body (Markdown) |
| `upvote` | `i32` | Yes | Net upvote count |
| `created_at` | `datetime` | Yes | — |
| `updated_at` | `datetime` | Yes | — |
| `user_vote` | `i16` | No | Requesting user's vote |
| `author` | `AuthorResource` | No | Embedded author info |

##### `AuthorResource`

| Field | Type | Description |
|-------|------|-------------|
| `id` | `uuid` | User ID |
| `name` | `string` | Username |

##### `VoteResource`

Returned after casting a vote.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `target_id` | `uuid` | Yes | ID of the voted-on item |
| `score` | `i32` | Yes | Updated net score |
| `user_vote` | `i16` | No | Requesting user's vote after update |

---

#### Build Resources

##### `BuildResource`

| Field | Type | Description |
|-------|------|-------------|
| `id` | `uuid` | Build ID |
| `number` | `i32` | Human-readable build number within the repo |
| `repository_id` | `uuid` | Repository ID |
| `ref_name` | `string` | Branch or tag that triggered the build |
| `trigger` | `string` | What triggered the build (e.g. `push`, `manual`) |
| `commit_sha` | `string` | Commit SHA the build ran against |
| `status` | `string` | `pending`, `running`, `success`, `failed`, `cancelled` |
| `total_tasks` | `i32` | Total number of tasks in this build |
| `completed_tasks` | `i32` | Number of completed tasks |
| `created_at` | `datetime` | — |
| `updated_at` | `datetime` | — |

---

#### Task Resources

##### `TaskResource`

| Field | Type | Description |
|-------|------|-------------|
| `id` | `uuid` | Task ID |
| `repository_id` | `uuid` | Repository ID |
| `build_id` | `uuid` | Build this task belongs to |
| `s2_uri` | `string` | S2 stream URI for log streaming |
| `name` | `string` | Task name |
| `command` | `string` | Shell command to execute |
| `status` | `string` | `pending`, `running`, `success`, `failed` |
| `waits_for` | `uuid[]` | Task IDs this task depends on |
| `created_at` | `datetime` | — |
| `updated_at` | `datetime` | — |

##### `PollTaskResource`

Returned by a runner when polling for work.

| Field | Type | Description |
|-------|------|-------------|
| `id` | `uuid` | Task ID |
| `token` | `string` | Short-lived token for reporting status |
| `owner_name` | `string` | Repository owner |
| `repository_name` | `string` | Repository name |
| `s2_uri` | `string` | S2 stream URI |
| `name` | `string` | Task name |
| `command` | `string` | Command to execute |
| `status` | `string` | Current status |

##### `TaskTokenResource`

| Field | Type | Description |
|-------|------|-------------|
| `token` | `string` | Short-lived token for authenticating task status updates |

---

#### Runner Resources

##### `RunnerResource`

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | `uuid` | Yes | Runner ID |
| `name` | `string` | Yes | Runner name |
| `owner_id` | `uuid` | Yes | Owner's user or organization ID |
| `owner_name` | `string` | Yes | Owner name |
| `owner_type` | `string` | Yes | `user` or `organization` |
| `last_active` | `datetime` | No | Last time the runner polled for work |
| `created_at` | `datetime` | Yes | — |

##### `RunnerTokenResource`

| Field | Type | Description |
|-------|------|-------------|
| `token` | `string` | Authentication token for the runner |

---

#### Settings Resources

##### `CommitFilterResource`

A named rule that selects a subset of commits (used in both repo and user settings).

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | `string` | Yes | Filter name |
| `authors` | `string[]` | No | Limit to commits by these authors |
| `tags` | `string[]` | No | Limit to commits with these tags |
| `included_paths` | `string[]` | No | Only show commits touching these paths |
| `excluded_paths` | `string[]` | No | Exclude commits that only touch these paths |
| `created_at` | `datetime` | Yes | — |
| `updated_at` | `datetime` | Yes | — |

---

#### Migration Resources

##### `GitHubInstallationResource`

| Field | Type | Description |
|-------|------|-------------|
| `id` | `uuid` | Internal installation record ID |
| `installation_id` | `i64` | GitHub App installation ID |
| `owner_id` | `uuid` | Gitdot user who connected the installation |
| `installation_type` | `string` | `User` or `Organization` |
| `github_login` | `string` | GitHub account login that installed the app |
| `created_at` | `datetime` | — |

##### `GitHubRepositoryResource`

A repository visible to a GitHub App installation.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | `u64` | Yes | GitHub repository ID |
| `name` | `string` | Yes | Repository name |
| `full_name` | `string` | Yes | `owner/repo` |
| `description` | `string` | No | Repository description |
| `private` | `bool` | Yes | Whether the repo is private on GitHub |
| `default_branch` | `string` | Yes | Default branch name |

##### `MigrationResource`

| Field | Type | Description |
|-------|------|-------------|
| `id` | `uuid` | Migration ID |
| `number` | `i32` | Human-readable migration number |
| `author_id` | `uuid` | User who initiated the migration |
| `origin_service` | `string` | Source service (e.g. `github`) |
| `origin` | `string` | Source namespace or owner |
| `origin_type` | `string` | `user` or `organization` |
| `destination` | `string` | Destination namespace on Gitdot |
| `destination_type` | `string` | `user` or `organization` |
| `status` | `string` | `pending`, `running`, `complete`, `failed` |
| `created_at` | `datetime` | — |
| `updated_at` | `datetime` | — |
| `repositories` | `MigrationRepositoryResource[]` | Per-repository migration status |

##### `MigrationRepositoryResource`

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | `uuid` | Yes | Record ID |
| `origin_full_name` | `string` | Yes | Source `owner/repo` |
| `destination_full_name` | `string` | Yes | Destination `owner/repo` |
| `visibility` | `string` | Yes | `public` or `private` |
| `status` | `string` | Yes | `pending`, `running`, `complete`, `failed` |
| `error` | `string` | No | Error message if migration failed |
| `created_at` | `datetime` | Yes | — |
| `updated_at` | `datetime` | Yes | — |

---

### Endpoints

All endpoints are relative to the API base URL. Path parameters are written as `{param}`. Query parameters and JSON body fields are listed in the **Request** table. The **Required** column shows `Yes` for mandatory fields; path parameters are always required.

---

#### Authentication

##### `POST /oauth/device`

Initiate a device authorization flow. Returns a device code and user code pair.

**Request body**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_id` | `string` | Yes | OAuth client ID |

**Response:** `DeviceCodeResource`

---

##### `POST /oauth/authorize`

Authorize a device code. The user calls this after entering their user code.

**Request body**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_code` | `string` | Yes | Code displayed to the user |

**Response:** `(empty)`

---

##### `POST /oauth/token`

Poll for an access token after device authorization.

**Request body**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `device_code` | `string` | Yes | Device code from `/oauth/device` |
| `client_id` | `string` | Yes | OAuth client ID |

**Response:** `TokenResource`

---

#### Users

##### `GET /user`

Get the currently authenticated user.

**Request:** none

**Response:** `UserResource`

---

##### `PATCH /user`

Update the currently authenticated user's profile.

**Request body**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | `string` | Yes | New username |

**Response:** `UserResource`

---

##### `GET /user/settings`

Get the current user's settings.

**Request:** none

**Response:** `UserSettingsResource`

---

##### `PATCH /user/settings`

Update the current user's settings.

**Request body**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `repos` | `map<string, UpdateUserRepoSettingsRequest>` | No | Per-repo settings to update |

`UpdateUserRepoSettingsRequest`:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `commit_filters` | `CommitFilterResource[]` | No | Replace the commit filters for this repo |

**Response:** `UserSettingsResource`

---

##### `GET /user/{user_name}`

Get a user by username.

**Path parameters**

| Param | Description |
|-------|-------------|
| `user_name` | Username to look up |

**Response:** `UserResource`

---

##### `HEAD /user/{user_name}`

Check whether a username is taken. Returns `200` if the user exists, `404` otherwise.

**Path parameters**

| Param | Description |
|-------|-------------|
| `user_name` | Username to check |

**Response:** `(empty)`

---

##### `GET /user/{user_name}/repositories`

List repositories owned by a user.

**Path parameters**

| Param | Description |
|-------|-------------|
| `user_name` | Username |

**Response:** `RepositoryResource[]`

---

##### `GET /user/{user_name}/organizations`

List organizations a user belongs to.

**Path parameters**

| Param | Description |
|-------|-------------|
| `user_name` | Username |

**Response:** `OrganizationResource[]`

---

##### `GET /user/{user_name}/reviews`

List reviews authored by or assigned to a user.

**Path parameters**

| Param | Description |
|-------|-------------|
| `user_name` | Username |

**Query parameters**

| Param | Type | Required | Description |
|-------|------|----------|-------------|
| `status` | `string` | No | Filter by review status |
| `owner` | `string` | No | Filter by repository owner |
| `repo` | `string` | No | Filter by repository name |

**Response:** `ReviewResource[]`

---

#### Organizations

##### `POST /organization/{org_name}`

Create a new organization.

**Path parameters**

| Param | Description |
|-------|-------------|
| `org_name` | Name for the new organization |

**Request body:** none

**Response:** `OrganizationResource`

---

##### `GET /organizations`

List all organizations.

**Request:** none

**Response:** `OrganizationResource[]`

---

##### `POST /organization/{org_name}/members`

Add a member to an organization.

**Path parameters**

| Param | Description |
|-------|-------------|
| `org_name` | Organization name |

**Request body**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_name` | `string` | Yes | Username to add |
| `role` | `string` | Yes | Member role (e.g. `member`, `owner`) |

**Response:** `OrganizationMemberResource`

---

##### `GET /organization/{org_name}/members`

List members of an organization.

**Path parameters**

| Param | Description |
|-------|-------------|
| `org_name` | Organization name |

**Query parameters**

| Param | Type | Required | Description |
|-------|------|----------|-------------|
| `role` | `string` | No | Filter by role |

**Response:** `OrganizationMemberResource[]`

---

##### `GET /organization/{org_name}/repositories`

List repositories owned by an organization.

**Path parameters**

| Param | Description |
|-------|-------------|
| `org_name` | Organization name |

**Response:** `RepositoryResource[]`

---

#### Repositories

##### `POST /repository/{owner}/{repo}`

Create a new repository.

**Path parameters**

| Param | Description |
|-------|-------------|
| `owner` | Owner name (user or organization) |
| `repo` | Repository name |

**Request body**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `owner_type` | `string` | Yes | `user` or `organization` |
| `visibility` | `string` | No | `public` (default) or `private` |

**Response:** `RepositoryResource`

---

##### `DELETE /repository/{owner}/{repo}`

Delete a repository.

**Path parameters**

| Param | Description |
|-------|-------------|
| `owner` | Owner name |
| `repo` | Repository name |

**Response:** `(empty)`

---

##### `POST /repository/{owner}/{repo}/resources`

Fetch multiple repository resources in a single request. Pass `last_commit` and `last_updated` to get only what changed since the client's last poll.

**Path parameters**

| Param | Description |
|-------|-------------|
| `owner` | Owner name |
| `repo` | Repository name |

**Request body**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `last_commit` | `string` | No | Last commit SHA the client has; omit for a full response |
| `last_updated` | `datetime` | No | Client's last-updated timestamp for delta detection |

**Response:** `RepositoryResourcesResource`

---

##### `GET /repository/{owner}/{repo}/paths`

List all file paths in the repository tree.

**Path parameters**

| Param | Description |
|-------|-------------|
| `owner` | Owner name |
| `repo` | Repository name |

**Query parameters**

| Param | Type | Required | Description |
|-------|------|----------|-------------|
| `ref_name` | `string` | No | Ref to list (default: `HEAD`) |

**Response:** `RepositoryPathsResource`

---

##### `GET /repository/{owner}/{repo}/blob`

Fetch a single file or directory.

**Path parameters**

| Param | Description |
|-------|-------------|
| `owner` | Owner name |
| `repo` | Repository name |

**Query parameters**

| Param | Type | Required | Description |
|-------|------|----------|-------------|
| `ref_name` | `string` | No | Ref to read from (default: `HEAD`) |
| `path` | `string` | Yes | Path to the file or directory |

**Response:** `RepositoryBlobResource`

---

##### `POST /repository/{owner}/{repo}/blobs`

Fetch multiple files or directories in one request.

**Path parameters**

| Param | Description |
|-------|-------------|
| `owner` | Owner name |
| `repo` | Repository name |

**Request body**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ref_name` | `string` | No | Ref to read from (default: `HEAD`) |
| `paths` | `string[]` | Yes | Paths to fetch |

**Response:** `RepositoryBlobsResource`

---

##### `GET /repository/{owner}/{repo}/commits`

List commits on a ref, optionally filtered by date range.

**Path parameters**

| Param | Description |
|-------|-------------|
| `owner` | Owner name |
| `repo` | Repository name |

**Query parameters**

| Param | Type | Required | Description |
|-------|------|----------|-------------|
| `ref_name` | `string` | No | Ref to walk (default: `HEAD`) |
| `from` | `datetime` | No | Include commits after this timestamp |
| `to` | `datetime` | No | Include commits before this timestamp |

**Response:** `RepositoryCommitsResource`

---

##### `GET /repository/{owner}/{repo}/commits/{sha}`

Get a single commit.

**Path parameters**

| Param | Description |
|-------|-------------|
| `owner` | Owner name |
| `repo` | Repository name |
| `sha` | Commit SHA |

**Response:** `RepositoryCommitResource`

---

##### `GET /repository/{owner}/{repo}/commits/{sha}/diff`

Get the full diff for a commit.

**Path parameters**

| Param | Description |
|-------|-------------|
| `owner` | Owner name |
| `repo` | Repository name |
| `sha` | Commit SHA |

**Response:** `RepositoryCommitDiffResource`

---

##### `GET /repository/{owner}/{repo}/file/commits`

List commits that touched a specific file, with pagination.

**Path parameters**

| Param | Description |
|-------|-------------|
| `owner` | Owner name |
| `repo` | Repository name |

**Query parameters**

| Param | Type | Required | Description |
|-------|------|----------|-------------|
| `path` | `string` | Yes | File path |
| `ref_name` | `string` | No | Ref to walk (default: `HEAD`) |
| `page` | `u32` | No | Page number (default: `1`) |
| `per_page` | `u32` | No | Results per page (default: `30`) |

**Response:** `RepositoryCommitsResource`

---

##### `GET /repository/{owner}/{repo}/settings`

Get repository settings.

**Path parameters**

| Param | Description |
|-------|-------------|
| `owner` | Owner name |
| `repo` | Repository name |

**Response:** `RepositorySettingsResource`

---

##### `PATCH /repository/{owner}/{repo}/settings`

Update repository settings.

**Path parameters**

| Param | Description |
|-------|-------------|
| `owner` | Owner name |
| `repo` | Repository name |

**Request body**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `commit_filters` | `CommitFilterResource[]` | No | Replace the repository's commit filters |

**Response:** `RepositorySettingsResource`

---

#### Reviews

##### `GET /repository/{owner}/{repo}/reviews`

List all reviews for a repository.

**Path parameters**

| Param | Description |
|-------|-------------|
| `owner` | Owner name |
| `repo` | Repository name |

**Response:** `ReviewResource[]`

---

##### `GET /repository/{owner}/{repo}/review/{number}`

Get a single review.

**Path parameters**

| Param | Description |
|-------|-------------|
| `owner` | Owner name |
| `repo` | Repository name |
| `number` | Review number |

**Response:** `ReviewResource`

---

##### `PATCH /repository/{owner}/{repo}/review/{number}`

Update review metadata.

**Path parameters**

| Param | Description |
|-------|-------------|
| `owner` | Owner name |
| `repo` | Repository name |
| `number` | Review number |

**Request body**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `title` | `string` | No | New title |
| `description` | `string` | No | New description |

**Response:** `ReviewResource`

---

##### `POST /repository/{owner}/{repo}/review/{number}/publish`

Publish (open) a review, optionally setting metadata for the review and its diffs in one call.

**Path parameters**

| Param | Description |
|-------|-------------|
| `owner` | Owner name |
| `repo` | Repository name |
| `number` | Review number |

**Request body**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `title` | `string` | No | Review title |
| `description` | `string` | No | Review description |
| `diffs` | `DiffUpdate[]` | No | Metadata updates for individual diffs |

`DiffUpdate`:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `position` | `i32` | Yes | Diff position to update |
| `title` | `string` | No | New diff title |
| `description` | `string` | No | New diff description |

**Response:** `ReviewResource`

---

##### `GET /repository/{owner}/{repo}/review/{number}/diff/{position}`

Get the file diffs for a specific diff within a review, optionally between two revisions.

**Path parameters**

| Param | Description |
|-------|-------------|
| `owner` | Owner name |
| `repo` | Repository name |
| `number` | Review number |
| `position` | Diff position |

**Query parameters**

| Param | Type | Required | Description |
|-------|------|----------|-------------|
| `revision` | `i32` | No | Revision number to view (latest if omitted) |
| `compare_to` | `i32` | No | Compare against this earlier revision number |

**Response**

| Field | Type | Description |
|-------|------|-------------|
| `files` | `RepositoryDiffFileResource[]` | Per-file diffs |

---

##### `PATCH /repository/{owner}/{repo}/review/{number}/diff/{position}`

Update a diff's metadata.

**Path parameters**

| Param | Description |
|-------|-------------|
| `owner` | Owner name |
| `repo` | Repository name |
| `number` | Review number |
| `position` | Diff position |

**Request body**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `title` | `string` | No | New title |
| `description` | `string` | No | New description |

**Response:** `ReviewResource`

---

##### `POST /repository/{owner}/{repo}/review/{number}/diff/{position}/merge`

Merge a diff into the target branch.

**Path parameters**

| Param | Description |
|-------|-------------|
| `owner` | Owner name |
| `repo` | Repository name |
| `number` | Review number |
| `position` | Diff position |

**Request body:** none

**Response:** `ReviewResource`

---

##### `POST /repository/{owner}/{repo}/review/{number}/diff/{position}/submit`

Submit a verdict and comments on a diff revision.

**Path parameters**

| Param | Description |
|-------|-------------|
| `owner` | Owner name |
| `repo` | Repository name |
| `number` | Review number |
| `position` | Diff position |

**Request body**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `action` | `string` | Yes | `approve`, `reject`, or `comment` |
| `comments` | `SubmitReviewComment[]` | Yes | Comments to post |

`SubmitReviewComment`:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `body` | `string` | Yes | Comment text (Markdown) |
| `parent_id` | `uuid` | No | Parent comment ID (for replies) |
| `file_path` | `string` | No | File path for inline comments |
| `line_number_start` | `i32` | No | Start line |
| `line_number_end` | `i32` | No | End line |
| `side` | `string` | No | `left` or `right` |

**Response:** `ReviewResource`

---

##### `POST /repository/{owner}/{repo}/review/{number}/reviewer`

Add a reviewer to a review.

**Path parameters**

| Param | Description |
|-------|-------------|
| `owner` | Owner name |
| `repo` | Repository name |
| `number` | Review number |

**Request body**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_name` | `string` | Yes | Username to add as reviewer |

**Response:** `ReviewerResource`

---

##### `DELETE /repository/{owner}/{repo}/review/{number}/reviewer/{reviewer_name}`

Remove a reviewer from a review.

**Path parameters**

| Param | Description |
|-------|-------------|
| `owner` | Owner name |
| `repo` | Repository name |
| `number` | Review number |
| `reviewer_name` | Username of reviewer to remove |

**Response:** `(empty)`

---

##### `PATCH /repository/{owner}/{repo}/review/{number}/comment/{comment_id}`

Edit a review comment.

**Path parameters**

| Param | Description |
|-------|-------------|
| `owner` | Owner name |
| `repo` | Repository name |
| `number` | Review number |
| `comment_id` | Comment ID |

**Request body**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `body` | `string` | Yes | Updated comment text |

**Response:** `ReviewCommentResource`

---

##### `POST /repository/{owner}/{repo}/review/{number}/comment/{comment_id}/resolve`

Toggle the resolved state of a review comment thread.

**Path parameters**

| Param | Description |
|-------|-------------|
| `owner` | Owner name |
| `repo` | Repository name |
| `number` | Review number |
| `comment_id` | Comment ID |

**Request body**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resolved` | `bool` | Yes | `true` to resolve, `false` to reopen |

**Response:** `ReviewCommentResource`

---

#### Questions

##### `GET /repository/{owner}/{repo}/questions`

List questions in a repository.

**Path parameters**

| Param | Description |
|-------|-------------|
| `owner` | Owner name |
| `repo` | Repository name |

**Response:** `QuestionResource[]`

---

##### `POST /repository/{owner}/{repo}/question`

Create a question.

**Path parameters**

| Param | Description |
|-------|-------------|
| `owner` | Owner name |
| `repo` | Repository name |

**Request body**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `title` | `string` | Yes | Question title |
| `body` | `string` | Yes | Question body (Markdown) |

**Response:** `QuestionResource`

---

##### `GET /repository/{owner}/{repo}/question/{number}`

Get a question with its answers and comments.

**Path parameters**

| Param | Description |
|-------|-------------|
| `owner` | Owner name |
| `repo` | Repository name |
| `number` | Question number |

**Response:** `QuestionResource`

---

##### `PATCH /repository/{owner}/{repo}/question/{number}`

Update a question.

**Path parameters**

| Param | Description |
|-------|-------------|
| `owner` | Owner name |
| `repo` | Repository name |
| `number` | Question number |

**Request body**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `title` | `string` | Yes | New title |
| `body` | `string` | Yes | New body |

**Response:** `QuestionResource`

---

##### `POST /repository/{owner}/{repo}/question/{number}/vote`

Vote on a question.

**Path parameters**

| Param | Description |
|-------|-------------|
| `owner` | Owner name |
| `repo` | Repository name |
| `number` | Question number |

**Request body**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `value` | `i16` | Yes | `1` (upvote), `-1` (downvote), or `0` (remove vote) |

**Response:** `VoteResource`

---

##### `POST /repository/{owner}/{repo}/question/{number}/comment`

Add a top-level comment to a question.

**Path parameters**

| Param | Description |
|-------|-------------|
| `owner` | Owner name |
| `repo` | Repository name |
| `number` | Question number |

**Request body**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `body` | `string` | Yes | Comment text (Markdown) |

**Response:** `CommentResource`

---

##### `POST /repository/{owner}/{repo}/question/{number}/answer`

Post an answer to a question.

**Path parameters**

| Param | Description |
|-------|-------------|
| `owner` | Owner name |
| `repo` | Repository name |
| `number` | Question number |

**Request body**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `body` | `string` | Yes | Answer text (Markdown) |

**Response:** `AnswerResource`

---

##### `PATCH /repository/{owner}/{repo}/question/{number}/answer/{answer_id}`

Edit an answer.

**Path parameters**

| Param | Description |
|-------|-------------|
| `owner` | Owner name |
| `repo` | Repository name |
| `number` | Question number |
| `answer_id` | Answer ID |

**Request body**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `body` | `string` | Yes | Updated answer text |

**Response:** `AnswerResource`

---

##### `POST /repository/{owner}/{repo}/question/{number}/answer/{answer_id}/vote`

Vote on an answer.

**Path parameters**

| Param | Description |
|-------|-------------|
| `owner` | Owner name |
| `repo` | Repository name |
| `number` | Question number |
| `answer_id` | Answer ID |

**Request body**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `value` | `i16` | Yes | `1`, `-1`, or `0` |

**Response:** `VoteResource`

---

##### `POST /repository/{owner}/{repo}/question/{number}/answer/{answer_id}/comment`

Add a comment to an answer.

**Path parameters**

| Param | Description |
|-------|-------------|
| `owner` | Owner name |
| `repo` | Repository name |
| `number` | Question number |
| `answer_id` | Answer ID |

**Request body**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `body` | `string` | Yes | Comment text (Markdown) |

**Response:** `CommentResource`

---

##### `PATCH /repository/{owner}/{repo}/question/{number}/comment/{comment_id}`

Edit a comment on a question or answer.

**Path parameters**

| Param | Description |
|-------|-------------|
| `owner` | Owner name |
| `repo` | Repository name |
| `number` | Question number |
| `comment_id` | Comment ID |

**Request body**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `body` | `string` | Yes | Updated comment text |

**Response:** `CommentResource`

---

##### `POST /repository/{owner}/{repo}/question/{number}/comment/{comment_id}/vote`

Vote on a comment.

**Path parameters**

| Param | Description |
|-------|-------------|
| `owner` | Owner name |
| `repo` | Repository name |
| `number` | Question number |
| `comment_id` | Comment ID |

**Request body**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `value` | `i16` | Yes | `1`, `-1`, or `0` |

**Response:** `VoteResource`

---

#### Builds

##### `GET /repository/{owner}/{repo}/builds`

List builds for a repository.

**Path parameters**

| Param | Description |
|-------|-------------|
| `owner` | Owner name |
| `repo` | Repository name |

**Query parameters**

| Param | Type | Required | Description |
|-------|------|----------|-------------|
| `from` | `datetime` | No | Include builds after this timestamp |
| `to` | `datetime` | No | Include builds before this timestamp |

**Response:** `BuildResource[]`

---

##### `POST /repository/{owner}/{repo}/build`

Trigger a new build.

**Path parameters**

| Param | Description |
|-------|-------------|
| `owner` | Owner name |
| `repo` | Repository name |

**Request body**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ref_name` | `string` | Yes | Branch or tag to build |
| `commit_sha` | `string` | Yes | Commit SHA to build |

**Response:** `BuildResource`

---

##### `GET /repository/{owner}/{repo}/build/{number}`

Get a single build.

**Path parameters**

| Param | Description |
|-------|-------------|
| `owner` | Owner name |
| `repo` | Repository name |
| `number` | Build number |

**Response:** `BuildResource`

---

##### `GET /repository/{owner}/{repo}/build/{number}/tasks`

List tasks belonging to a build.

**Path parameters**

| Param | Description |
|-------|-------------|
| `owner` | Owner name |
| `repo` | Repository name |
| `number` | Build number |

**Response:** `TaskResource[]`

---

#### Tasks

These endpoints are used by CI runners, not end users.

##### `GET /ci/task/poll`

Poll for the next available task. Called by idle runners.

**Request:** none

**Response:** `PollTaskResource` (nullable — `null` when no work is available)

---

##### `POST /ci/task/{id}/token`

Issue a short-lived token for a task. The runner calls this before reporting status.

**Path parameters**

| Param | Description |
|-------|-------------|
| `id` | Task ID |

**Request body:** none

**Response:** `TaskTokenResource`

---

##### `PATCH /ci/task/{id}`

Report a task status update.

**Path parameters**

| Param | Description |
|-------|-------------|
| `id` | Task ID |

**Request body**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `status` | `string` | Yes | New status: `running`, `success`, `failed` |

**Response:** `TaskResource`

---

#### Runners

##### `GET /ci/runner/{owner}`

List runners for an owner.

**Path parameters**

| Param | Description |
|-------|-------------|
| `owner` | Owner name (user or organization) |

**Response:** `RunnerResource[]`

---

##### `POST /ci/runner/{owner}`

Register a new runner.

**Path parameters**

| Param | Description |
|-------|-------------|
| `owner` | Owner name |

**Request body**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | `string` | Yes | Runner name |
| `owner_type` | `string` | Yes | `user` or `organization` |

**Response:** `RunnerResource`

---

##### `GET /ci/runner/{owner}/{name}`

Get a runner by name.

**Path parameters**

| Param | Description |
|-------|-------------|
| `owner` | Owner name |
| `name` | Runner name |

**Response:** `RunnerResource`

---

##### `DELETE /ci/runner/{owner}/{name}`

Delete a runner.

**Path parameters**

| Param | Description |
|-------|-------------|
| `owner` | Owner name |
| `name` | Runner name |

**Response:** `(empty)`

---

##### `POST /ci/runner/{owner}/{name}/token`

Refresh the authentication token for a runner.

**Path parameters**

| Param | Description |
|-------|-------------|
| `owner` | Owner name |
| `name` | Runner name |

**Request body:** none

**Response:** `RunnerTokenResource`

---

##### `POST /ci/runner/{id}/verify`

Verify a runner's identity. Called by the runner on startup.

**Path parameters**

| Param | Description |
|-------|-------------|
| `id` | Runner ID |

**Request body:** none

**Response:** `(empty)`

---

#### Migrations

##### `GET /migrations`

List all migrations initiated by the current user.

**Response:** `MigrationResource[]`

---

##### `GET /migration/{number}`

Get a single migration.

**Path parameters**

| Param | Description |
|-------|-------------|
| `number` | Migration number |

**Response:** `MigrationResource`

---

##### `GET /migration/github/installations`

List GitHub App installations connected by the current user.

**Response:** `GitHubInstallationResource[]`

---

##### `POST /migration/github/{installation_id}`

Register a GitHub App installation.

**Path parameters**

| Param | Description |
|-------|-------------|
| `installation_id` | GitHub App installation ID |

**Request body:** none

**Response:** `GitHubInstallationResource`

---

##### `GET /migration/github/{installation_id}/repositories`

List repositories accessible via a GitHub App installation.

**Path parameters**

| Param | Description |
|-------|-------------|
| `installation_id` | GitHub App installation ID |

**Response:** `GitHubRepositoryResource[]`

---

##### `POST /migration/github/{installation_id}/migrate`

Start migrating repositories from GitHub.

**Path parameters**

| Param | Description |
|-------|-------------|
| `installation_id` | GitHub App installation ID |

**Request body**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `origin` | `string` | Yes | Source namespace (GitHub owner) |
| `origin_type` | `string` | Yes | `user` or `organization` |
| `destination` | `string` | Yes | Destination namespace on Gitdot |
| `destination_type` | `string` | Yes | `user` or `organization` |
| `repositories` | `string[]` | Yes | Repository names to migrate |

**Response:** `MigrationResource`
