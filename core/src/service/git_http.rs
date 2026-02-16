use async_trait::async_trait;

use crate::{
    client::{GitHttpClient, GitHttpClientImpl},
    dto::{GitHttpResponse, InfoRefsRequest, ReceivePackRequest, UploadPackRequest},
    error::GitHttpError,
};

#[async_trait]
pub trait GitHttpService: Send + Sync + 'static {
    async fn info_refs(&self, request: InfoRefsRequest) -> Result<GitHttpResponse, GitHttpError>;

    async fn upload_pack(
        &self,
        request: UploadPackRequest,
    ) -> Result<GitHttpResponse, GitHttpError>;

    async fn receive_pack(
        &self,
        request: ReceivePackRequest,
    ) -> Result<GitHttpResponse, GitHttpError>;
}

#[derive(Debug, Clone)]
pub struct GitHttpServiceImpl<G>
where
    G: GitHttpClient,
{
    git_http_client: G,
}

impl GitHttpServiceImpl<GitHttpClientImpl> {
    pub fn new(git_http_client: GitHttpClientImpl) -> Self {
        Self { git_http_client }
    }
}

#[async_trait]
impl<G> GitHttpService for GitHttpServiceImpl<G>
where
    G: GitHttpClient,
{
    async fn info_refs(&self, request: InfoRefsRequest) -> Result<GitHttpResponse, GitHttpError> {
        self.git_http_client
            .info_refs(&request.owner, &request.repo, &request.service)
            .await
    }

    async fn upload_pack(
        &self,
        request: UploadPackRequest,
    ) -> Result<GitHttpResponse, GitHttpError> {
        self.git_http_client
            .service_rpc(
                &request.owner,
                &request.repo,
                "upload-pack",
                &request.content_type,
                request.body,
            )
            .await
    }

    async fn receive_pack(
        &self,
        request: ReceivePackRequest,
    ) -> Result<GitHttpResponse, GitHttpError> {
        self.git_http_client
            .service_rpc(
                &request.owner,
                &request.repo,
                "receive-pack",
                &request.content_type,
                request.body,
            )
            .await
    }
}
