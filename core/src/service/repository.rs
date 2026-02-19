use std::collections::{HashMap, HashSet};

use async_trait::async_trait;

use crate::{
    client::{DiffClient, DifftClient, Git2Client, GitClient},
    dto::{
        CommitAuthorResponse, CreateRepositoryRequest, DeleteRepositoryRequest,
        GetRepositoryCommitDiffRequest, GetRepositoryCommitRequest, GetRepositoryCommitStatRequest,
        GetRepositoryCommitsRequest, GetRepositoryFileCommitsRequest, GetRepositoryFileRequest,
        GetRepositoryPreviewRequest, GetRepositoryTreeRequest, RepositoryCommitDiffResponse,
        RepositoryCommitResponse, RepositoryCommitStatResponse, RepositoryCommitsResponse,
        RepositoryFileResponse, RepositoryPreviewResponse, RepositoryResponse,
        RepositoryTreeResponse,
    },
    error::RepositoryError,
    model::RepositoryOwnerType,
    repository::{
        OrganizationRepository, OrganizationRepositoryImpl, RepositoryRepository,
        RepositoryRepositoryImpl, UserRepository, UserRepositoryImpl,
    },
    util::git::{GitHookType, POST_RECEIVE_SCRIPT},
};

#[async_trait]
pub trait RepositoryService: Send + Sync + 'static {
    async fn create_repository(
        &self,
        request: CreateRepositoryRequest,
    ) -> Result<RepositoryResponse, RepositoryError>;

    async fn get_repository_tree(
        &self,
        request: GetRepositoryTreeRequest,
    ) -> Result<RepositoryTreeResponse, RepositoryError>;

    async fn get_repository_file(
        &self,
        request: GetRepositoryFileRequest,
    ) -> Result<RepositoryFileResponse, RepositoryError>;

    async fn get_repository_commit(
        &self,
        request: GetRepositoryCommitRequest,
    ) -> Result<RepositoryCommitResponse, RepositoryError>;

    async fn get_repository_commits(
        &self,
        request: GetRepositoryCommitsRequest,
    ) -> Result<RepositoryCommitsResponse, RepositoryError>;

    async fn get_repository_file_commits(
        &self,
        request: GetRepositoryFileCommitsRequest,
    ) -> Result<RepositoryCommitsResponse, RepositoryError>;

    async fn get_repository_preview(
        &self,
        request: GetRepositoryPreviewRequest,
    ) -> Result<RepositoryPreviewResponse, RepositoryError>;

    async fn get_repository_commit_diff(
        &self,
        request: GetRepositoryCommitDiffRequest,
    ) -> Result<Vec<RepositoryCommitDiffResponse>, RepositoryError>;

    async fn get_repository_commit_stat(
        &self,
        request: GetRepositoryCommitStatRequest,
    ) -> Result<Vec<RepositoryCommitStatResponse>, RepositoryError>;

    async fn delete_repository(
        &self,
        request: DeleteRepositoryRequest,
    ) -> Result<(), RepositoryError>;
}

#[derive(Debug, Clone)]
pub struct RepositoryServiceImpl<G, D, O, R, U>
where
    G: GitClient,
    D: DiffClient,
    O: OrganizationRepository,
    R: RepositoryRepository,
    U: UserRepository,
{
    git_client: G,
    diff_client: D,
    org_repo: O,
    repo_repo: R,
    user_repo: U,
}

impl
    RepositoryServiceImpl<
        Git2Client,
        DifftClient,
        OrganizationRepositoryImpl,
        RepositoryRepositoryImpl,
        UserRepositoryImpl,
    >
{
    pub fn new(
        git_client: Git2Client,
        diff_client: DifftClient,
        org_repo: OrganizationRepositoryImpl,
        repo_repo: RepositoryRepositoryImpl,
        user_repo: UserRepositoryImpl,
    ) -> Self {
        Self {
            git_client,
            diff_client,
            org_repo,
            repo_repo,
            user_repo,
        }
    }
}

impl<G, D, O, R, U> RepositoryServiceImpl<G, D, O, R, U>
where
    G: GitClient,
    D: DiffClient,
    O: OrganizationRepository,
    R: RepositoryRepository,
    U: UserRepository,
{
    async fn enrich_commits_with_users(
        &self,
        commits: &mut [RepositoryCommitResponse],
    ) -> Result<(), RepositoryError> {
        let emails: Vec<_> = commits
            .iter()
            .map(|c| c.author.email.clone())
            .filter(|e| !e.is_empty())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();

        if emails.is_empty() {
            return Ok(());
        }

        let users = self.user_repo.get_by_emails(&emails).await?;
        let email_to_user: HashMap<_, _> = users.iter().map(|u| (u.email.as_str(), u)).collect();
        for commit in commits {
            if let Some(user) = email_to_user.get(commit.author.email.as_str()) {
                commit.author = CommitAuthorResponse {
                    id: Some(user.id),
                    name: user.name.clone(),
                    email: commit.author.email.clone(),
                };
            }
        }

        Ok(())
    }
}

