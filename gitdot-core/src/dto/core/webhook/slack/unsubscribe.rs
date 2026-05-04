use uuid::Uuid;

use crate::{
    dto::common::{OwnerName, RepositoryName},
    error::{InputError, WebhookError},
};

#[derive(Debug, Clone)]
pub struct UnsubscribeSlackWebhookRequest {
    pub webhook_id: Uuid,
    pub user_id: Uuid,
    pub owner_name: OwnerName,
    pub repo_name: RepositoryName,
    pub slack_user_id: String,
    pub slack_team_id: String,
    pub slack_channel_id: String,
}

impl UnsubscribeSlackWebhookRequest {
    pub fn new(
        webhook_id: Uuid,
        user_id: Uuid,
        owner: &str,
        repo: &str,
        slack_user_id: String,
        slack_team_id: String,
        slack_channel_id: String,
    ) -> Result<Self, WebhookError> {
        Ok(Self {
            webhook_id,
            user_id,
            owner_name: OwnerName::try_new(owner).map_err(|e| InputError::new("owner name", e))?,
            repo_name: RepositoryName::try_new(repo)
                .map_err(|e| InputError::new("repository name", e))?,
            slack_user_id,
            slack_team_id,
            slack_channel_id,
        })
    }
}
