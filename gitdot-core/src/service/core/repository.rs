use async_trait::async_trait;
use chrono::Utc;
use uuid::Uuid;

use crate::{
    client::{Git2Client, GitClient},
    dto::{
        CommitResponse, CreateRepositoryCommitFilterRequest, CreateRepositoryRequest,
        DeleteRepositoryCommitFilterRequest, DeleteRepositoryRequest, GetRepositoryActivityRequest,
        GetRepositoryBlobRequest, GetRepositoryBlobsRequest, GetRepositoryCommitBlobsRequest,
        GetRepositoryCommitRequest, GetRepositoryPathsRequest, GetRepositoryRequest,
        InitialCommitFile, ListRepositoryCommitFiltersRequest, ListRepositoryCommitsRequest,
        MAX_PER_PAGE_LIMIT, Page, RepositoryActivityEvent, RepositoryBlobPairResponse,
        RepositoryBlobResponse, RepositoryBlobsResponse, RepositoryCommitFilterResponse,
        RepositoryPathsResponse, RepositoryResponse, StarRepositoryRequest,
        UnstarRepositoryRequest, UpdateRepositoryCommitFilterRequest, UpdateRepositoryRequest,
    },
    error::{ConflictError, NotFoundError, OptionNotFoundExt, RepositoryError},
    model::{CommitDiff, RepositoryOwnerType},
    repository::{
        CommitRepository, OrganizationRepository, PgCommitRepository, PgOrganizationRepository,
        PgRepositoryRepository, PgUserRepository, RepositoryRepository, UserRepository,
    },
    util::{
        auth::is_offensive_name,
        cursor,
        git::{
            DEFAULT_BRANCH, GitHookType, POST_RECEIVE_SCRIPT, PRE_RECEIVE_SCRIPT,
            PROC_RECEIVE_SCRIPT, ZERO_SHA,
        },
        template::{gitignore_for, license_for},
    },
};

