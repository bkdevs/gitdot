use async_trait::async_trait;

use supabase::{Client, Error};

#[async_trait]
pub trait SupabaseClient: Send + Sync + Clone + 'static {
    async fn create_user(&self, email: &str, password: &str) -> Result<(), Error>;
}

#[derive(Debug, Clone)]
pub struct SupabaseClientImpl {
    client: Client,
}

impl SupabaseClientImpl {
    pub fn new(supabase_project_url: &str, supabase_anon_key: &str) -> Result<Self, Error> {
        Ok(Self {
            client: Client::new(supabase_project_url, supabase_anon_key)?,
        })
    }
}

#[async_trait]
impl SupabaseClient for SupabaseClientImpl {
    async fn create_user(&self, email: &str, password: &str) -> Result<(), Error> {
        self.client
            .auth()
            .sign_up_with_email_and_password(email, password)
            .await?;
        Ok(())
    }
}
