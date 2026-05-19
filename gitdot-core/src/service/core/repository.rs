use std::collections::HashMap;

use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    client::{Git2Client, GitClient},
    dto::{
        CommitDiffResponse, CommitResponse, CommitsResponse, CreateRepositoryCommitFilterRequest,
        CreateRepositoryRequest, DeleteRepositoryCommitFilterRequest, DeleteRepositoryRequest,
        GetRepositoryActivityRequest, GetRepositoryBlobDiffsRequest, GetRepositoryBlobRequest,
        GetRepositoryBlobsRequest, GetRepositoryCommitDiffRequest, GetRepositoryCommitRequest,
        GetRepositoryPathsRequest, GetRepositoryRequest, ListRepositoryCommitFiltersRequest,
        ListRepositoryCommitsRequest, MAX_PER_PAGE_LIMIT, Page, RepositoryActivityEvent,
        RepositoryBlobDiffsResponse, RepositoryBlobResponse, RepositoryBlobsResponse,
        RepositoryCommitFilterResponse, RepositoryDiffFileResponse, RepositoryFileResponse,
        RepositoryPathsResponse, RepositoryResponse, StarRepositoryRequest,
        UnstarRepositoryRequest, UpdateRepositoryCommitFilterRequest,
    },
    error::{ConflictError, NotFoundError, OptionNotFoundExt, RepositoryError},
    model::RepositoryOwnerType,
    repository::{
        CommitRepository, CommitRepositoryImpl, OrganizationRepository, OrganizationRepositoryImpl,
        RepositoryRepository, RepositoryRepositoryImpl,
    },
    util::{
        cursor,
        git::{GitHookType, POST_RECEIVE_SCRIPT, PRE_RECEIVE_SCRIPT, PROC_RECEIVE_SCRIPT},
    },
};

#[async_trait]
pub trait RepositoryService: Send + Sync + 'static {
    async fn create_repository(
        &self,
        request: CreateRepositoryRequest,
    ) -> Result<RepositoryResponse, RepositoryError>;

    async fn get_repository(
        &self,
        request: GetRepositoryRequest,
    ) -> Result<RepositoryResponse, RepositoryError>;

    async fn get_repository_blob(
        &self,
        request: GetRepositoryBlobRequest,
    ) -> Result<RepositoryBlobResponse, RepositoryError>;

    async fn get_repository_blobs(
        &self,
        request: GetRepositoryBlobsRequest,
    ) -> Result<RepositoryBlobsResponse, RepositoryError>;

    async fn get_repository_paths(
        &self,
        request: GetRepositoryPathsRequest,
    ) -> Result<RepositoryPathsResponse, RepositoryError>;

    async fn get_repository_by_id(&self, id: Uuid) -> Result<RepositoryResponse, RepositoryError>;

    async fn delete_repository(
        &self,
        request: DeleteRepositoryRequest,
    ) -> Result<(), RepositoryError>;

    async fn resolve_ref_sha(
        &self,
        owner: &str,
        repo: &str,
        ref_name: &str,
    ) -> Result<String, RepositoryError>;

    async fn get_repository_blob_diffs(
        &self,
        request: GetRepositoryBlobDiffsRequest,
    ) -> Result<RepositoryBlobDiffsResponse, RepositoryError>;

    async fn get_repository_commit(
        &self,
        request: GetRepositoryCommitRequest,
    ) -> Result<CommitResponse, RepositoryError>;

    async fn get_repository_commit_diff(
        &self,
        request: GetRepositoryCommitDiffRequest,
    ) -> Result<CommitDiffResponse, RepositoryError>;

    async fn list_repository_commits(
        &self,
        request: ListRepositoryCommitsRequest,
    ) -> Result<CommitsResponse, RepositoryError>;

    async fn star_repository(&self, request: StarRepositoryRequest) -> Result<(), RepositoryError>;

    async fn unstar_repository(
        &self,
        request: UnstarRepositoryRequest,
    ) -> Result<(), RepositoryError>;

    async fn get_repository_activity(
        &self,
        request: GetRepositoryActivityRequest,
    ) -> Result<Vec<RepositoryActivityEvent>, RepositoryError>;

    async fn list_repository_commit_filters(
        &self,
        request: ListRepositoryCommitFiltersRequest,
    ) -> Result<Page<RepositoryCommitFilterResponse>, RepositoryError>;

    async fn create_repository_commit_filter(
        &self,
        request: CreateRepositoryCommitFilterRequest,
    ) -> Result<RepositoryCommitFilterResponse, RepositoryError>;

    async fn update_repository_commit_filter(
        &self,
        request: UpdateRepositoryCommitFilterRequest,
    ) -> Result<RepositoryCommitFilterResponse, RepositoryError>;

    async fn delete_repository_commit_filter(
        &self,
        request: DeleteRepositoryCommitFilterRequest,
    ) -> Result<(), RepositoryError>;
}

