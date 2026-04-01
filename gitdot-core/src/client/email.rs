use async_trait::async_trait;
use resend_rs::{Resend, types::CreateEmailBaseOptions};

use crate::error::EmailError;

#[async_trait]
pub trait EmailClient: Send + Sync + Clone + 'static {
    async fn send_email(
        &self,
        from: &str,
        to: &str,
        subject: &str,
        html: &str,
    ) -> Result<(), EmailError>;
}

#[derive(Debug, Clone)]
pub struct ResendClient {
    resend: Resend,
}

impl ResendClient {
    pub fn new(api_key: &str) -> Self {
        Self {
            resend: Resend::new(api_key),
        }
    }
}

#[async_trait]
impl EmailClient for ResendClient {
    async fn send_email(
        &self,
        from: &str,
        to: &str,
        subject: &str,
        html: &str,
    ) -> Result<(), EmailError> {
        let options = CreateEmailBaseOptions::new(from, [to], subject).with_html(html);
        self.resend.emails.send(options).await?;
        Ok(())
    }
}