#[async_trait]
impl<G, D, O, R, U> RepositoryService for RepositoryServiceImpl<G, D, O, R, U>
where
    G: GitClient,
    D: DiffClient,
    O: OrganizationRepository,
    R: RepositoryRepository,
    U: UserRepository,
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
            return Err(RepositoryError::Duplicate(repo_name));
        }

        let owner_id = match request.owner_type {
            RepositoryOwnerType::User => request.user_id,
            RepositoryOwnerType::Organization => {
                let org = self
                    .org_repo
                    .get(&request.owner_name)
                    .await?
                    .ok_or_else(|| {
                        RepositoryError::OwnerNotFound(request.owner_name.to_string())
                    })?;
                org.id
            }
        };

        // Create git repo first
        self.git_client
            .create_repo(&request.owner_name, &repo_name)
            .await?;

        // Install post-receive hook to create commit rows
        self.git_client
            .install_hook(
                &request.owner_name,
                &repo_name,
                GitHookType::PostReceive,
                POST_RECEIVE_SCRIPT,
            )
            .await?;

        // Insert into DB, delete git repo on failure
        let repository = match self
            .repo_repo
            .create(
                &repo_name,
                owner_id,
                &request.owner_name,
                &request.owner_type,
                &request.visibility,
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

    async fn get_repository_tree(
        &self,
        request: GetRepositoryTreeRequest,
    ) -> Result<RepositoryTreeResponse, RepositoryError> {
        let mut response = self
            .git_client
            .get_repo_tree(&request.owner_name, &request.name, &request.ref_name)
            .await?;

        let mut commits: Vec<RepositoryCommitResponse> =
            response.entries.iter().map(|e| e.commit.clone()).collect();
        self.enrich_commits_with_users(&mut commits).await?;

        for (entry, enriched_commit) in response.entries.iter_mut().zip(commits.into_iter()) {
            entry.commit = enriched_commit;
        }

        Ok(response)
    }

    async fn get_repository_file(
        &self,
        request: GetRepositoryFileRequest,
    ) -> Result<RepositoryFileResponse, RepositoryError> {
        self.git_client
            .get_repo_file(
                &request.owner_name,
                &request.name,
                &request.ref_name,
                &request.path,
            )
            .await
            .map_err(|e| e.into())
    }

    async fn get_repository_commit(
        &self,
        request: GetRepositoryCommitRequest,
    ) -> Result<RepositoryCommitResponse, RepositoryError> {
        let commit = self
            .git_client
            .get_repo_commit(&request.owner_name, &request.name, &request.ref_name)
            .await
            .map_err(RepositoryError::from)?;

        let mut commits = [commit];
        self.enrich_commits_with_users(&mut commits).await?;

        Ok(commits.into_iter().next().unwrap())
    }

    async fn get_repository_commits(
        &self,
        request: GetRepositoryCommitsRequest,
    ) -> Result<RepositoryCommitsResponse, RepositoryError> {
        let mut response = self
            .git_client
            .get_repo_commits(
                &request.owner_name,
                &request.name,
                &request.ref_name,
                request.page,
                request.per_page,
            )
            .await?;

        self.enrich_commits_with_users(&mut response.commits)
            .await?;

        Ok(response)
    }

    async fn get_repository_file_commits(
        &self,
        request: GetRepositoryFileCommitsRequest,
    ) -> Result<RepositoryCommitsResponse, RepositoryError> {
        let mut response = self
            .git_client
            .get_repo_file_commits(
                &request.owner_name,
                &request.name,
                &request.ref_name,
                &request.path,
                request.page,
                request.per_page,
            )
            .await?;

        self.enrich_commits_with_users(&mut response.commits)
            .await?;

        Ok(response)
    }

    async fn get_repository_preview(
        &self,
        request: GetRepositoryPreviewRequest,
    ) -> Result<RepositoryPreviewResponse, RepositoryError> {
        self.git_client
            .get_repo_preview(
                &request.owner_name,
                &request.name,
                &request.ref_name,
                request.preview_lines,
            )
            .await
            .map_err(|e| e.into())
    }

    async fn get_repository_commit_diff(
        &self,
        request: GetRepositoryCommitDiffRequest,
    ) -> Result<Vec<RepositoryCommitDiffResponse>, RepositoryError> {
        let commit = self
            .git_client
            .get_repo_commit(&request.owner_name, &request.name, &request.ref_name)
            .await
            .map_err(RepositoryError::from)?;

        let parent_sha = commit
            .parent_sha
            .unwrap_or(crate::util::git::EMPTY_TREE_REF.to_string());

        let file_pairs = self
            .git_client
            .get_repo_diff_files(
                &request.owner_name,
                &request.name,
                &parent_sha,
                &request.ref_name,
            )
            .await?;

        let handles: Vec<_> = file_pairs
            .into_iter()
            .map(|(left, right)| {
                let diff_client = self.diff_client.clone();
                tokio::task::spawn_blocking(move || {
                    let diff = diff_client.diff_files(left.as_ref(), right.as_ref())?;
                    Ok::<_, RepositoryError>(RepositoryCommitDiffResponse { diff, left, right })
                })
            })
            .collect();

        let mut diffs = Vec::with_capacity(handles.len());
        for handle in handles {
            let result = handle.await.map_err(|e| {
                RepositoryError::DiffError(crate::error::DiffError::DifftasticFailed(e.to_string()))
            })?;
            diffs.push(result?);
        }

        Ok(diffs)
    }

    async fn get_repository_commit_stat(
        &self,
        request: GetRepositoryCommitStatRequest,
    ) -> Result<Vec<RepositoryCommitStatResponse>, RepositoryError> {
        let commit = self
            .git_client
            .get_repo_commit(&request.owner_name, &request.name, &request.ref_name)
            .await
            .map_err(RepositoryError::from)?;

        let parent_sha = commit
            .parent_sha
            .unwrap_or(crate::util::git::EMPTY_TREE_REF.to_string());

        let stats = self
            .git_client
            .get_repo_diff_stats(
                &request.owner_name,
                &request.name,
                &parent_sha,
                &request.ref_name,
            )
            .await?;

        Ok(stats)
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
            .ok_or_else(|| RepositoryError::NotFound(format!("{}/{}", owner, repo)))?;

        self.git_client.delete_repo(owner, repo).await?;
        self.repo_repo.delete(repository.id).await?;

        Ok(())
    }
}
