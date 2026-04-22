use uuid::Uuid;

use crate::{
    dto::common::{OwnerName, RepositoryName},
    error::{InputError, ReviewError},
};

#[derive(Debug, Clone)]
pub struct JudgeReviewDiffRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub number: i32,
    pub position: i32,
    pub reviewer_id: Uuid,
    pub verdict: JudgeVerdict,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JudgeVerdict {
    Approve,
    Reject,
}

impl JudgeReviewDiffRequest {
    pub fn new(
        owner: &str,
        repo: &str,
        number: i32,
        position: i32,
        reviewer_id: Uuid,
        verdict: &str,
    ) -> Result<Self, ReviewError> {
        let verdict = match verdict {
            "approve" => JudgeVerdict::Approve,
            "reject" => JudgeVerdict::Reject,
            _ => {
                return Err(InputError::new(
                    "verdict",
                    format!("Invalid verdict: {verdict}. Must be approve or reject"),
                )
                .into());
            }
        };

        Ok(Self {
            owner: OwnerName::try_new(owner).map_err(|e| InputError::new("owner name", e))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| InputError::new("repository name", e))?,
            number,
            position,
            reviewer_id,
            verdict,
        })
    }
}
