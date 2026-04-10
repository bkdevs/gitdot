pub trait R2Client: Send + Sync + Clone + 'static {}

#[derive(Debug, Clone)]
pub struct R2ClientImpl {
    account_id: String,
    r2_access_key_id: String,
    r2_secret_access_key: String,
}

impl R2ClientImpl {
    pub fn new(account_id: String, r2_access_key_id: String, r2_secret_access_key: String) -> Self {
        Self {
            account_id,
            r2_access_key_id,
            r2_secret_access_key,
        }
    }
}

impl R2Client for R2ClientImpl {}
