use uuid::Uuid;

use crate::{
    dto::{OwnerName, RepositoryName, ReviewRef},
    error::ReviewError,
    util::review::MAGIC_REF_PREFIX,
};

#[derive(Debug, Clone)]
pub struct ProcessReviewRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub review_ref: ReviewRef,
    pub new_sha: String,
    pub pusher_id: Uuid,
}

impl ProcessReviewRequest {
    pub fn new(
        owner: &str,
        repo: &str,
        ref_name: &str,
        new_sha: String,
        pusher_id: Uuid,
    ) -> Result<Self, ReviewError> {
        Ok(Self {
            owner: OwnerName::try_new(owner)
                .map_err(|e| ReviewError::InvalidOwnerName(e.to_string()))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| ReviewError::InvalidRepositoryName(e.to_string()))?,
            review_ref: ReviewRef::try_new(ref_name)
                .map_err(|e| ReviewError::InvalidRefName(e.to_string()))?,
            new_sha,
            pusher_id,
        })
    }

    pub fn is_new(&self) -> bool {
        let rest = self
            .review_ref
            .as_ref()
            .strip_prefix(MAGIC_REF_PREFIX)
            .and_then(|r| r.strip_prefix('/'))
            .unwrap();

        !rest
            .rsplit_once('/')
            .is_some_and(|(_, number)| number.parse::<i64>().is_ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pusher_id() -> Uuid {
        Uuid::nil()
    }

    fn sha() -> String {
        "abc123".to_string()
    }

    #[test]
    fn valid_create_ref() {
        let req = ProcessReviewRequest::new("owner", "repo", "refs/for/main", sha(), pusher_id());
        assert!(req.is_ok());
        assert!(req.unwrap().is_new());
    }

    #[test]
    fn valid_create_ref_with_slashes_in_branch() {
        let req =
            ProcessReviewRequest::new("owner", "repo", "refs/for/feature/foo", sha(), pusher_id());
        assert!(req.is_ok());
        assert!(req.unwrap().is_new());
    }

    #[test]
    fn valid_update_ref() {
        let req =
            ProcessReviewRequest::new("owner", "repo", "refs/for/main/42", sha(), pusher_id());
        assert!(req.is_ok());
        assert!(!req.unwrap().is_new());
    }

    #[test]
    fn valid_update_ref_with_slashes_in_branch() {
        let req = ProcessReviewRequest::new(
            "owner",
            "repo",
            "refs/for/feature/foo/42",
            sha(),
            pusher_id(),
        );
        assert!(req.is_ok());
        assert!(!req.unwrap().is_new());
    }

    #[test]
    fn rejects_invalid_ref_no_prefix() {
        let req = ProcessReviewRequest::new(
            "owner",
            "repo",
            "refs/heads/main",
            sha(),
            pusher_id(),
        );
        assert!(matches!(req, Err(ReviewError::InvalidRefName(_))));
    }

    #[test]
    fn rejects_invalid_ref_empty_branch() {
        let req =
            ProcessReviewRequest::new("owner", "repo", "refs/for/", sha(), pusher_id());
        assert!(matches!(req, Err(ReviewError::InvalidRefName(_))));
    }

    #[test]
    fn rejects_invalid_ref_just_prefix() {
        let req =
            ProcessReviewRequest::new("owner", "repo", "refs/for", sha(), pusher_id());
        assert!(matches!(req, Err(ReviewError::InvalidRefName(_))));
    }

    #[test]
    fn rejects_invalid_owner() {
        let req =
            ProcessReviewRequest::new("", "repo", "refs/for/main", sha(), pusher_id());
        assert!(matches!(req, Err(ReviewError::InvalidOwnerName(_))));
    }

    #[test]
    fn rejects_invalid_repo() {
        let req =
            ProcessReviewRequest::new("owner", "", "refs/for/main", sha(), pusher_id());
        assert!(matches!(req, Err(ReviewError::InvalidRepositoryName(_))));
    }
}
