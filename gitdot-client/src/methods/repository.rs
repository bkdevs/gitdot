use anyhow::Result;

use gitdot_api::endpoint::repository::{
    create_repository::{CreateRepositoryRequest, CreateRepositoryResponse},
    delete_repository::{DeleteRepositoryRequest, DeleteRepositoryResponse},
    get_repository_commit::{GetRepositoryCommitRequest, GetRepositoryCommitResponse},
    get_repository_commit_diff::{GetRepositoryCommitDiffRequest, GetRepositoryCommitDiffResponse},
    get_repository_commit_stat::{GetRepositoryCommitStatRequest, GetRepositoryCommitStatResponse},
    get_repository_commits::{GetRepositoryCommitsRequest, GetRepositoryCommitsResponse},
    get_repository_file::{GetRepositoryFileRequest, GetRepositoryFileResponse},
    get_repository_file_commits::{
        GetRepositoryFileCommitsRequest, GetRepositoryFileCommitsResponse,
    },
    get_repository_preview::{GetRepositoryPreviewRequest, GetRepositoryPreviewResponse},
    get_repository_tree::{GetRepositoryTreeRequest, GetRepositoryTreeResponse},
};

use crate::client::GitdotClient;

impl GitdotClient {
    pub async fn create_repository(
        &self,
        owner: &str,
        repo: &str,
        request: CreateRepositoryRequest,
    ) -> Result<CreateRepositoryResponse> {
        self.post(format!("repository/{}/{}", owner, repo), request)
            .await
    }

    pub async fn delete_repository(
        &self,
        owner: &str,
        repo: &str,
        request: DeleteRepositoryRequest,
    ) -> Result<DeleteRepositoryResponse> {
        self.delete(format!("repository/{}/{}", owner, repo), request)
            .await
    }

    pub async fn get_repository_file(
        &self,
        owner: &str,
        repo: &str,
        request: GetRepositoryFileRequest,
    ) -> Result<GetRepositoryFileResponse> {
        self.get(format!("repository/{}/{}/file", owner, repo), request)
            .await
    }

    pub async fn get_repository_commits(
        &self,
        owner: &str,
        repo: &str,
        request: GetRepositoryCommitsRequest,
    ) -> Result<GetRepositoryCommitsResponse> {
        self.get(format!("repository/{}/{}/commits", owner, repo), request)
            .await
    }

    pub async fn get_repository_commit(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
        request: GetRepositoryCommitRequest,
    ) -> Result<GetRepositoryCommitResponse> {
        self.get(
            format!("repository/{}/{}/commits/{}", owner, repo, sha),
            request,
        )
        .await
    }

    pub async fn get_repository_commit_stat(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
        request: GetRepositoryCommitStatRequest,
    ) -> Result<GetRepositoryCommitStatResponse> {
        self.get(
            format!("repository/{}/{}/commits/{}/stat", owner, repo, sha),
            request,
        )
        .await
    }

    pub async fn get_repository_commit_diff(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
        request: GetRepositoryCommitDiffRequest,
    ) -> Result<GetRepositoryCommitDiffResponse> {
        self.get(
            format!("repository/{}/{}/commits/{}/diff", owner, repo, sha),
            request,
        )
        .await
    }

    pub async fn get_repository_file_commits(
        &self,
        owner: &str,
        repo: &str,
        request: GetRepositoryFileCommitsRequest,
    ) -> Result<GetRepositoryFileCommitsResponse> {
        self.get(
            format!("repository/{}/{}/file/commits", owner, repo),
            request,
        )
        .await
    }

    pub async fn get_repository_tree(
        &self,
        owner: &str,
        repo: &str,
        request: GetRepositoryTreeRequest,
    ) -> Result<GetRepositoryTreeResponse> {
        self.get(format!("repository/{}/{}/tree", owner, repo), request)
            .await
    }

    pub async fn get_repository_preview(
        &self,
        owner: &str,
        repo: &str,
        request: GetRepositoryPreviewRequest,
    ) -> Result<GetRepositoryPreviewResponse> {
        self.get(format!("repository/{}/{}/preview", owner, repo), request)
            .await
    }
}
