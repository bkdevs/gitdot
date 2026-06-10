use uuid::Uuid;

use crate::{
    dto::common::{OwnerName, RepositoryName},
    error::{InputError, WebhookError},
};

#[derive(Debug, Clone)]
pub struct SubscribeSlackWebhookRequest {
    pub user_id: Uuid,
    pub owner_name: OwnerName,
    pub repo_name: RepositoryName,
    pub slack_user_id: String,
    pub slack_team_id: String,
    pub slack_channel_id: String,
}

impl SubscribeSlackWebhookRequest {
    pub fn new(
        user_id: Uuid,
        owner: &str,
        repo: &str,
        slack_user_id: String,
        slack_team_id: String,
        slack_channel_id: String,
    ) -> Result<Self, WebhookError> {
        Ok(Self {
            user_id,
            owner_name: OwnerName::parse(owner, "owner name")?,
            repo_name: RepositoryName::try_new(repo)
                .map_err(|e| InputError::new("repository name", e))?,
            slack_user_id,
            slack_team_id,
            slack_channel_id,
        })
    }
}
