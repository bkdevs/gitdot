use async_trait::async_trait;

use crate::{
    client::{DiffClient, DifftClient, Git2Client, GitClient},
    dto::{
        AddReviewerRequest, GetReviewDiffRequest, GetReviewRequest, ListReviewsRequest,
        ProcessReviewRequest, PublishReviewRequest, RemoveReviewerRequest, ReviewDiffResponse,
        ReviewFileDiffResponse, ReviewResponse, ReviewerResponse,
    },
    error::ReviewError,
    model::ReviewStatus,
    repository::{
        RepositoryRepository, RepositoryRepositoryImpl, ReviewRepository, ReviewRepositoryImpl,
        UserRepository, UserRepositoryImpl,
    },
    util::review::{get_current_ref, get_head_ref, get_revision_ref, get_target_ref},
};

#[async_trait]
pub trait ReviewService: Send + Sync + 'static {
    async fn get_review(&self, request: GetReviewRequest) -> Result<ReviewResponse, ReviewError>;

    async fn list_reviews(
        &self,
        request: ListReviewsRequest,
    ) -> Result<Vec<ReviewResponse>, ReviewError>;

    async fn create_review(
        &self,
        request: ProcessReviewRequest,
    ) -> Result<ReviewResponse, ReviewError>;

    async fn add_reviewer(
        &self,
        request: AddReviewerRequest,
    ) -> Result<ReviewerResponse, ReviewError>;

    async fn remove_reviewer(&self, request: RemoveReviewerRequest) -> Result<(), ReviewError>;

    async fn publish_review(
        &self,
        request: PublishReviewRequest,
    ) -> Result<ReviewResponse, ReviewError>;

    async fn get_review_diff(
        &self,
        request: GetReviewDiffRequest,
    ) -> Result<ReviewDiffResponse, ReviewError>;
}

#[derive(Debug, Clone)]
pub struct ReviewServiceImpl<V, R, U, G, D>
where
    V: ReviewRepository,
    R: RepositoryRepository,
    U: UserRepository,
    G: GitClient,
    D: DiffClient,
{
    review_repo: V,
    repo_repo: R,
    user_repo: U,
    git_client: G,
    diff_client: D,
}

impl
    ReviewServiceImpl<
        ReviewRepositoryImpl,
        RepositoryRepositoryImpl,
        UserRepositoryImpl,
        Git2Client,
        DifftClient,
    >
{
    pub fn new(
        review_repo: ReviewRepositoryImpl,
        repo_repo: RepositoryRepositoryImpl,
        user_repo: UserRepositoryImpl,
        git_client: Git2Client,
        diff_client: DifftClient,
    ) -> Self {
        Self {
            review_repo,
            repo_repo,
            user_repo,
            git_client,
            diff_client,
        }
    }
}