#[derive(Debug, Clone)]
pub struct RepositoryServiceImpl<G, O, R, C>
where
    G: GitClient,
    O: OrganizationRepository,
    R: RepositoryRepository,
    C: CommitRepository,
{
    git_client: G,
    org_repo: O,
    repo_repo: R,
    commit_repo: C,
}

impl
    RepositoryServiceImpl<
        Git2Client,
        OrganizationRepositoryImpl,
        RepositoryRepositoryImpl,
        CommitRepositoryImpl,
    >
{
    pub fn new(
        git_client: Git2Client,
        org_repo: OrganizationRepositoryImpl,
        repo_repo: RepositoryRepositoryImpl,
        commit_repo: CommitRepositoryImpl,
    ) -> Self {
        Self {
            git_client,
            org_repo,
            repo_repo,
            commit_repo,
        }
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl<G, O, R, C> RepositoryService for RepositoryServiceImpl<G, O, R, C>
where
    G: GitClient,
    O: OrganizationRepository,
    R: RepositoryRepository,
    C: CommitRepository,
{
    async fn create_repository(
        &self,
        request: CreateRepositoryRequest,
    ) -> Result<RepositoryResponse, RepositoryError> {
        let repo_name = request.name.to_string();
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

        // Create git repo first
        self.git_client
            .create_repo(&request.owner_name, &repo_name)
            .await?;

        // Install gitdot hooks
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

        // Insert into DB, delete git repo on failure
        let repository = match self
            .repo_repo
            .create(
                &repo_name,
                owner_id,
                &request.owner_type,
                &request.visibility,
                request.description,
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
            .get(owner, repo)
            .await?
            .or_not_found("repository", format!("{}/{}", owner, repo))?;

        let user_star = match request.user_id {
            Some(uid) => self.repo_repo.is_starred(repository.id, uid).await?,
            None => false,
        };

        Ok(RepositoryResponse::from(repository).with_user_star(user_star))
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
        if request.refs.len() > 1 {
            self.git_client
                .get_repo_blob_at_refs(
                    &request.owner_name,
                    &request.name,
                    &request.paths[0],
                    &request.refs,
                )
                .await
                .map_err(Into::into)
        } else {
            self.git_client
                .get_repo_blobs(
                    &request.owner_name,
                    &request.name,
                    &request.refs[0],
                    &request.paths,
                )
                .await
                .map_err(Into::into)
        }
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
            .get_by_id(id)
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
            .get(owner, repo)
            .await?
            .or_not_found("repository", format!("{}/{}", owner, repo))?;

        self.git_client.delete_repo(owner, repo).await?;
        self.repo_repo.delete(repository.id).await?;

        Ok(())
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

    async fn get_repository_blob_diffs(
        &self,
        request: GetRepositoryBlobDiffsRequest,
    ) -> Result<RepositoryBlobDiffsResponse, RepositoryError> {
        let blobs = self
            .git_client
            .get_repo_blob_at_refs(
                &request.owner_name,
                &request.name,
                &request.path,
                &request.commit_shas,
            )
            .await
            .map_err(RepositoryError::from)?;

        let mut files = Vec::new();
        for blob in blobs.blobs {
            match blob {
                RepositoryBlobResponse::File(f) => files.push(f),
                RepositoryBlobResponse::Folder(_) => {
                    return Err(RepositoryError::NotAFile(request.path.clone()));
                }
            }
        }

        let mut diffs = HashMap::new();
        for (i, ref_name) in request.commit_shas.iter().enumerate() {
            let diff = diff_file_pair(files.get(i + 1), files.get(i));
            diffs.insert(ref_name.clone(), diff);
        }

        Ok(RepositoryBlobDiffsResponse { diffs })
    }

    async fn get_repository_commit(
        &self,
        request: GetRepositoryCommitRequest,
    ) -> Result<CommitResponse, RepositoryError> {
        let owner = request.owner.to_string();
        let repo_name = request.repo.to_string();

        let repository = self
            .repo_repo
            .get(&owner, &repo_name)
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

    async fn get_repository_commit_diff(
        &self,
        request: GetRepositoryCommitDiffRequest,
    ) -> Result<CommitDiffResponse, RepositoryError> {
        let owner = request.owner.to_string();
        let repo_name = request.repo.to_string();

        let repository = self
            .repo_repo
            .get(&owner, &repo_name)
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
        let left_ref = if is_initial {
            None
        } else {
            Some(parent_sha.as_str())
        };

        let files = self
            .git_client
            .get_repo_diff_files(&owner, &repo_name, left_ref, &sha)
            .await?;

        Ok(CommitDiffResponse {
            sha,
            parent_sha,
            files,
        })
    }

    async fn list_repository_commits(
        &self,
        request: ListRepositoryCommitsRequest,
    ) -> Result<CommitsResponse, RepositoryError> {
        let owner = request.owner.to_string();
        let repo_name = request.repo.to_string();

        let repository = self
            .repo_repo
            .get(&owner, &repo_name)
            .await?
            .or_not_found("repository", format!("{}/{}", owner, repo_name))?;

        let commits = self
            .commit_repo
            .get_commits(repository.id, request.from, request.to)
            .await?;

        Ok(CommitsResponse {
            commits: commits.into_iter().map(Into::into).collect(),
        })
    }

    async fn star_repository(&self, request: StarRepositoryRequest) -> Result<(), RepositoryError> {
        let owner = request.owner.as_ref();
        let repo = request.repo.as_ref();

        let repository = self
            .repo_repo
            .get(owner, repo)
            .await?
            .or_not_found("repository", format!("{}/{}", owner, repo))?;

        let star = self.repo_repo.star(repository.id, request.user_id).await?;
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

        let repository = self
            .repo_repo
            .get(owner, repo)
            .await?
            .or_not_found("repository", format!("{}/{}", owner, repo))?;

        let removed = self
            .repo_repo
            .unstar(repository.id, request.user_id)
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
            .get(owner, repo)
            .await?
            .or_not_found("repository", format!("{}/{}", owner, repo))?;

        let stars = self
            .repo_repo
            .list_recent_stars(repository.id, ACTIVITY_EVENT_LIMIT)
            .await?;

        Ok(stars
            .into_iter()
            .map(|(user, at)| RepositoryActivityEvent::Starred { user, at })
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
            .get(owner, repo)
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
            .get(owner, repo)
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
        let filter = self
            .repo_repo
            .update_commit_filter(
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
        let deleted = self
            .repo_repo
            .delete_commit_filter(request.filter_id)
            .await?;
        if !deleted {
            return Err(NotFoundError::new("commit filter", request.filter_id).into());
        }
        Ok(())
    }
}

fn diff_file_pair(
    left: Option<&RepositoryFileResponse>,
    right: Option<&RepositoryFileResponse>,
) -> RepositoryDiffFileResponse {
    let path = right
        .map(|r| r.path.clone())
        .or_else(|| left.map(|l| l.path.clone()))
        .unwrap_or_default();

    let left_bytes = left.map(|l| l.content.as_bytes());
    let right_bytes = right.map(|r| r.content.as_bytes());

    let (lines_added, lines_removed) = match git2::Patch::from_buffers(
        left_bytes.unwrap_or(&[]),
        None,
        right_bytes.unwrap_or(&[]),
        None,
        None,
    ) {
        Ok(patch) => patch
            .line_stats()
            .map(|(_, ins, del)| (ins as u32, del as u32))
            .unwrap_or((0, 0)),
        Err(_) => (0, 0),
    };

    RepositoryDiffFileResponse {
        path,
        left_content: left.map(|l| l.content.clone()),
        right_content: right.map(|r| r.content.clone()),
        lines_added,
        lines_removed,
    }
}
