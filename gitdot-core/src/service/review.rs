use async_trait::async_trait;

use crate::{
    dto::{GetReviewRequest, ListReviewsRequest, ReviewResponse},
    error::ReviewError,
    repository::{ReviewRepository, ReviewRepositoryImpl},
};

#[async_trait]
pub trait ReviewService: Send + Sync + 'static {
    async fn get_review(&self, request: GetReviewRequest) -> Result<ReviewResponse, ReviewError>;

    async fn list_reviews(
        &self,
        request: ListReviewsRequest,
    ) -> Result<Vec<ReviewResponse>, ReviewError>;
}

#[derive(Debug, Clone)]
pub struct ReviewServiceImpl<V>
where
    V: ReviewRepository,
{
    review_repo: V,
}

impl ReviewServiceImpl<ReviewRepositoryImpl> {
    pub fn new(review_repo: ReviewRepositoryImpl) -> Self {
        Self { review_repo }
    }
}

#[async_trait]
impl<V> ReviewService for ReviewServiceImpl<V>
where
    V: ReviewRepository,
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
}
