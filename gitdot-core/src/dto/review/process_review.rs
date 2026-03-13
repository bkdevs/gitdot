use uuid::Uuid;

use crate::{
    dto::{OwnerName, RepositoryName},
    error::ReviewError,
    util::review::MAGIC_REF_PREFIX,
};

#[derive(Debug, Clone)]
pub struct ProcessReviewRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub target_branch: String,
    pub review_number: Option<i64>,
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
        let (target_branch, review_number) = Self::parse_ref(ref_name)?;

        Ok(Self {
            owner: OwnerName::try_new(owner)
                .map_err(|e| ReviewError::InvalidOwnerName(e.to_string()))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| ReviewError::InvalidRepositoryName(e.to_string()))?,
            target_branch,
            review_number,
            new_sha,
            pusher_id,
        })
    }

    pub fn is_new(&self) -> bool {
        self.review_number.is_none()
    }

    /// Parses a ref like `refs/for/<branch>` or `refs/for/<branch>/<number>`
    /// into (target_branch, optional review_number).
    fn parse_ref(ref_name: &str) -> Result<(String, Option<i64>), ReviewError> {
        let rest = ref_name
            .strip_prefix(MAGIC_REF_PREFIX)
            .and_then(|r| r.strip_prefix('/'))
            .filter(|r| !r.is_empty())
            .ok_or_else(|| {
                ReviewError::InvalidRefName(format!("invalid review ref: {ref_name}"))
            })?;

        match rest.rsplit_once('/') {
            Some((branch, number)) if number.parse::<i64>().is_ok() => {
                if branch.is_empty() {
                    return Err(ReviewError::InvalidRefName(format!(
                        "invalid review ref: {ref_name}"
                    )));
                }
                Ok((branch.to_string(), Some(number.parse().unwrap())))
            }
            _ => Ok((rest.to_string(), None)),
        }
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
    fn create_ref_parses_target_branch() {
        let req = ProcessReviewRequest::new("owner", "repo", "refs/for/main", sha(), pusher_id())
            .unwrap();
        assert!(req.is_new());
        assert_eq!(req.target_branch, "main");
        assert_eq!(req.review_number, None);
    }

    #[test]
    fn create_ref_with_slashes_in_branch() {
        let req =
            ProcessReviewRequest::new("owner", "repo", "refs/for/feature/foo", sha(), pusher_id())
                .unwrap();
        assert!(req.is_new());
        assert_eq!(req.target_branch, "feature/foo");
        assert_eq!(req.review_number, None);
    }

    #[test]
    fn update_ref_parses_branch_and_number() {
        let req =
            ProcessReviewRequest::new("owner", "repo", "refs/for/main/42", sha(), pusher_id())
                .unwrap();
        assert!(!req.is_new());
        assert_eq!(req.target_branch, "main");
        assert_eq!(req.review_number, Some(42));
    }

    #[test]
    fn update_ref_with_slashes_in_branch() {
        let req = ProcessReviewRequest::new(
            "owner",
            "repo",
            "refs/for/feature/foo/42",
            sha(),
            pusher_id(),
        )
        .unwrap();
        assert!(!req.is_new());
        assert_eq!(req.target_branch, "feature/foo");
        assert_eq!(req.review_number, Some(42));
    }

    #[test]
    fn branch_name_with_trailing_non_numeric_segment() {
        let req =
            ProcessReviewRequest::new("owner", "repo", "refs/for/feature/bar", sha(), pusher_id())
                .unwrap();
        assert!(req.is_new());
        assert_eq!(req.target_branch, "feature/bar");
        assert_eq!(req.review_number, None);
    }

    #[test]
    fn single_segment_branch() {
        let req =
            ProcessReviewRequest::new("owner", "repo", "refs/for/develop", sha(), pusher_id())
                .unwrap();
        assert_eq!(req.target_branch, "develop");
        assert_eq!(req.review_number, None);
    }

    #[test]
    fn review_number_one() {
        let req = ProcessReviewRequest::new("owner", "repo", "refs/for/main/1", sha(), pusher_id())
            .unwrap();
        assert_eq!(req.target_branch, "main");
        assert_eq!(req.review_number, Some(1));
    }

    #[test]
    fn large_review_number() {
        let req =
            ProcessReviewRequest::new("owner", "repo", "refs/for/main/99999", sha(), pusher_id())
                .unwrap();
        assert_eq!(req.target_branch, "main");
        assert_eq!(req.review_number, Some(99999));
    }

    #[test]
    fn rejects_invalid_ref_no_prefix() {
        let req = ProcessReviewRequest::new("owner", "repo", "refs/heads/main", sha(), pusher_id());
        assert!(matches!(req, Err(ReviewError::InvalidRefName(_))));
    }

    #[test]
    fn rejects_invalid_ref_empty_branch() {
        let req = ProcessReviewRequest::new("owner", "repo", "refs/for/", sha(), pusher_id());
        assert!(matches!(req, Err(ReviewError::InvalidRefName(_))));
    }

    #[test]
    fn rejects_invalid_ref_just_prefix() {
        let req = ProcessReviewRequest::new("owner", "repo", "refs/for", sha(), pusher_id());
        assert!(matches!(req, Err(ReviewError::InvalidRefName(_))));
    }

    #[test]
    fn rejects_invalid_owner() {
        let req = ProcessReviewRequest::new("", "repo", "refs/for/main", sha(), pusher_id());
        assert!(matches!(req, Err(ReviewError::InvalidOwnerName(_))));
    }

    #[test]
    fn rejects_invalid_repo() {
        let req = ProcessReviewRequest::new("owner", "", "refs/for/main", sha(), pusher_id());
        assert!(matches!(req, Err(ReviewError::InvalidRepositoryName(_))));
    }
}
