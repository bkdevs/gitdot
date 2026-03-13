use async_trait::async_trait;

use crate::{
    client::{Git2Client, GitClient},
    dto::{GetReviewRequest, ListReviewsRequest, ProcessReviewRequest, ReviewResponse},
    error::ReviewError,
    repository::{
        RepositoryRepository, RepositoryRepositoryImpl, ReviewRepository, ReviewRepositoryImpl,
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
}

#[derive(Debug, Clone)]
pub struct ReviewServiceImpl<V, R, G>
where
    V: ReviewRepository,
    R: RepositoryRepository,
    G: GitClient,
{
    review_repo: V,
    repo_repo: R,
    git_client: G,
}

impl ReviewServiceImpl<ReviewRepositoryImpl, RepositoryRepositoryImpl, Git2Client> {
    pub fn new(
        review_repo: ReviewRepositoryImpl,
        repo_repo: RepositoryRepositoryImpl,
        git_client: Git2Client,
    ) -> Self {
        Self {
            review_repo,
            repo_repo,
            git_client,
        }
    }
}

#[crate::instrument_all]
#[async_trait]
impl<V, R, G> ReviewService for ReviewServiceImpl<V, R, G>
where
    V: ReviewRepository,
    R: RepositoryRepository,
    G: GitClient,
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

        for (position, commit) in commits.iter().rev().enumerate() {
            let diff_title = commit
                .message
                .lines()
                .next()
                .unwrap_or(&commit.message)
                .to_string();
            let diff = self
                .review_repo
                .create_diff(review.id, (position + 1) as i32, &diff_title)
                .await?;

            self.review_repo
                .create_revision(diff.id, 1, &commit.sha)
                .await?;

            let diff_position = (position + 1) as i32;
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
}