#[crate::instrument_all]
#[async_trait]
impl<V, R, U, G, D> ReviewService for ReviewServiceImpl<V, R, U, G, D>
where
    V: ReviewRepository,
    R: RepositoryRepository,
    U: UserRepository,
    G: GitClient,
    D: DiffClient,
{
    async fn get_review(&self, request: GetReviewRequest) -> Result<ReviewResponse, ReviewError> {
        let review = self
            .review_repo
            .get_review(
                request.owner.as_ref(),
                request.repo.as_ref(),
                request.number,
            )
            .await?
            .ok_or_else(|| ReviewError::ReviewNotFound(request.get_review_path()))?;

        Ok(review.into())
    }

    async fn list_reviews(
        &self,
        request: ListReviewsRequest,
    ) -> Result<Vec<ReviewResponse>, ReviewError> {
        let reviews = self
            .review_repo
            .get_reviews(request.owner.as_ref(), request.repo.as_ref())
            .await?;

        Ok(reviews.into_iter().map(ReviewResponse::from).collect())
    }

    async fn create_review(
        &self,
        request: ProcessReviewRequest,
    ) -> Result<ReviewResponse, ReviewError> {
        let owner = request.owner.as_ref();
        let repo = request.repo.as_ref();

        let target_sha = self
            .git_client
            .resolve_ref_sha(owner, repo, &get_target_ref(&request.target_branch))
            .await?;

        let commits = self
            .git_client
            .rev_list(owner, repo, &target_sha, &request.new_sha)
            .await?;
        if commits.is_empty() {
            return Err(ReviewError::CommitsNotFound);
        }

        let repository = self
            .repo_repo
            .get(owner, repo)
            .await?
            .ok_or_else(|| ReviewError::RepositoryNotFound(format!("{}/{}", owner, repo)))?;
        let review = self
            .review_repo
            .create_review(repository.id, request.pusher_id, &request.target_branch)
            .await?;

        let mut previous_sha = target_sha.clone();
        for (position, commit) in commits.iter().rev().enumerate() {
            let commit_title = commit
                .message
                .lines()
                .next()
                .unwrap_or(&commit.message)
                .to_string();
            let diff_position = (position + 1) as i32;
            let diff = self
                .review_repo
                .create_diff(review.id, diff_position, &commit_title, &commit_title)
                .await?;

            self.review_repo
                .create_revision(diff.id, 1, &commit.sha, &previous_sha)
                .await?;
            previous_sha = commit.sha.clone();

            self.git_client
                .create_ref(
                    owner,
                    repo,
                    &get_revision_ref(review.number, diff_position, 1),
                    &commit.sha,
                )
                .await?;

            self.git_client
                .create_ref(
                    owner,
                    repo,
                    &get_current_ref(review.number, diff_position),
                    &commit.sha,
                )
                .await?;
        }

        self.git_client
            .create_ref(owner, repo, &get_head_ref(review.number), &request.new_sha)
            .await?;

        Ok(review.into())
    }

    async fn add_reviewer(
        &self,
        request: AddReviewerRequest,
    ) -> Result<ReviewerResponse, ReviewError> {
        let user = self
            .user_repo
            .get(request.user_name.as_ref())
            .await?
            .ok_or_else(|| ReviewError::UserNotFound(request.user_name.to_string()))?;

        let review = self
            .review_repo
            .get_review(
                request.owner.as_ref(),
                request.repo.as_ref(),
                request.number,
            )
            .await?
            .ok_or_else(|| {
                ReviewError::ReviewNotFound(format!(
                    "{}/{}/review/{}",
                    request.owner.as_ref(),
                    request.repo.as_ref(),
                    request.number
                ))
            })?;

        if user.id == review.author_id {
            return Err(ReviewError::CannotReviewOwnReview(
                request.user_name.to_string(),
            ));
        }

        let reviewer = self
            .review_repo
            .add_reviewer(review.id, user.id)
            .await?
            .ok_or_else(|| ReviewError::ReviewerAlreadyExists(request.user_name.to_string()))?;

        Ok(reviewer.into())
    }

    async fn remove_reviewer(&self, request: RemoveReviewerRequest) -> Result<(), ReviewError> {
        let user = self
            .user_repo
            .get(request.reviewer_name.as_ref())
            .await?
            .ok_or_else(|| ReviewError::UserNotFound(request.reviewer_name.to_string()))?;

        let review = self
            .review_repo
            .get_review(
                request.owner.as_ref(),
                request.repo.as_ref(),
                request.number,
            )
            .await?
            .ok_or_else(|| {
                ReviewError::ReviewNotFound(format!(
                    "{}/{}/review/{}",
                    request.owner.as_ref(),
                    request.repo.as_ref(),
                    request.number
                ))
            })?;

        let removed = self.review_repo.remove_reviewer(review.id, user.id).await?;
        if !removed {
            return Err(ReviewError::ReviewerNotFound(
                request.reviewer_name.to_string(),
            ));
        }

        Ok(())
    }

    async fn publish_review(
        &self,
        request: PublishReviewRequest,
    ) -> Result<ReviewResponse, ReviewError> {
        let review = self
            .review_repo
            .get_review(
                request.owner.as_ref(),
                request.repo.as_ref(),
                request.number,
            )
            .await?
            .ok_or_else(|| {
                ReviewError::ReviewNotFound(format!(
                    "{}/{}/review/{}",
                    request.owner.as_ref(),
                    request.repo.as_ref(),
                    request.number
                ))
            })?;

        if review.status != ReviewStatus::Draft {
            return Err(ReviewError::ReviewNotPublishable(format!(
                "review is in '{}' status, expected 'draft'",
                serde_json::to_string(&review.status)
                    .unwrap_or_default()
                    .trim_matches('"')
            )));
        }

        let title = request.title.as_deref().unwrap_or(&review.title);
        let description = request
            .description
            .as_deref()
            .unwrap_or(&review.description);
        self.review_repo
            .publish_review(review.id, title, description)
            .await?;

        let diffs = review.diffs.unwrap_or_default();
        for diff_update in &request.diffs {
            if let Some(diff) = diffs.iter().find(|d| d.position == diff_update.position) {
                let diff_title = diff_update.title.as_deref().unwrap_or(&diff.title);
                let diff_desc = diff_update
                    .description
                    .as_deref()
                    .unwrap_or(&diff.description);
                self.review_repo
                    .update_diff(diff.id, diff_title, diff_desc)
                    .await?;
            }
        }

        let updated = self
            .review_repo
            .get_review(
                request.owner.as_ref(),
                request.repo.as_ref(),
                request.number,
            )
            .await?
            .ok_or_else(|| {
                ReviewError::ReviewNotFound(format!(
                    "{}/{}/review/{}",
                    request.owner.as_ref(),
                    request.repo.as_ref(),
                    request.number
                ))
            })?;

        Ok(updated.into())
    }

    async fn get_review_diff(
        &self,
        request: GetReviewDiffRequest,
    ) -> Result<ReviewDiffResponse, ReviewError> {
        let owner = request.owner.as_ref();
        let repo = request.repo.as_ref();

        let review = self
            .review_repo
            .get_review(owner, repo, request.number)
            .await?
            .ok_or_else(|| {
                ReviewError::ReviewNotFound(format!("{}/{}/review/{}", owner, repo, request.number))
            })?;

        let diffs = review.diffs.unwrap_or_default();
        let diff = diffs
            .iter()
            .find(|d| d.position == request.position)
            .ok_or_else(|| {
                ReviewError::DiffNotFound(format!(
                    "{}/{}/review/{}/diff/{}",
                    owner, repo, request.number, request.position
                ))
            })?;

        let revisions = diff.revisions.as_ref().cloned().unwrap_or_default();
        let revision = if let Some(rev_num) = request.revision {
            revisions
                .iter()
                .find(|r| r.number == rev_num)
                .ok_or_else(|| {
                    ReviewError::RevisionNotFound(format!(
                        "{}/{}/review/{}/diff/{}/revision/{}",
                        owner, repo, request.number, request.position, rev_num
                    ))
                })?
        } else {
            revisions.first().ok_or_else(|| {
                ReviewError::RevisionNotFound(format!(
                    "{}/{}/review/{}/diff/{} has no revisions",
                    owner, repo, request.number, request.position
                ))
            })?
        };

        let right_sha = &revision.commit_hash;
        let left_sha = if let Some(compare_to) = request.compare_to {
            let compare_rev = revisions
                .iter()
                .find(|r| r.number == compare_to)
                .ok_or_else(|| {
                    ReviewError::RevisionNotFound(format!(
                        "{}/{}/review/{}/diff/{}/revision/{}",
                        owner, repo, request.number, request.position, compare_to
                    ))
                })?;
            compare_rev.commit_hash.clone()
        } else {
            revision.parent_hash.clone()
        };

        let diff_files = self
            .git_client
            .get_repo_diff_files(owner, repo, Some(&left_sha), right_sha)
            .await?;

        let mut files = Vec::new();
        for (left, right) in diff_files {
            let path = right
                .as_ref()
                .or(left.as_ref())
                .map(|f| f.path.clone())
                .unwrap_or_default();
            let diff = self
                .diff_client
                .diff_files(left.as_ref(), right.as_ref())
                .await?;
            files.push(ReviewFileDiffResponse { path, diff });
        }

        Ok(ReviewDiffResponse { files })
    }
}
