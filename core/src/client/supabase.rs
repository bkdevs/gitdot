use async_trait::async_trait;

use supabase::{Client, Error, auth::User};

#[async_trait]
pub trait SupabaseClient: Send + Sync + Clone + 'static {
    async fn create_user(&self, email: &str, password: &str) -> Result<User, Error>;
}

#[derive(Debug, Clone)]
pub struct SupabaseClientImpl {
    client: Client,
}

impl SupabaseClientImpl {
    pub fn new(supabase_project_url: &str, supabase_anon_key: &str) -> Self {
        Self {
            client: Client::new(supabase_project_url, supabase_anon_key)
                .expect("Failed to create Supabase client"),
        }
    }
}

#[async_trait]
impl SupabaseClient for SupabaseClientImpl {
    async fn create_user(&self, email: &str, password: &str) -> Result<User, Error> {
        let response = self
            .client
            .auth()
            .sign_up_with_email_and_password(email, password)
            .await?;
        response
            .user
            .ok_or_else(|| Error::auth("Failed to create user"))
    }
}