/// Repository domain: lifecycle (create/update/delete), browsing (blobs, paths,
/// commits, diffs), discovery (latest/trending), stars, activity, and per-repo
/// commit filters. Bridges the database (`RepositoryRepository`/`CommitRepository`)
/// and the on-disk bare git repos accessed through a [`GitClient`].
#[async_trait]
pub trait RepositoryService: Send + Sync + 'static {
    /// Creates a new repository, both on disk and in the database.
    ///
    /// Creates the bare git repo under `GIT_PROJECT_ROOT` and installs the
    /// pre-receive, post-receive and proc-receive hooks, then inserts the
    /// `repositories` row (owner resolved to a user or organization id by
    /// `owner_type`). If any requested seed files are present (README from
    /// `init_readme`, `.gitignore`, `LICENSE`), an initial commit is written on
    /// the default branch and recorded.
    ///
    /// Failures are unwound: the on-disk repo (and DB row, for a failed initial
    /// commit) are best-effort deleted before the error is returned.
    ///
    /// # Errors
    /// - [`RepositoryError::Conflict`] if a repo with the same name already
    ///   exists on disk for the owner.
    /// - [`RepositoryError::NotFound`] if `owner_type` is organization and no
    ///   such organization exists, or the committing user/email is missing.
    async fn create_repository(
        &self,
        request: CreateRepositoryRequest,
    ) -> Result<RepositoryResponse, RepositoryError>;

    /// Returns repository metadata for `owner/repo`, scoped to `request.user_id`
    /// so private repos are only visible to permitted viewers.
    ///
    /// # Errors
    /// - [`RepositoryError::NotFound`] if no matching (visible) repository exists.
    async fn get_repository(
        &self,
        request: GetRepositoryRequest,
    ) -> Result<RepositoryResponse, RepositoryError>;

    /// Returns a single blob (file contents or a folder listing) at `ref_name`
    /// and `path`, read directly from the bare git repo.
    async fn get_repository_blob(
        &self,
        request: GetRepositoryBlobRequest,
    ) -> Result<RepositoryBlobResponse, RepositoryError>;

    /// Returns multiple blobs in one call. When more than one ref is given,
    /// reads the single `paths[0]` across all `refs`; otherwise reads all
    /// `paths` at the single `refs[0]`.
    async fn get_repository_blobs(
        &self,
        request: GetRepositoryBlobsRequest,
    ) -> Result<RepositoryBlobsResponse, RepositoryError>;

    /// Returns the repository's full tree of paths at `ref_name`.
    async fn get_repository_paths(
        &self,
        request: GetRepositoryPathsRequest,
    ) -> Result<RepositoryPathsResponse, RepositoryError>;

    /// Returns repository metadata by id, unscoped (no visibility filtering).
    ///
    /// # Errors
    /// - [`RepositoryError::NotFound`] if no repository has that id.
    async fn get_repository_by_id(&self, id: Uuid) -> Result<RepositoryResponse, RepositoryError>;

    /// Deletes `owner/repo` from both disk and the database.
    ///
    /// Removes the bare git repo via the [`GitClient`] first, then the DB row.
    /// Unscoped — does not filter by viewer.
    ///
    /// # Errors
    /// - [`RepositoryError::NotFound`] if the repository does not exist.
    async fn delete_repository(
        &self,
        request: DeleteRepositoryRequest,
    ) -> Result<(), RepositoryError>;

    /// Updates the repository's `description` and/or `readonly` flag and returns
    /// the updated metadata. Unscoped lookup by `owner/repo`.
    ///
    /// # Errors
    /// - [`RepositoryError::NotFound`] if the repository does not exist.
    async fn update_repository(
        &self,
        request: UpdateRepositoryRequest,
    ) -> Result<RepositoryResponse, RepositoryError>;

    /// Resolves `ref_name` (branch, tag, or revision) to its commit SHA in the
    /// bare git repo.
    async fn resolve_ref_sha(
        &self,
        owner: &str,
        repo: &str,
        ref_name: &str,
    ) -> Result<String, RepositoryError>;

    /// Returns a single stored commit (metadata plus its per-file diff stats)
    /// for `owner/repo` at `sha`, read from the database.
    ///
    /// # Errors
    /// - [`RepositoryError::NotFound`] if the repository or the commit is absent.
    async fn get_repository_commit(
        &self,
        request: GetRepositoryCommitRequest,
    ) -> Result<CommitResponse, RepositoryError>;

    /// Returns the before/after blob pairs for every file touched by the commit
    /// `sha`, suitable for rendering a diff view.
    ///
    /// Looks up the stored commit to find its parent and changed paths, then
    /// reads each path's blob at the parent and at the commit (concurrently).
    /// For a root commit (all-zero parent SHA) the "old" side is empty.
    ///
    /// # Errors
    /// - [`RepositoryError::NotFound`] if the repository or the commit is absent.
    async fn get_repository_commit_blobs(
        &self,
        request: GetRepositoryCommitBlobsRequest,
    ) -> Result<Vec<RepositoryBlobPairResponse>, RepositoryError>;

    /// Lists commits for `owner/repo` on a ref, cursor-paginated.
    ///
    /// `ref_name` is normalized: `HEAD` resolves to the repo's default ref, a
    /// bare branch name is expanded to `refs/heads/<name>`, and a fully-qualified
    /// `refs/...` name is used as-is. Optional `from`/`to` bound the range. The
    /// returned page carries an encoded `next_cursor` when more results exist.
    ///
    /// # Errors
    /// - [`RepositoryError::NotFound`] if the repository does not exist.
    async fn list_repository_commits(
        &self,
        request: ListRepositoryCommitsRequest,
    ) -> Result<Page<CommitResponse>, RepositoryError>;

    /// Returns up to 100 most recently created repositories, for discovery.
    async fn list_latest_repositories(&self) -> Result<Vec<RepositoryResponse>, RepositoryError>;

    /// Returns up to 100 trending repositories, for discovery.
    async fn list_trending_repositories(&self) -> Result<Vec<RepositoryResponse>, RepositoryError>;

    /// Stars `owner/repo` on behalf of `request.user_id`.
    ///
    /// # Errors
    /// - [`RepositoryError::NotFound`] if the repository does not exist.
    /// - [`RepositoryError::Conflict`] if the user has already starred it.
    async fn star_repository(&self, request: StarRepositoryRequest) -> Result<(), RepositoryError>;

    /// Removes `request.user_id`'s star from `owner/repo`.
    ///
    /// # Errors
    /// - [`RepositoryError::NotFound`] if the repository does not exist.
    /// - [`RepositoryError::Conflict`] if no star existed to remove.
    async fn unstar_repository(
        &self,
        request: UnstarRepositoryRequest,
    ) -> Result<(), RepositoryError>;

    /// Returns the repository's recent activity feed (up to 25 events).
    ///
    /// Currently surfaces the most recent stars as `Starred` events.
    ///
    /// # Errors
    /// - [`RepositoryError::NotFound`] if the repository does not exist.
    async fn get_repository_activity(
        &self,
        request: GetRepositoryActivityRequest,
    ) -> Result<Vec<RepositoryActivityEvent>, RepositoryError>;

    /// Lists a repository's saved commit filters, cursor-paginated.
    ///
    /// # Errors
    /// - [`RepositoryError::NotFound`] if the repository does not exist.
    async fn list_repository_commit_filters(
        &self,
        request: ListRepositoryCommitFiltersRequest,
    ) -> Result<Page<RepositoryCommitFilterResponse>, RepositoryError>;

    /// Creates a named commit filter (by authors, tags, and/or paths) for the
    /// repository. Names must be unique within the repository.
    ///
    /// # Errors
    /// - [`RepositoryError::NotFound`] if the repository does not exist.
    /// - [`RepositoryError::Conflict`] if a filter with the same name exists.
    async fn create_repository_commit_filter(
        &self,
        request: CreateRepositoryCommitFilterRequest,
    ) -> Result<RepositoryCommitFilterResponse, RepositoryError>;

    /// Updates an existing commit filter by id and returns it.
    ///
    /// # Errors
    /// - [`RepositoryError::NotFound`] if no filter has that id.
    async fn update_repository_commit_filter(
        &self,
        request: UpdateRepositoryCommitFilterRequest,
    ) -> Result<RepositoryCommitFilterResponse, RepositoryError>;

    /// Deletes a commit filter by id.
    ///
    /// # Errors
    /// - [`RepositoryError::NotFound`] if no filter has that id.
    async fn delete_repository_commit_filter(
        &self,
        request: DeleteRepositoryCommitFilterRequest,
    ) -> Result<(), RepositoryError>;
}

#[derive(Debug, Clone)]
pub struct RepositoryServiceImpl<G, O, R, C, U>
where
    G: GitClient,
    O: OrganizationRepository,
    R: RepositoryRepository,
    C: CommitRepository,
    U: UserRepository,
{
    git_client: G,
    org_repo: O,
    repo_repo: R,
    commit_repo: C,
    user_repo: U,
}

impl
    RepositoryServiceImpl<
        Git2Client,
        PgOrganizationRepository,
        PgRepositoryRepository,
        PgCommitRepository,
        PgUserRepository,
    >
{
    pub fn new(
        git_client: Git2Client,
        org_repo: PgOrganizationRepository,
        repo_repo: PgRepositoryRepository,
        commit_repo: PgCommitRepository,
        user_repo: PgUserRepository,
    ) -> Self {
        Self {
            git_client,
            org_repo,
            repo_repo,
            commit_repo,
            user_repo,
        }
    }
}

