## gitdot-api

### Overview

`gitdot-api` defines the shared API contract between the Gitdot backend and its clients. It provides typed resource structs (response shapes) and endpoint definitions (request/response pairs with HTTP method and path metadata), forming a single source of truth that both the Axum server and TypeScript frontend mirror.

The crate is split into two layers: `resource/` contains plain data structs annotated with `#[derive(ApiResource)]`, and `endpoint/` contains zero-sized-type (ZST) structs implementing the `Endpoint` trait that associate a path, method, request type, and response type for each API call. A companion proc-macro crate (`gitdot-api/derive`) provides the `#[derive(ApiResource)]` and `#[derive(ApiRequest)]` derives.

### Files

```
gitdot-api
├── Cargo.toml
├── derive
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
└── src
    ├── lib.rs
    ├── resource.rs
    ├── resource
    │   ├── build.rs
    │   ├── migration.rs
    │   ├── oauth.rs
    │   ├── organization.rs
    │   ├── question.rs
    │   ├── repository.rs
    │   ├── review.rs
    │   ├── runner.rs
    │   ├── settings.rs
    │   ├── task.rs
    │   └── user.rs
    ├── endpoint.rs
    └── endpoint
        ├── build.rs
        ├── build
        │   ├── create_build.rs
        │   ├── get_build.rs
        │   ├── list_build_tasks.rs
        │   └── list_builds.rs
        ├── migration.rs
        ├── migration
        │   ├── get_migration.rs
        │   ├── list_migrations.rs
        │   └── github
        │       ├── create_github_installation.rs
        │       ├── list_github_installation_repositories.rs
        │       ├── list_github_installations.rs
        │       └── migrate_github_repositories.rs
        ├── oauth.rs
        ├── oauth
        │   ├── authorize_device.rs
        │   ├── create_device_code.rs
        │   └── poll_token.rs
        ├── organization.rs
        ├── organization
        │   ├── add_member.rs
        │   ├── create_organization.rs
        │   ├── list_organization_members.rs
        │   ├── list_organization_repositories.rs
        │   └── list_organizations.rs
        ├── question.rs
        ├── question
        │   ├── create_answer.rs
        │   ├── create_answer_comment.rs
        │   ├── create_question.rs
        │   ├── create_question_comment.rs
        │   ├── get_question.rs
        │   ├── list_questions.rs
        │   ├── update_answer.rs
        │   ├── update_comment.rs
        │   ├── update_question.rs
        │   ├── vote_answer.rs
        │   ├── vote_comment.rs
        │   └── vote_question.rs
        ├── repository.rs
        ├── repository
        │   ├── create_repository.rs
        │   ├── delete_repository.rs
        │   ├── get_repository_blob.rs
        │   ├── get_repository_blobs.rs
        │   ├── get_repository_commit.rs
        │   ├── get_repository_commit_diff.rs
        │   ├── get_repository_commits.rs
        │   ├── get_repository_file_commits.rs
        │   ├── get_repository_paths.rs
        │   ├── get_repository_resources.rs
        │   ├── get_repository_settings.rs
        │   └── update_repository_settings.rs
        ├── review.rs
        ├── review
        │   ├── add_reviewer.rs
        │   ├── get_review.rs
        │   ├── get_review_diff.rs
        │   ├── list_reviews.rs
        │   ├── merge_diff.rs
        │   ├── publish_review.rs
        │   ├── remove_reviewer.rs
        │   ├── resolve_review_comment.rs
        │   ├── submit_review.rs
        │   ├── update_diff.rs
        │   ├── update_review.rs
        │   └── update_review_comment.rs
        ├── runner.rs
        ├── runner
        │   ├── create_runner.rs
        │   ├── delete_runner.rs
        │   ├── get_runner.rs
        │   ├── list_runners.rs
        │   ├── refresh_runner_token.rs
        │   └── verify_runner.rs
        ├── task.rs
        ├── task
        │   ├── issue_task_token.rs
        │   ├── poll_task.rs
        │   └── update_task.rs
        ├── user.rs
        └── user
            ├── get_current_user.rs
            ├── get_current_user_settings.rs
            ├── get_user.rs
            ├── has_user.rs
            ├── list_user_organizations.rs
            ├── list_user_repositories.rs
            ├── list_user_reviews.rs
            ├── update_current_user.rs
            └── update_current_user_settings.rs
```

### APIs

- **Core traits** ([gitdot-api/src/resource.rs](gitdot-api/src/resource.rs), [gitdot-api/src/endpoint.rs](gitdot-api/src/endpoint.rs))
  - `ApiResource` — marker trait (`Serialize + PartialEq + DeserializeOwned`); blanket impls for `Vec<T>`, `Option<T>`, `()`
  - `ApiRequest` — marker trait (`Serialize + DeserializeOwned + Send`); blanket impl for `()`
  - `Endpoint` — associates `PATH`, `METHOD`, `Request: ApiRequest`, `Response: ApiResource`

- **Proc macros** ([gitdot-api/derive/src/lib.rs](gitdot-api/derive/src/lib.rs))
  - `#[derive(ApiResource)]` — implements `ApiResource` for any struct/enum
  - `#[derive(ApiRequest)]` — implements `ApiRequest` for any struct/enum

