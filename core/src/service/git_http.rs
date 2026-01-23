use async_trait::async_trait;

use crate::client::{GitHttpBackendClient, GitHttpBackendClientImpl};
use crate::dto::{GitHttpBackendResponse, InfoRefsRequest, ReceivePackRequest, UploadPackRequest};
use crate::error::GitHttpBackendError;

#[async_trait]
pub trait GitHttpBackendService: Send + Sync + 'static {
    async fn info_refs(
        &self,
        request: InfoRefsRequest,
    ) -> Result<GitHttpBackendResponse, GitHttpBackendError>;

    async fn upload_pack(
        &self,
        request: UploadPackRequest,
    ) -> Result<GitHttpBackendResponse, GitHttpBackendError>;

    async fn receive_pack(
        &self,
        request: ReceivePackRequest,
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
        request: InfoRefsRequest,
    ) -> Result<GitHttpBackendResponse, GitHttpBackendError> {
        self.git_http_client
            .info_refs(&request.owner, &request.repo, &request.service)
            .await
    }

    async fn upload_pack(
        &self,
        request: UploadPackRequest,
    ) -> Result<GitHttpBackendResponse, GitHttpBackendError> {
        self.git_http_client
            .service_rpc(
                &request.owner,
                &request.repo,
                "upload-pack",
                &request.content_type,
                &request.body,
            )
            .await
    }

    async fn receive_pack(
        &self,
        request: ReceivePackRequest,
    ) -> Result<GitHttpBackendResponse, GitHttpBackendError> {
        // TODO: After successful receive-pack, we can parse the pack data
        // to extract commit information and persist it in DB (e.g. via CommitRepository)
        self.git_http_client
            .service_rpc(
                &request.owner,
                &request.repo,
                "receive-pack",
                &request.content_type,
                &request.body,
            )
            .await
    }
}