impl<G, O, R, C, U> RepositoryServiceImpl<G, O, R, C, U>
where
    G: GitClient,
    O: OrganizationRepository,
    R: RepositoryRepository,
    C: CommitRepository,
    U: UserRepository,
{
    fn get_initial_files(
        &self,
        repo_name: &str,
        request: &CreateRepositoryRequest,
    ) -> Vec<InitialCommitFile> {
        let mut files = Vec::new();
        if request.init_readme {
            files.push(InitialCommitFile {
                path: "README.md",
                content: format!("# {}\n", repo_name),
            });
        }
        if let Some(t) = request.gitignore {
            files.push(InitialCommitFile {
                path: ".gitignore",
                content: gitignore_for(t).to_string(),
            });
        }
        if let Some(t) = request.license {
            files.push(InitialCommitFile {
                path: "LICENSE",
                content: license_for(t).to_string(),
            });
        }
        files
    }

    async fn create_initial_commit(
        &self,
        request: &CreateRepositoryRequest,
        repo_name: &str,
        repo_id: Uuid,
        files: Vec<InitialCommitFile>,
    ) -> Result<(), RepositoryError> {
        let user = self
            .user_repo
            .get_by_id(request.user_id)
            .await?
            .or_not_found("user", request.user_id.to_string())?;
        let primary_email = user
            .primary_email()
            .or_not_found("user_email", request.user_id.to_string())?
            .email
            .clone();

        let diffs: Vec<CommitDiff> = files
            .iter()
            .map(|f| CommitDiff {
                path: f.path.to_string(),
                lines_added: f.content.lines().count() as i32,
                lines_removed: 0,
            })
            .collect();

        let committed_at = Utc::now();
        let sha = self
            .git_client
            .create_initial_commit(
                &request.owner_name,
                repo_name,
                files,
                &user.name,
                &primary_email,
                committed_at,
            )
            .await?;

        let ref_name = format!("refs/heads/{}", DEFAULT_BRANCH);
        self.commit_repo
            .create_bulk(
                &[Some(user.id)],
                &[user.name.clone()],
                &[primary_email],
                &[repo_id],
                &[ref_name],
                &[sha],
                &[ZERO_SHA.to_string()],
                &["Initial commit".to_string()],
                &[committed_at],
                &[diffs],
                &[None],
                &[None],
            )
            .await?;

        Ok(())
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl<G, O, R, C, U> RepositoryService for RepositoryServiceImpl<G, O, R, C, U>
where
    G: GitClient,
    O: OrganizationRepository,
    R: RepositoryRepository,
    C: CommitRepository,
    U: UserRepository,
{
    async fn create_repository(
        &self,
        request: CreateRepositoryRequest,
    ) -> Result<RepositoryResponse, RepositoryError> {
        let repo_name = request.name.to_string();
        if is_offensive_name(&repo_name) {
            return Err(
                ConflictError::new("repository", format!("{repo_name} is not allowed")).into(),
            );
        }
        if self
            .git_client
            .repo_exists(&request.owner_name, &repo_name)
            .await
        {
            return Err(RepositoryError::Conflict(ConflictError::new(
                "repository",
                repo_name,
            )));
        }

        let owner_id = match request.owner_type {
            RepositoryOwnerType::User => request.user_id,
            RepositoryOwnerType::Organization => {
                let org = self
                    .org_repo
                    .get(&request.owner_name)
                    .await?
                    .or_not_found("owner", request.owner_name.as_ref())?;
                org.id
            }
        };
        self.git_client
            .create_repo(&request.owner_name, &repo_name)
            .await?;

        self.git_client
            .install_hook(
                &request.owner_name,
                &repo_name,
                GitHookType::PreReceive,
                PRE_RECEIVE_SCRIPT,
            )
            .await?;
        self.git_client
            .install_hook(
                &request.owner_name,
                &repo_name,
                GitHookType::PostReceive,
                POST_RECEIVE_SCRIPT,
            )
            .await?;
        self.git_client
            .install_hook(
                &request.owner_name,
                &repo_name,
                GitHookType::ProcReceive,
                PROC_RECEIVE_SCRIPT,
            )
            .await?;

        let repository = match self
            .repo_repo
            .create(
                &repo_name,
                owner_id,
                &request.owner_type,
                &request.visibility,
                request.description.clone(),
                false,
                None,
            )
            .await
        {
            Ok(repo) => repo,
            Err(e) => {
                let _ = self
                    .git_client
                    .delete_repo(&request.owner_name, &repo_name)
                    .await;
                return Err(e.into());
            }
        };

        let files = self.get_initial_files(&repo_name, &request);
        if !files.is_empty() {
            if let Err(e) = self
                .create_initial_commit(&request, &repo_name, repository.id, files)
                .await
            {
                let _ = self.repo_repo.delete(repository.id).await;
                let _ = self
                    .git_client
                    .delete_repo(&request.owner_name, &repo_name)
                    .await;
                return Err(e);
            }
        }

        Ok(repository.into())
    }

    async fn get_repository(
        &self,
        request: GetRepositoryRequest,
    ) -> Result<RepositoryResponse, RepositoryError> {
        let owner = request.owner.as_ref();
        let repo = request.repo.as_ref();

        let repository = self
            .repo_repo
            .get(owner, repo, request.user_id)
            .await?
            .or_not_found("repository", format!("{}/{}", owner, repo))?;

        Ok(RepositoryResponse::from(repository))
    }

    async fn get_repository_blob(
        &self,
        request: GetRepositoryBlobRequest,
    ) -> Result<RepositoryBlobResponse, RepositoryError> {
        self.git_client
            .get_repo_blob(
                &request.owner_name,
                &request.name,
                &request.ref_name,
                &request.path,
            )
            .await
            .map_err(Into::into)
    }

    async fn get_repository_blobs(
        &self,
        request: GetRepositoryBlobsRequest,
    ) -> Result<RepositoryBlobsResponse, RepositoryError> {
        let blobs = self
            .git_client
            .get_repo_blobs(
                &request.owner_name,
                &request.name,
                &request.paths,
                &request.refs,
            )
            .await?;
        Ok(RepositoryBlobsResponse {
            blobs: blobs.into_iter().flatten().collect(),
        })
    }

    async fn get_repository_paths(
        &self,
        request: GetRepositoryPathsRequest,
    ) -> Result<RepositoryPathsResponse, RepositoryError> {
        self.git_client
            .get_repo_paths(&request.owner_name, &request.name, &request.ref_name)
            .await
            .map_err(Into::into)
    }

    async fn get_repository_by_id(&self, id: Uuid) -> Result<RepositoryResponse, RepositoryError> {
        let repository = self
            .repo_repo
            .get_by_id(id, None)
            .await?
            .or_not_found("repository", id)?;

        Ok(repository.into())
    }

    async fn delete_repository(
        &self,
        request: DeleteRepositoryRequest,
    ) -> Result<(), RepositoryError> {
        let owner = request.owner.as_ref();
        let repo = request.repo.as_ref();

        let repository = self
            .repo_repo
            .get(owner, repo, None)
            .await?
            .or_not_found("repository", format!("{}/{}", owner, repo))?;

        self.git_client.delete_repo(owner, repo).await?;
        self.repo_repo.delete(repository.id).await?;

        Ok(())
    }

    async fn update_repository(
        &self,
        request: UpdateRepositoryRequest,
    ) -> Result<RepositoryResponse, RepositoryError> {
        let owner = request.owner.as_ref();
        let repo = request.repo.as_ref();

        let repository = self
            .repo_repo
            .get(owner, repo, None)
            .await?
            .or_not_found("repository", format!("{}/{}", owner, repo))?;

        let updated = self
            .repo_repo
            .update(repository.id, request.description, request.readonly)
            .await?
            .or_not_found("repository", format!("{}/{}", owner, repo))?;

        Ok(updated.into())
    }

    async fn resolve_ref_sha(
        &self,
        owner: &str,
        repo: &str,
        ref_name: &str,
    ) -> Result<String, RepositoryError> {
        self.git_client
            .resolve_ref_sha(owner, repo, ref_name)
            .await
            .map_err(Into::into)
    }

    async fn get_repository_commit(
        &self,
        request: GetRepositoryCommitRequest,
    ) -> Result<CommitResponse, RepositoryError> {
        let owner = request.owner.to_string();
        let repo_name = request.repo.to_string();

        let repository = self
            .repo_repo
            .get(&owner, &repo_name, None)
            .await?
            .or_not_found("repository", format!("{}/{}", owner, repo_name))?;

        let commit = self
            .commit_repo
            .get_commit(repository.id, &request.sha)
            .await?
            .map(Into::into)
            .or_not_found("commit", &request.sha)?;

        Ok(commit)
    }

    async fn get_repository_commit_blobs(
        &self,
        request: GetRepositoryCommitBlobsRequest,
    ) -> Result<Vec<RepositoryBlobPairResponse>, RepositoryError> {
        let owner = request.owner.to_string();
        let repo_name = request.repo.to_string();

        let repository = self
            .repo_repo
            .get(&owner, &repo_name, None)
            .await?
            .or_not_found("repository", format!("{}/{}", owner, repo_name))?;

        let commit = self
            .commit_repo
            .get_commit(repository.id, &request.sha)
            .await?
            .or_not_found("commit", &request.sha)?;

        let sha = commit.sha.clone();
        let parent_sha = commit.parent_sha.clone();
        let is_initial = parent_sha == "0000000000000000000000000000000000000000";

        let paths: Vec<String> = commit.diffs.iter().map(|d| d.path.clone()).collect();
        let parent_refs: &[String] = if is_initial {
            &[]
        } else {
            std::slice::from_ref(&parent_sha)
        };
        let (left, right) = tokio::try_join!(
            self.git_client
                .get_repo_blobs(&owner, &repo_name, &paths, parent_refs),
            self.git_client
                .get_repo_blobs(&owner, &repo_name, &paths, std::slice::from_ref(&sha)),
        )?;

        let pairs = paths
            .into_iter()
            .enumerate()
            .map(|(i, path)| RepositoryBlobPairResponse {
                path,
                old: left.get(i).cloned().flatten(),
                new: right.get(i).cloned().flatten(),
            })
            .collect();

        Ok(pairs)
    }

    async fn list_repository_commits(
        &self,
        request: ListRepositoryCommitsRequest,
    ) -> Result<Page<CommitResponse>, RepositoryError> {
        let owner = request.owner.to_string();
        let repo_name = request.repo.to_string();

        let repository = self
            .repo_repo
            .get(&owner, &repo_name, None)
            .await?
            .or_not_found("repository", format!("{}/{}", owner, repo_name))?;

        let ref_name = if request.ref_name == "HEAD" {
            self.git_client
                .get_default_ref(&owner, &repo_name)
                .await
                .map_err(RepositoryError::from)?
        } else if !request.ref_name.starts_with("refs/") {
            format!("refs/heads/{}", request.ref_name)
        } else {
            request.ref_name
        };

        let (commits, next_cursor) = self
            .commit_repo
            .list_by_repository(
                repository.id,
                &ref_name,
                request.from,
                request.to,
                request.cursor,
                request.limit as i64,
            )
            .await?;

        Ok(Page {
            data: commits.into_iter().map(CommitResponse::from).collect(),
            next_cursor: next_cursor.as_ref().map(cursor::encode),
        })
    }

    async fn list_latest_repositories(&self) -> Result<Vec<RepositoryResponse>, RepositoryError> {
        const LATEST_REPOSITORIES_LIMIT: i64 = 100;

        let repositories = self
            .repo_repo
            .list_latest(LATEST_REPOSITORIES_LIMIT)
            .await?;

        Ok(repositories
            .into_iter()
            .map(RepositoryResponse::from)
            .collect())
    }

    async fn list_trending_repositories(&self) -> Result<Vec<RepositoryResponse>, RepositoryError> {
        const TRENDING_REPOSITORIES_LIMIT: i64 = 100;

        let repositories = self
            .repo_repo
            .list_trending(TRENDING_REPOSITORIES_LIMIT)
            .await?;

        Ok(repositories
            .into_iter()
            .map(RepositoryResponse::from)
            .collect())
    }

    async fn star_repository(&self, request: StarRepositoryRequest) -> Result<(), RepositoryError> {
        let owner = request.owner.as_ref();
        let repo = request.repo.as_ref();

        let repository_id = self
            .repo_repo
            .get_id(owner, repo)
            .await?
            .or_not_found("repository", format!("{}/{}", owner, repo))?;

        let star = self.repo_repo.star(repository_id, request.user_id).await?;
        if star.is_none() {
            return Err(RepositoryError::Conflict(ConflictError::new(
                "star",
                format!("{}/{}", owner, repo),
            )));
        }
        Ok(())
    }

    async fn unstar_repository(
        &self,
        request: UnstarRepositoryRequest,
    ) -> Result<(), RepositoryError> {
        let owner = request.owner.as_ref();
        let repo = request.repo.as_ref();

        let repository_id = self
            .repo_repo
            .get_id(owner, repo)
            .await?
            .or_not_found("repository", format!("{}/{}", owner, repo))?;

        let removed = self
            .repo_repo
            .unstar(repository_id, request.user_id)
            .await?;
        if !removed {
            return Err(RepositoryError::Conflict(ConflictError::new(
                "star",
                format!("{}/{}", owner, repo),
            )));
        }
        Ok(())
    }

    async fn get_repository_activity(
        &self,
        request: GetRepositoryActivityRequest,
    ) -> Result<Vec<RepositoryActivityEvent>, RepositoryError> {
        const ACTIVITY_EVENT_LIMIT: i64 = 25;

        let owner = request.owner.as_ref();
        let repo = request.repo.as_ref();

        let repository = self
            .repo_repo
            .get(owner, repo, None)
            .await?
            .or_not_found("repository", format!("{}/{}", owner, repo))?;

        let stars = self
            .repo_repo
            .list_recent_stars(repository.id, ACTIVITY_EVENT_LIMIT)
            .await?;

        Ok(stars
            .into_iter()
            .map(|(user, at)| RepositoryActivityEvent::Starred {
                user: user.into(),
                at,
            })
            .collect())
    }

    async fn list_repository_commit_filters(
        &self,
        request: ListRepositoryCommitFiltersRequest,
    ) -> Result<Page<RepositoryCommitFilterResponse>, RepositoryError> {
        let owner = request.owner.as_ref();
        let repo = request.repo.as_ref();

        let repository = self
            .repo_repo
            .get(owner, repo, None)
            .await?
            .or_not_found("repository", format!("{}/{}", owner, repo))?;

        let (filters, next_cursor) = self
            .repo_repo
            .list_commit_filters(repository.id, request.cursor, request.limit as i64)
            .await?;

        Ok(Page {
            data: filters
                .into_iter()
                .map(RepositoryCommitFilterResponse::from)
                .collect(),
            next_cursor: next_cursor.as_ref().map(cursor::encode),
        })
    }

    async fn create_repository_commit_filter(
        &self,
        request: CreateRepositoryCommitFilterRequest,
    ) -> Result<RepositoryCommitFilterResponse, RepositoryError> {
        let owner = request.owner.as_ref();
        let repo = request.repo.as_ref();
        let name = request.name.to_string();

        let repository = self
            .repo_repo
            .get(owner, repo, None)
            .await?
            .or_not_found("repository", format!("{}/{}", owner, repo))?;

        let (existing, _) = self
            .repo_repo
            .list_commit_filters(repository.id, None, MAX_PER_PAGE_LIMIT as i64)
            .await?;
        if existing.iter().any(|f| f.name == name) {
            return Err(RepositoryError::Conflict(ConflictError::new(
                "commit filter",
                format!("{}/{}/{}", owner, repo, name),
            )));
        }

        let filter = self
            .repo_repo
            .create_commit_filter(
                repository.id,
                &name,
                request.authors,
                request.tags,
                request.paths,
            )
            .await?;
        Ok(filter.into())
    }

    async fn update_repository_commit_filter(
        &self,
        request: UpdateRepositoryCommitFilterRequest,
    ) -> Result<RepositoryCommitFilterResponse, RepositoryError> {
        let owner = request.owner.as_ref();
        let repo = request.repo.as_ref();

        let repository = self
            .repo_repo
            .get(owner, repo, None)
            .await?
            .or_not_found("repository", format!("{}/{}", owner, repo))?;

        let filter = self
            .repo_repo
            .update_commit_filter(
                repository.id,
                request.filter_id,
                request.name.as_ref(),
                request.authors,
                request.tags,
                request.paths,
            )
            .await?
            .or_not_found("commit filter", request.filter_id)?;

        Ok(filter.into())
    }

    async fn delete_repository_commit_filter(
        &self,
        request: DeleteRepositoryCommitFilterRequest,
    ) -> Result<(), RepositoryError> {
        let owner = request.owner.as_ref();
        let repo = request.repo.as_ref();

        let repository = self
            .repo_repo
            .get(owner, repo, None)
            .await?
            .or_not_found("repository", format!("{}/{}", owner, repo))?;

        let deleted = self
            .repo_repo
            .delete_commit_filter(repository.id, request.filter_id)
            .await?;
        if !deleted {
            return Err(NotFoundError::new("commit filter", request.filter_id).into());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use uuid::Uuid;

    use super::{RepositoryService, RepositoryServiceImpl};
    use crate::{
        dto::{
            CreateRepositoryCommitFilterRequest, CreateRepositoryRequest,
            DeleteRepositoryCommitFilterRequest, DeleteRepositoryRequest,
            GetRepositoryActivityRequest, GetRepositoryCommitRequest, GetRepositoryRequest,
            ListRepositoryCommitFiltersRequest, ListRepositoryCommitsRequest,
            StarRepositoryRequest, UnstarRepositoryRequest, UpdateRepositoryCommitFilterRequest,
            UpdateRepositoryRequest,
        },
        error::{DatabaseError, RepositoryError},
        model::{CommitFilter, RepositoryOwnerType, RepositoryStar, RepositoryVisibility},
        service::{
            test_client::MockGitClient,
            test_common::{create_commit, create_repository, create_user},
            test_repository::{
                MockCommitRepository, MockOrganizationRepository, MockRepositoryRepository,
                MockUserRepository,
            },
        },
    };

    type Service = RepositoryServiceImpl<
        MockGitClient,
        MockOrganizationRepository,
        MockRepositoryRepository,
        MockCommitRepository,
        MockUserRepository,
    >;

    fn create_service() -> Service {
        RepositoryServiceImpl {
            git_client: MockGitClient::default(),
            org_repo: MockOrganizationRepository::new(),
            repo_repo: MockRepositoryRepository::new(),
            commit_repo: MockCommitRepository::new(),
            user_repo: MockUserRepository::new(),
        }
    }

    fn public_repo() -> crate::model::Repository {
        create_repository(
            Uuid::new_v4(),
            RepositoryOwnerType::User,
            RepositoryVisibility::Public,
        )
    }

    fn create_commit_filter(name: &str) -> CommitFilter {
        CommitFilter {
            id: Uuid::new_v4(),
            repository_id: Uuid::new_v4(),
            name: name.to_string(),
            authors: None,
            tags: None,
            paths: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    // --- create_repository ---

    #[tokio::test]
    async fn create_repository_conflicts_when_repo_exists_on_disk() {
        let mut service = create_service();
        service.git_client = MockGitClient::default().with_repo_exists(true);

        let req = CreateRepositoryRequest::new(
            "my-repo",
            Uuid::new_v4(),
            "alice",
            "user",
            "public",
            None,
            false,
            None,
            None,
        )
        .unwrap();
        let err = service.create_repository(req).await.unwrap_err();
        assert!(matches!(err, RepositoryError::Conflict(_)));
    }

    #[tokio::test]
    async fn create_repository_missing_org_owner_is_not_found() {
        let mut service = create_service();
        service.org_repo.expect_get().returning(|_| Ok(None));

        let req = CreateRepositoryRequest::new(
            "my-repo",
            Uuid::new_v4(),
            "acme",
            "organization",
            "public",
            None,
            false,
            None,
            None,
        )
        .unwrap();
        let err = service.create_repository(req).await.unwrap_err();
        assert!(matches!(err, RepositoryError::NotFound(_)));
    }

    #[tokio::test]
    async fn create_repository_user_owner_no_seed_files_succeeds() {
        let mut service = create_service();
        service
            .repo_repo
            .expect_create()
            .returning(|_, _, _, _, _, _, _| Ok(public_repo()));

        let req = CreateRepositoryRequest::new(
            "my-repo",
            Uuid::new_v4(),
            "alice",
            "user",
            "public",
            None,
            false,
            None,
            None,
        )
        .unwrap();
        let resp = service.create_repository(req).await.unwrap();
        assert_eq!(resp.name, "myrepo");
        // The bare repo was created on disk under the requested owner.
        assert_eq!(
            service.git_client.created_repos(),
            vec![("alice".into(), "my-repo".into())]
        );
    }

    #[tokio::test]
    async fn create_repository_deletes_disk_repo_when_db_insert_fails() {
        let mut service = create_service();
        service
            .repo_repo
            .expect_create()
            .returning(|_, _, _, _, _, _, _| Err(DatabaseError::RowNotFound));

        let req = CreateRepositoryRequest::new(
            "my-repo",
            Uuid::new_v4(),
            "alice",
            "user",
            "public",
            None,
            false,
            None,
            None,
        )
        .unwrap();
        let err = service.create_repository(req).await.unwrap_err();
        assert!(matches!(err, RepositoryError::DatabaseError(_)));
        // The on-disk repo is unwound after the DB insert fails.
        assert_eq!(
            service.git_client.deleted_repos(),
            vec![("alice".into(), "my-repo".into())]
        );
    }

    // --- get_repository ---

    #[tokio::test]
    async fn get_repository_returns_response() {
        let mut service = create_service();
        service
            .repo_repo
            .expect_get()
            .returning(|_, _, _| Ok(Some(public_repo())));

        let resp = service
            .get_repository(GetRepositoryRequest::new(None, "alice", "myrepo").unwrap())
            .await
            .unwrap();
        assert_eq!(resp.name, "myrepo");
    }

    #[tokio::test]
    async fn get_repository_missing_is_not_found() {
        let mut service = create_service();
        service.repo_repo.expect_get().returning(|_, _, _| Ok(None));

        let err = service
            .get_repository(GetRepositoryRequest::new(None, "alice", "ghost").unwrap())
            .await
            .unwrap_err();
        assert!(matches!(err, RepositoryError::NotFound(_)));
    }

    #[tokio::test]
    async fn get_repository_by_id_missing_is_not_found() {
        let mut service = create_service();
        service
            .repo_repo
            .expect_get_by_id()
            .returning(|_, _| Ok(None));

        let err = service
            .get_repository_by_id(Uuid::new_v4())
            .await
            .unwrap_err();
        assert!(matches!(err, RepositoryError::NotFound(_)));
    }

    // --- update_repository ---

    #[tokio::test]
    async fn update_repository_updates_description() {
        let mut service = create_service();
        service
            .repo_repo
            .expect_get()
            .returning(|_, _, _| Ok(Some(public_repo())));
        service.repo_repo.expect_update().returning(|_, desc, _| {
            let mut repo = public_repo();
            repo.description = desc;
            Ok(Some(repo))
        });

        let req =
            UpdateRepositoryRequest::new("alice", "myrepo", Some("new desc".to_string()), None)
                .unwrap();
        let resp = service.update_repository(req).await.unwrap();
        assert_eq!(resp.description.as_deref(), Some("new desc"));
    }

    #[tokio::test]
    async fn update_repository_missing_is_not_found() {
        let mut service = create_service();
        service.repo_repo.expect_get().returning(|_, _, _| Ok(None));

        let req = UpdateRepositoryRequest::new("alice", "ghost", None, None).unwrap();
        let err = service.update_repository(req).await.unwrap_err();
        assert!(matches!(err, RepositoryError::NotFound(_)));
    }

    // --- delete_repository ---

    #[tokio::test]
    async fn delete_repository_removes_disk_and_db() {
        let mut service = create_service();
        service
            .repo_repo
            .expect_get()
            .returning(|_, _, _| Ok(Some(public_repo())));
        service.repo_repo.expect_delete().returning(|_| Ok(()));

        let req = DeleteRepositoryRequest::new("alice", "myrepo").unwrap();
        service.delete_repository(req).await.unwrap();
        assert_eq!(
            service.git_client.deleted_repos(),
            vec![("alice".into(), "myrepo".into())]
        );
    }

    #[tokio::test]
    async fn delete_repository_missing_is_not_found() {
        let mut service = create_service();
        service.repo_repo.expect_get().returning(|_, _, _| Ok(None));

        let req = DeleteRepositoryRequest::new("alice", "ghost").unwrap();
        let err = service.delete_repository(req).await.unwrap_err();
        assert!(matches!(err, RepositoryError::NotFound(_)));
        // No on-disk deletion when the repo was never found.
        assert!(service.git_client.deleted_repos().is_empty());
    }

    // --- get_repository_commit ---

    #[tokio::test]
    async fn get_repository_commit_returns_commit() {
        let mut service = create_service();
        service
            .repo_repo
            .expect_get()
            .returning(|_, _, _| Ok(Some(public_repo())));
        service
            .commit_repo
            .expect_get_commit()
            .returning(|_, _| Ok(Some(create_commit("abc123"))));

        let req = GetRepositoryCommitRequest::new("alice", "myrepo", "abc123".to_string()).unwrap();
        let resp = service.get_repository_commit(req).await.unwrap();
        assert_eq!(resp.sha, "abc123");
    }

    #[tokio::test]
    async fn get_repository_commit_missing_commit_is_not_found() {
        let mut service = create_service();
        service
            .repo_repo
            .expect_get()
            .returning(|_, _, _| Ok(Some(public_repo())));
        service
            .commit_repo
            .expect_get_commit()
            .returning(|_, _| Ok(None));

        let req = GetRepositoryCommitRequest::new("alice", "myrepo", "abc123".to_string()).unwrap();
        let err = service.get_repository_commit(req).await.unwrap_err();
        assert!(matches!(err, RepositoryError::NotFound(_)));
    }

    // --- list_repository_commits ---

    #[tokio::test]
    async fn list_repository_commits_expands_bare_branch_and_maps_page() {
        let mut service = create_service();
        service
            .repo_repo
            .expect_get()
            .returning(|_, _, _| Ok(Some(public_repo())));
        // A bare branch name is expanded to `refs/heads/<name>` before the lookup.
        service
            .commit_repo
            .expect_list_by_repository()
            .withf(|_, ref_name, _, _, _, _| ref_name == "refs/heads/main")
            .returning(|_, _, _, _, _, _| Ok((vec![create_commit("a"), create_commit("b")], None)));

        let req = ListRepositoryCommitsRequest::new(
            "alice",
            "myrepo",
            "main".to_string(),
            None,
            None,
            None,
            None,
        )
        .unwrap();
        let page = service.list_repository_commits(req).await.unwrap();
        assert_eq!(page.data.len(), 2);
        assert!(page.next_cursor.is_none());
    }

    #[tokio::test]
    async fn list_repository_commits_missing_repo_is_not_found() {
        let mut service = create_service();
        service.repo_repo.expect_get().returning(|_, _, _| Ok(None));

        let req = ListRepositoryCommitsRequest::new(
            "alice",
            "ghost",
            "main".to_string(),
            None,
            None,
            None,
            None,
        )
        .unwrap();
        let err = service.list_repository_commits(req).await.unwrap_err();
        assert!(matches!(err, RepositoryError::NotFound(_)));
    }

    // --- discovery ---

    #[tokio::test]
    async fn list_latest_repositories_maps_responses() {
        let mut service = create_service();
        service
            .repo_repo
            .expect_list_latest()
            .returning(|_| Ok(vec![public_repo(), public_repo()]));

        let resp = service.list_latest_repositories().await.unwrap();
        assert_eq!(resp.len(), 2);
    }

    #[tokio::test]
    async fn list_trending_repositories_maps_responses() {
        let mut service = create_service();
        service
            .repo_repo
            .expect_list_trending()
            .returning(|_| Ok(vec![public_repo()]));

        let resp = service.list_trending_repositories().await.unwrap();
        assert_eq!(resp.len(), 1);
    }

    // --- star / unstar ---

    #[tokio::test]
    async fn star_repository_succeeds() {
        let mut service = create_service();
        service
            .repo_repo
            .expect_get_id()
            .returning(|_, _| Ok(Some(Uuid::new_v4())));
        service.repo_repo.expect_star().returning(|id, user_id| {
            Ok(Some(RepositoryStar {
                id: Uuid::new_v4(),
                user_id,
                repository_id: id,
                created_at: Utc::now(),
            }))
        });

        let req = StarRepositoryRequest::new(Uuid::new_v4(), "alice", "myrepo").unwrap();
        service.star_repository(req).await.unwrap();
    }

    #[tokio::test]
    async fn star_repository_already_starred_is_conflict() {
        let mut service = create_service();
        service
            .repo_repo
            .expect_get_id()
            .returning(|_, _| Ok(Some(Uuid::new_v4())));
        // A null star insert means the user had already starred the repo.
        service.repo_repo.expect_star().returning(|_, _| Ok(None));

        let req = StarRepositoryRequest::new(Uuid::new_v4(), "alice", "myrepo").unwrap();
        let err = service.star_repository(req).await.unwrap_err();
        assert!(matches!(err, RepositoryError::Conflict(_)));
    }

    #[tokio::test]
    async fn star_repository_missing_repo_is_not_found() {
        let mut service = create_service();
        service.repo_repo.expect_get_id().returning(|_, _| Ok(None));

        let req = StarRepositoryRequest::new(Uuid::new_v4(), "alice", "ghost").unwrap();
        let err = service.star_repository(req).await.unwrap_err();
        assert!(matches!(err, RepositoryError::NotFound(_)));
    }

    #[tokio::test]
    async fn unstar_repository_no_star_is_conflict() {
        let mut service = create_service();
        service
            .repo_repo
            .expect_get_id()
            .returning(|_, _| Ok(Some(Uuid::new_v4())));
        service
            .repo_repo
            .expect_unstar()
            .returning(|_, _| Ok(false));

        let req = UnstarRepositoryRequest::new(Uuid::new_v4(), "alice", "myrepo").unwrap();
        let err = service.unstar_repository(req).await.unwrap_err();
        assert!(matches!(err, RepositoryError::Conflict(_)));
    }

    // --- activity ---

    #[tokio::test]
    async fn get_repository_activity_maps_recent_stars() {
        let mut service = create_service();
        service
            .repo_repo
            .expect_get()
            .returning(|_, _, _| Ok(Some(public_repo())));
        service
            .repo_repo
            .expect_list_recent_stars()
            .returning(|_, _| Ok(vec![(create_user("bob"), Utc::now())]));

        let req = GetRepositoryActivityRequest::new("alice", "myrepo").unwrap();
        let events = service.get_repository_activity(req).await.unwrap();
        assert_eq!(events.len(), 1);
    }

    // --- commit filters ---

    #[tokio::test]
    async fn list_repository_commit_filters_maps_page() {
        let mut service = create_service();
        service
            .repo_repo
            .expect_get()
            .returning(|_, _, _| Ok(Some(public_repo())));
        service
            .repo_repo
            .expect_list_commit_filters()
            .returning(|_, _, _| Ok((vec![create_commit_filter("recent")], None)));

        let req =
            ListRepositoryCommitFiltersRequest::new(None, "alice", "myrepo", None, None).unwrap();
        let page = service.list_repository_commit_filters(req).await.unwrap();
        assert_eq!(page.data.len(), 1);
    }

    #[tokio::test]
    async fn create_repository_commit_filter_succeeds() {
        let mut service = create_service();
        service
            .repo_repo
            .expect_get()
            .returning(|_, _, _| Ok(Some(public_repo())));
        service
            .repo_repo
            .expect_list_commit_filters()
            .returning(|_, _, _| Ok((vec![], None)));
        service
            .repo_repo
            .expect_create_commit_filter()
            .returning(|_, name, _, _, _| Ok(create_commit_filter(name)));

        let req = CreateRepositoryCommitFilterRequest::new(
            Uuid::new_v4(),
            "alice",
            "myrepo",
            "recent",
            None,
            None,
            None,
        )
        .unwrap();
        let resp = service.create_repository_commit_filter(req).await.unwrap();
        assert_eq!(resp.name, "recent");
    }

    #[tokio::test]
    async fn create_repository_commit_filter_duplicate_name_is_conflict() {
        let mut service = create_service();
        service
            .repo_repo
            .expect_get()
            .returning(|_, _, _| Ok(Some(public_repo())));
        service
            .repo_repo
            .expect_list_commit_filters()
            .returning(|_, _, _| Ok((vec![create_commit_filter("recent")], None)));

        let req = CreateRepositoryCommitFilterRequest::new(
            Uuid::new_v4(),
            "alice",
            "myrepo",
            "recent",
            None,
            None,
            None,
        )
        .unwrap();
        let err = service
            .create_repository_commit_filter(req)
            .await
            .unwrap_err();
        assert!(matches!(err, RepositoryError::Conflict(_)));
    }

    #[tokio::test]
    async fn update_repository_commit_filter_missing_is_not_found() {
        let mut service = create_service();
        service
            .repo_repo
            .expect_get()
            .returning(|_, _, _| Ok(Some(public_repo())));
        service
            .repo_repo
            .expect_update_commit_filter()
            .returning(|_, _, _, _, _, _| Ok(None));

        let req = UpdateRepositoryCommitFilterRequest::new(
            "alice",
            "myrepo",
            Uuid::new_v4(),
            "recent",
            None,
            None,
            None,
        )
        .unwrap();
        let err = service
            .update_repository_commit_filter(req)
            .await
            .unwrap_err();
        assert!(matches!(err, RepositoryError::NotFound(_)));
    }

    #[tokio::test]
    async fn delete_repository_commit_filter_missing_is_not_found() {
        let mut service = create_service();
        service
            .repo_repo
            .expect_get()
            .returning(|_, _, _| Ok(Some(public_repo())));
        service
            .repo_repo
            .expect_delete_commit_filter()
            .returning(|_, _| Ok(false));

        let req =
            DeleteRepositoryCommitFilterRequest::new("alice", "myrepo", Uuid::new_v4()).unwrap();
        let err = service
            .delete_repository_commit_filter(req)
            .await
            .unwrap_err();
        assert!(matches!(err, RepositoryError::NotFound(_)));
    }
}