- **Resources** — data structs returned by the API
  - `build` ([gitdot-api/src/resource/build.rs](gitdot-api/src/resource/build.rs))
    - `BuildResource`
  - `migration` ([gitdot-api/src/resource/migration.rs](gitdot-api/src/resource/migration.rs))
    - `GitHubInstallationResource`, `GitHubRepositoryResource`, `MigrationResource`, `MigrationRepositoryResource`
  - `oauth` ([gitdot-api/src/resource/oauth.rs](gitdot-api/src/resource/oauth.rs))
    - `DeviceCodeResource`, `TokenResource`
  - `organization` ([gitdot-api/src/resource/organization.rs](gitdot-api/src/resource/organization.rs))
    - `OrganizationResource`, `OrganizationMemberResource`
  - `question` ([gitdot-api/src/resource/question.rs](gitdot-api/src/resource/question.rs))
    - `QuestionResource`, `AnswerResource`, `CommentResource`, `AuthorResource`, `VoteResource`
  - `repository` ([gitdot-api/src/resource/repository.rs](gitdot-api/src/resource/repository.rs))
    - `RepositoryResource`, `RepositoryPathsResource`, `RepositoryPathResource`, `PathType`
    - `RepositoryFileResource`, `RepositoryFolderResource`, `RepositoryBlobsResource`, `RepositoryBlobResource`
    - `RepositoryCommitsResource`, `RepositoryCommitResource`, `CommitAuthorResource`
    - `RepositoryDiffStatResource`, `RepositoryDiffFileResource`, `DiffHunkResource`, `DiffPairResource`, `DiffLineResource`, `DiffChangeResource`, `SyntaxHighlight`
    - `RepositoryCommitDiffResource`, `RepositorySettingsResource`
    - `RepositoryQuestionsResource`, `RepositoryReviewsResource`, `RepositoryBuildsResource`, `RepositoryResourcesResource`
  - `review` ([gitdot-api/src/resource/review.rs](gitdot-api/src/resource/review.rs))
    - `ReviewResource`, `ReviewAuthorResource`, `DiffResource`, `RevisionResource`, `ReviewVerdictResource`, `ReviewerResource`, `ReviewCommentResource`
  - `runner` ([gitdot-api/src/resource/runner.rs](gitdot-api/src/resource/runner.rs))
    - `RunnerResource`, `RunnerTokenResource`
  - `settings` ([gitdot-api/src/resource/settings.rs](gitdot-api/src/resource/settings.rs))
    - `CommitFilterResource`
  - `task` ([gitdot-api/src/resource/task.rs](gitdot-api/src/resource/task.rs))
    - `TaskResource`, `PollTaskResource`, `TaskTokenResource`
  - `user` ([gitdot-api/src/resource/user.rs](gitdot-api/src/resource/user.rs))
    - `UserResource`, `UserSettingsResource`, `UserRepoSettingsResource`

- **Endpoints** — ZST structs implementing `Endpoint`
  - `build` ([gitdot-api/src/endpoint/build/](gitdot-api/src/endpoint/build/))
    - `CreateBuild`, `GetBuild`, `ListBuilds`, `ListBuildTasks`
  - `migration` ([gitdot-api/src/endpoint/migration/](gitdot-api/src/endpoint/migration/))
    - `GetMigration`, `ListMigrations`
    - `CreateGitHubInstallation`, `ListGitHubInstallations`, `ListGitHubInstallationRepositories`, `MigrateGitHubRepositories`
  - `oauth` ([gitdot-api/src/endpoint/oauth/](gitdot-api/src/endpoint/oauth/))
    - `CreateDeviceCode`, `AuthorizeDevice`, `PollToken`
  - `organization` ([gitdot-api/src/endpoint/organization/](gitdot-api/src/endpoint/organization/))
    - `CreateOrganization`, `ListOrganizations`, `AddMember`, `ListOrganizationMembers`, `ListOrganizationRepositories`
  - `question` ([gitdot-api/src/endpoint/question/](gitdot-api/src/endpoint/question/))
    - `CreateQuestion`, `GetQuestion`, `ListQuestions`, `UpdateQuestion`
    - `CreateAnswer`, `UpdateAnswer`, `VoteQuestion`, `VoteAnswer`
    - `CreateQuestionComment`, `CreateAnswerComment`, `UpdateComment`, `VoteComment`
  - `repository` ([gitdot-api/src/endpoint/repository/](gitdot-api/src/endpoint/repository/))
    - `CreateRepository`, `DeleteRepository`
    - `GetRepositoryResources`, `GetRepositoryPaths`, `GetRepositoryBlob`, `GetRepositoryBlobs`
    - `GetRepositoryCommits`, `GetRepositoryCommit`, `GetRepositoryCommitDiff`, `GetRepositoryFileCommits`
    - `GetRepositorySettings`, `UpdateRepositorySettings`
  - `review` ([gitdot-api/src/endpoint/review/](gitdot-api/src/endpoint/review/))
    - `GetReview`, `ListReviews`, `UpdateReview`, `PublishReview`, `SubmitReview`
    - `GetReviewDiff`, `UpdateDiff`, `MergeDiff`
    - `AddReviewer`, `RemoveReviewer`
    - `UpdateReviewComment`, `ResolveReviewComment`
  - `runner` ([gitdot-api/src/endpoint/runner/](gitdot-api/src/endpoint/runner/))
    - `CreateRunner`, `GetRunner`, `ListRunners`, `DeleteRunner`, `VerifyRunner`, `CreateRunnerToken`
  - `task` ([gitdot-api/src/endpoint/task/](gitdot-api/src/endpoint/task/))
    - `IssueTaskToken`, `PollTask`, `UpdateTask`
  - `user` ([gitdot-api/src/endpoint/user/](gitdot-api/src/endpoint/user/))
    - `GetUser`, `HasUser`, `GetCurrentUser`, `UpdateCurrentUser`
    - `GetCurrentUserSettings`, `UpdateCurrentUserSettings`
    - `ListUserRepositories`, `ListUserOrganizations`, `ListUserReviews`
