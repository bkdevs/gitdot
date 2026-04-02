use std::collections::HashMap;

use uuid::Uuid;

use crate::model::UserRepoSettings;

pub struct UpdateCurrentUserSettingsRequest {
    pub user_id: Uuid,
    pub repos: Option<HashMap<String, UserRepoSettings>>,
}

impl UpdateCurrentUserSettingsRequest {
    pub fn new(user_id: Uuid, repos: Option<HashMap<String, UserRepoSettings>>) -> Self {
        Self { user_id, repos }
    }
}
