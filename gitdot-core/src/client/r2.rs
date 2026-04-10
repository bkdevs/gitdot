use async_trait::async_trait;
use aws_config::BehaviorVersion;
use aws_sdk_s3::{config::Credentials, primitives::ByteStream};
use bytes::Bytes;

use crate::error::R2Error;

#[async_trait]
pub trait R2Client: Send + Sync + Clone + 'static {
    async fn upload_object(&self, key: &str, body: Bytes) -> Result<(), R2Error>;
}

#[derive(Clone)]
pub struct R2ClientImpl {
    client: aws_sdk_s3::Client,
    bucket_name: String,
}

impl R2ClientImpl {
    pub async fn new(
        account_id: String,
        bucket_name: String,
        access_key_id: String,
        secret_access_key: String,
    ) -> Self {
        let config = aws_config::defaults(BehaviorVersion::latest())
            .endpoint_url(format!("https://{}.r2.cloudflarestorage.com", account_id))
            .credentials_provider(Credentials::new(
                access_key_id,
                secret_access_key,
                None,
                None,
                "R2",
            ))
            .region(aws_config::Region::new("auto"))
            .load()
            .await;

        Self {
            client: aws_sdk_s3::Client::new(&config),
            bucket_name,
        }
    }
}

#[async_trait]
impl R2Client for R2ClientImpl {
    async fn upload_object(&self, key: &str, body: Bytes) -> Result<(), R2Error> {
        self.client
            .put_object()
            .bucket(&self.bucket_name)
            .key(key)
            .body(ByteStream::from(body))
            .send()
            .await
            .map_err(|e| R2Error::UploadError(e.to_string()))?;

        Ok(())
    }
}
