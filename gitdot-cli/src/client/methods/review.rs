use anyhow::Result;

use gitdot_api::endpoint::review::update_review::{UpdateReviewRequest, UpdateReviewResponse};

use crate::client::GitdotClient;

#[allow(dead_code)]
impl GitdotClient {
    pub async fn update_review(
        &self,
        owner: &str,
        repo: &str,
        number: i32,
        title: Option<String>,
        description: Option<String>,
    ) -> Result<UpdateReviewResponse> {
        self.patch(
            format!("repository/{}/{}/review/{}", owner, repo, number),
            UpdateReviewRequest { title, description },
        )
        .await
    }
}
