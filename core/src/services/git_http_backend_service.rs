use async_trait::async_trait;

use crate::clients::{GitHttpBackendClient, GitHttpBackendClientImpl};
use crate::dto::GitHttpBackendResponse;
use crate::errors::GitHttpBackendError;

#[async_trait]
pub trait GitHttpBackendService: Send + Sync + 'static {
    async fn info_refs(
        &self,
        owner: &str,
        repo: &str,
        service: &str,
    ) -> Result<GitHttpBackendResponse, GitHttpBackendError>;

    async fn upload_pack(
        &self,
        owner: &str,
        repo: &str,
        content_type: &str,
        body: &[u8],
    ) -> Result<GitHttpBackendResponse, GitHttpBackendError>;

    async fn receive_pack(
        &self,
        owner: &str,
        repo: &str,
        content_type: &str,
        body: &[u8],
    ) -> Result<GitHttpBackendResponse, GitHttpBackendError>;
}

#[derive(Debug, Clone)]
pub struct GitHttpBackendServiceImpl<G>
where
    G: GitHttpBackendClient,
{
    git_http_client: G,
}

impl GitHttpBackendServiceImpl<GitHttpBackendClientImpl> {
    pub fn new(git_http_client: GitHttpBackendClientImpl) -> Self {
        Self { git_http_client }
    }
}

#[async_trait]
impl<G> GitHttpBackendService for GitHttpBackendServiceImpl<G>
where
    G: GitHttpBackendClient,
{
    async fn info_refs(
        &self,
        owner: &str,
        repo: &str,
        service: &str,
    ) -> Result<GitHttpBackendResponse, GitHttpBackendError> {
        self.git_http_client.info_refs(owner, repo, service).await
    }

    async fn upload_pack(
        &self,
        owner: &str,
        repo: &str,
        content_type: &str,
        body: &[u8],
    ) -> Result<GitHttpBackendResponse, GitHttpBackendError> {
        self.git_http_client
            .service_rpc(owner, repo, "upload-pack", content_type, body)
            .await
    }

    async fn receive_pack(
        &self,
        owner: &str,
        repo: &str,
        content_type: &str,
        body: &[u8],
    ) -> Result<GitHttpBackendResponse, GitHttpBackendError> {
        // TODO: After successful receive-pack, we can parse the pack data
        // to extract commit information and persist it in DB (e.g. via CommitRepository)
        self.git_http_client
            .service_rpc(owner, repo, "receive-pack", content_type, body)
            .await
    }
}
