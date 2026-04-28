use async_trait::async_trait;

use crate::{
    client::{DiffClient, DifftClient, Git2Client, GitClient},
    dto::{
        AddReviewReviewerReqeuest, ApproveReviewDiffRequest, CreateReviewCommentsRequest,
        RejectReviewDiffRequest,
        GetReviewDiffRequest, GetReviewRequest, ListReviewsRequest, MergeReviewDiffRequest,
        MergeReviewRequest, ProcessReviewRequest, PublishReviewDiffRequest, PublishReviewRequest,
        RemoveReviewReviewerRequest, ResolveReviewCommentRequest,
        ReviewCommentResponse, ReviewDiffResponse, ReviewResponse, ReviewerResponse, ReviewsResponse,
        UpdateReviewCommentRequest, UpdateReviewDiffRequest, UpdateReviewRequest,
    },
    error::{ConflictError, InputError, NotFoundError, OptionNotFoundExt, ReviewError},
    model::{DiffStatus, Review, ReviewStatus, Verdict},
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
    ) -> Result<ReviewsResponse, ReviewError>;

    /// Creates a new review from a push to `refs/for/<branch>`.
    ///
    /// Triggered by the proc-receive hook when a user pushes to a magic ref
    /// without a review number (e.g. `refs/for/main`).
    ///
    /// Steps:
    /// 1. Resolve the target branch SHA and list new commits between it and the pushed ref.
    /// 2. Create a review record (draft status, auto-incremented number).
    /// 3. For each commit (oldest first), create a diff (position 1..N) and an initial
    ///    revision (number 1) with the commit's SHA and parent SHA.
    /// 4. Create git refs for tracking:
    ///    - `refs/reviews/<number>/diffs/<position>/revisions/1` — specific revision
    ///    - `refs/reviews/<number>/diffs/<position>/current` — latest revision of this diff
    ///    - `refs/reviews/<number>/head` — tip of the review
    async fn create_review(
        &self,
        request: ProcessReviewRequest,
    ) -> Result<ReviewResponse, ReviewError>;

    /// Updates an existing review from a push to `refs/for/<branch>/<number>`.
    ///
    /// Triggered by the proc-receive hook when a user pushes to a magic ref
    /// that includes a review number (e.g. `refs/for/main/42`).
    ///
    /// Steps:
    /// 1. Fetch the review (with diffs and revisions) and verify the pusher is the author.
    /// 2. Resolve the target branch SHA and list new commits between it and the pushed ref.
    /// 3. For each commit (oldest first), match it to an existing diff by position:
    ///    - **Existing diff, unchanged**: Compare patch IDs (content-based hash of
    ///      added/removed lines, ignoring context and line numbers). If the patch is
    ///      identical, the commit was only rebased — update the latest revision's
    ///      commit_hash and parent_hash in place, and force-update the git refs.
    ///    - **Existing diff, modified**: The patch changed — create a new revision
    ///      (incrementing the revision number), create a new revision ref, force-update
    ///      the current ref, reset the diff status to `open` (invalidating any prior
    ///      approval or change request), and update `updated_at`.
    ///    - **New diff position**: More commits than before — create a new diff and
    ///      initial revision, same as in `create_review`.
    /// 4. Force-update `refs/reviews/<number>/head` to the pushed SHA.
    /// 5. Touch the review's `updated_at`.

    async fn process_review_update(
        &self,
        request: ProcessReviewRequest,
    ) -> Result<ReviewResponse, ReviewError>;

    async fn publish_review(
        &self,
        request: PublishReviewRequest,
    ) -> Result<ReviewResponse, ReviewError>;

    async fn update_review(
        &self,
        request: UpdateReviewRequest,
    ) -> Result<ReviewResponse, ReviewError>;

    async fn get_review_diff(
        &self,
        request: GetReviewDiffRequest,
    ) -> Result<ReviewDiffResponse, ReviewError>;

    async fn update_review_diff(
        &self,
        request: UpdateReviewDiffRequest,
    ) -> Result<ReviewResponse, ReviewError>;

    async fn publish_review_diff(
        &self,
        request: PublishReviewDiffRequest,
    ) -> Result<ReviewResponse, ReviewError>;

    async fn merge_review_diff(
        &self,
        request: MergeReviewDiffRequest,
    ) -> Result<ReviewResponse, ReviewError>;

    async fn merge_review(
        &self,
        request: MergeReviewRequest,
    ) -> Result<ReviewResponse, ReviewError>;

    async fn add_review_reviewer(
        &self,
        request: AddReviewReviewerReqeuest,
    ) -> Result<ReviewerResponse, ReviewError>;

    async fn remove_review_reviewer(
        &self,
        request: RemoveReviewReviewerRequest,
    ) -> Result<(), ReviewError>;

    async fn update_review_comment(
        &self,
        request: UpdateReviewCommentRequest,
    ) -> Result<ReviewCommentResponse, ReviewError>;

    async fn resolve_review_comment(
        &self,
        request: ResolveReviewCommentRequest,
    ) -> Result<ReviewCommentResponse, ReviewError>;

    async fn create_review_comments(
        &self,
        request: CreateReviewCommentsRequest,
    ) -> Result<ReviewResponse, ReviewError>;

    async fn approve_review_diff(
        &self,
        request: ApproveReviewDiffRequest,
    ) -> Result<ReviewResponse, ReviewError>;

    async fn reject_review_diff(
        &self,
        request: RejectReviewDiffRequest,
    ) -> Result<ReviewResponse, ReviewError>;
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

impl<V, R, U, G, D> ReviewServiceImpl<V, R, U, G, D>
where
    V: ReviewRepository,
    R: RepositoryRepository,
    U: UserRepository,
    G: GitClient,
    D: DiffClient,
{
    async fn get_review_by_id(
        &self,
        owner: &str,
        repo: &str,
        number: i32,
    ) -> Result<Review, ReviewError> {
        Ok(self
            .review_repo
            .get_review_by_number(owner, repo, number)
            .await?
            .or_not_found("review", format!("{}/{}/review/{}", owner, repo, number))?)
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
            .get_review_by_id(
                request.owner.as_ref(),
                request.repo.as_ref(),
                request.number,
            )
            .await?;

        Ok(review.into())
    }

    async fn list_reviews(
        &self,
        request: ListReviewsRequest,
    ) -> Result<ReviewsResponse, ReviewError> {
        let reviews = self
            .review_repo
            .list_reviews(
                request.owner.as_ref(),
                request.repo.as_ref(),
                request.viewer_id,
                request.from,
                request.to,
            )
            .await?;

        Ok(ReviewsResponse {
            reviews: reviews.into_iter().map(ReviewResponse::from).collect(),
        })
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
            .or_not_found("repository", format!("{}/{}", owner, repo))?;
        let review = self
            .review_repo
            .create_review(repository.id, request.pusher_id, &request.target_branch)
            .await?;

        let review_ref_id = &review.id.to_string()[..8];
        let mut previous_sha = target_sha.clone();
        for (position, commit) in commits.iter().rev().enumerate() {
            let diff_position = (position + 1) as i32;
            let diff = self
                .review_repo
                .create_diff(review.id, diff_position, &commit.message)
                .await?;

            self.review_repo
                .create_revision(diff.id, 1, &commit.sha, &previous_sha)
                .await?;
            previous_sha = commit.sha.clone();

            self.git_client
                .create_ref(
                    owner,
                    repo,
                    &get_revision_ref(review_ref_id, diff_position, 1),
                    &commit.sha,
                )
                .await?;

            self.git_client
                .create_ref(
                    owner,
                    repo,
                    &get_current_ref(review_ref_id, diff_position),
                    &commit.sha,
                )
                .await?;
        }

        self.git_client
            .create_ref(owner, repo, &get_head_ref(review_ref_id), &request.new_sha)
            .await?;

        Ok(review.into())
    }

    async fn process_review_update(
        &self,
        request: ProcessReviewRequest,
    ) -> Result<ReviewResponse, ReviewError> {
        let owner = request.owner.as_ref();
        let repo = request.repo.as_ref();
        let review_number = request
            .review_number
            .ok_or_else(|| InputError::new("ref name", "missing review number"))?
            as i32;

        let review = self
            .review_repo
            .get_review_by_number(owner, repo, review_number)
            .await?
            .or_not_found(
                "review",
                format!("{}/{}/review/{}", owner, repo, review_number),
            )?;
        let review_ref_id = review.id.to_string();
        let review_ref_id = &review_ref_id[..8];

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

        let existing_diffs = review.diffs.unwrap_or_default();

        let mut previous_sha = target_sha.clone();
        for (position, commit) in commits.iter().rev().enumerate() {
            let diff_position = (position + 1) as i32;

            if let Some(existing_diff) = existing_diffs.iter().find(|d| d.position == diff_position)
            {
                let revisions = existing_diff
                    .revisions
                    .as_ref()
                    .cloned()
                    .unwrap_or_default();
                let latest_revision = revisions.first().or_not_found(
                    "revision",
                    format!(
                        "{}/{}/review/{}/diff/{} has no revisions",
                        owner, repo, review_number, diff_position
                    ),
                )?;

                let old_patch_id = self
                    .git_client
                    .get_commit_patch_id(owner, repo, &latest_revision.commit_hash)
                    .await?;
                let new_patch_id = self
                    .git_client
                    .get_commit_patch_id(owner, repo, &commit.sha)
                    .await?;

                if old_patch_id == new_patch_id {
                    // Rebased but unchanged — update SHA only
                    self.review_repo
                        .update_revision_sha(latest_revision.id, &commit.sha, &previous_sha)
                        .await?;

                    self.git_client
                        .update_ref(
                            owner,
                            repo,
                            &get_revision_ref(review_ref_id, diff_position, latest_revision.number),
                            &commit.sha,
                        )
                        .await?;

                    self.git_client
                        .update_ref(
                            owner,
                            repo,
                            &get_current_ref(review_ref_id, diff_position),
                            &commit.sha,
                        )
                        .await?;
                } else {
                    // Actually modified — create new revision
                    let new_revision_number = latest_revision.number + 1;

                    self.review_repo
                        .create_revision(
                            existing_diff.id,
                            new_revision_number,
                            &commit.sha,
                            &previous_sha,
                        )
                        .await?;

                    self.git_client
                        .create_ref(
                            owner,
                            repo,
                            &get_revision_ref(review_ref_id, diff_position, new_revision_number),
                            &commit.sha,
                        )
                        .await?;

                    self.git_client
                        .update_ref(
                            owner,
                            repo,
                            &get_current_ref(review_ref_id, diff_position),
                            &commit.sha,
                        )
                        .await?;

                    self.review_repo
                        .update_diff(existing_diff.id, Some(DiffStatus::Open), None)
                        .await?;
                }
            } else {
                // New diff position — create diff + revision
                let diff = self
                    .review_repo
                    .create_diff(review.id, diff_position, &commit.message)
                    .await?;

                self.review_repo
                    .create_revision(diff.id, 1, &commit.sha, &previous_sha)
                    .await?;

                self.git_client
                    .create_ref(
                        owner,
                        repo,
                        &get_revision_ref(review_ref_id, diff_position, 1),
                        &commit.sha,
                    )
                    .await?;

                self.git_client
                    .create_ref(
                        owner,
                        repo,
                        &get_current_ref(review_ref_id, diff_position),
                        &commit.sha,
                    )
                    .await?;
            }

            previous_sha = commit.sha.clone();
        }

        self.git_client
            .update_ref(owner, repo, &get_head_ref(review_ref_id), &request.new_sha)
            .await?;

        self.review_repo
            .update_review(review.id, None, None, None)
            .await?;

        let updated = self
            .review_repo
            .get_review_by_number(owner, repo, review_number)
            .await?
            .or_not_found(
                "review",
                format!("{}/{}/review/{}", owner, repo, review_number),
            )?;

        Ok(updated.into())
    }

    async fn publish_review(
        &self,
        request: PublishReviewRequest,
    ) -> Result<ReviewResponse, ReviewError> {
        let owner = request.owner.as_ref();
        let repo = request.repo.as_ref();

        let review = self.get_review_by_id(owner, repo, request.number).await?;

        if review.status != ReviewStatus::Draft {
            return Err(ReviewError::ReviewNotPublishable(format!(
                "review is in '{}' status, expected 'draft'",
                serde_json::to_string(&review.status)
                    .unwrap_or_default()
                    .trim_matches('"')
            )));
        }

        let pending_count = review
            .diffs
            .as_deref()
            .unwrap_or(&[])
            .iter()
            .filter(|d| d.status == DiffStatus::Draft)
            .count();

        if pending_count > 0 {
            return Err(ReviewError::ReviewNotPublishable(format!(
                "{} diff(s) are still in draft — all diffs must be open before publishing",
                pending_count
            )));
        }

        self.review_repo
            .update_review(review.id, Some(ReviewStatus::Open), None, None)
            .await?;

        let updated = self.get_review_by_id(owner, repo, request.number).await?;
        Ok(updated.into())
    }

    async fn publish_review_diff(
        &self,
        request: PublishReviewDiffRequest,
    ) -> Result<ReviewResponse, ReviewError> {
        let owner = request.owner.as_ref();
        let repo = request.repo.as_ref();

        let review = self.get_review_by_id(owner, repo, request.number).await?;

        if review.status != ReviewStatus::Draft {
            return Err(ReviewError::DiffNotPublishable(format!(
                "review is in '{}' status, expected 'draft'",
                serde_json::to_string(&review.status)
                    .unwrap_or_default()
                    .trim_matches('"')
            )));
        }

        let diffs = review.diffs.as_deref().unwrap_or(&[]);
        let diff = diffs
            .iter()
            .find(|d| d.position == request.position)
            .or_not_found("diff", format!("position {}", request.position))?;

        if diff.status != DiffStatus::Draft {
            return Err(ReviewError::DiffNotPublishable(format!(
                "diff at position {} is in '{}' status, expected 'draft'",
                request.position,
                serde_json::to_string(&diff.status)
                    .unwrap_or_default()
                    .trim_matches('"')
            )));
        }

        self.review_repo
            .update_diff(diff.id, Some(DiffStatus::Open), None)
            .await?;

        self.review_repo
            .update_review(review.id, None, None, None)
            .await?;

        let updated = self.get_review_by_id(owner, repo, request.number).await?;
        Ok(updated.into())
    }

    async fn update_review(
        &self,
        request: UpdateReviewRequest,
    ) -> Result<ReviewResponse, ReviewError> {
        let owner = request.owner.as_ref();
        let repo = request.repo.as_ref();

        let review = self.get_review_by_id(owner, repo, request.number).await?;

        self.review_repo
            .update_review(review.id, None, request.title, request.description)
            .await?;

        let updated = self.get_review_by_id(owner, repo, request.number).await?;

        Ok(updated.into())
    }

    async fn get_review_diff(
        &self,
        request: GetReviewDiffRequest,
    ) -> Result<ReviewDiffResponse, ReviewError> {
        let owner = request.owner.as_ref();
        let repo = request.repo.as_ref();

        let review = self.get_review_by_id(owner, repo, request.number).await?;
        let review_number = review.number;

        let diffs = review.diffs.unwrap_or_default();
        let diff = diffs
            .iter()
            .find(|d| d.position == request.position)
            .or_not_found(
                "diff",
                format!(
                    "{}/{}/review/{}/diff/{}",
                    owner, repo, review_number, request.position
                ),
            )?;

        let revisions = diff.revisions.as_ref().cloned().unwrap_or_default();
        let revision = if let Some(rev_num) = request.revision {
            revisions
                .iter()
                .find(|r| r.number == rev_num)
                .or_not_found(
                    "revision",
                    format!(
                        "{}/{}/review/{}/diff/{}/revision/{}",
                        owner, repo, review_number, request.position, rev_num
                    ),
                )?
        } else {
            revisions.first().or_not_found(
                "revision",
                format!(
                    "{}/{}/review/{}/diff/{} has no revisions",
                    owner, repo, review_number, request.position
                ),
            )?
        };

        let right_sha = &revision.commit_hash;
        let left_sha = if let Some(compare_to) = request.compare_to {
            let compare_rev = revisions
                .iter()
                .find(|r| r.number == compare_to)
                .or_not_found(
                    "revision",
                    format!(
                        "{}/{}/review/{}/diff/{}/revision/{}",
                        owner, repo, review_number, request.position, compare_to
                    ),
                )?;
            compare_rev.commit_hash.clone()
        } else {
            revision.parent_hash.clone()
        };

        let diff_files = self
            .git_client
            .get_repo_diff_files(owner, repo, Some(&left_sha), right_sha)
            .await?;

        let files = futures::future::try_join_all(
            diff_files
                .iter()
                .map(|(left, right)| self.diff_client.diff_files(left.as_ref(), right.as_ref())),
        )
        .await?;

        Ok(ReviewDiffResponse { files })
    }

    async fn merge_review_diff(
        &self,
        request: MergeReviewDiffRequest,
    ) -> Result<ReviewResponse, ReviewError> {
        let owner = request.owner.as_ref();
        let repo = request.repo.as_ref();

        let review = self.get_review_by_id(owner, repo, request.number).await?;

        if review.status != ReviewStatus::Open {
            return Err(ReviewError::DiffNotMergeable(
                "review must be open to merge diffs".to_string(),
            ));
        }

        let diffs = review.diffs.unwrap_or_default();
        let diffs_to_merge: Vec<_> = diffs
            .iter()
            .filter(|d| d.position <= request.position && d.status != DiffStatus::Merged)
            .collect();

        if diffs_to_merge.is_empty() {
            return Err(NotFoundError::new(
                "diff",
                format!(
                    "no open diffs found at or before position {}",
                    request.position
                ),
            )
            .into());
        }

        let mut diff_revisions = Vec::new();
        for diff in &diffs_to_merge {
            let revisions = diff.revisions.as_ref().cloned().unwrap_or_default();
            let revision = revisions.first().or_not_found(
                "revision",
                format!("diff at position {} has no revisions", diff.position),
            )?;
            diff_revisions.push((diff, revision.clone()));
        }
        let first_revision = &diff_revisions.first().unwrap().1;

        let target_sha = self
            .git_client
            .resolve_ref_sha(owner, repo, &get_target_ref(&review.target_branch))
            .await?;
        let merge_commit_sha = if target_sha == first_revision.parent_hash {
            // Fast-forward: target hasn't moved, use the last diff's commit directly
            diff_revisions.last().unwrap().1.commit_hash.clone()
        } else {
            // Target has advanced — attempt to rebase each diff onto the new target
            let mut new_parent_sha = target_sha;
            for (diff, revision) in &diff_revisions {
                let new_sha = self
                    .git_client
                    .cherry_pick_commit(owner, repo, &revision.commit_hash, &new_parent_sha)
                    .await
                    .map_err(|e| match e {
                        crate::error::GitError::MergeConflict(_) => {
                            ReviewError::DiffNotMergeable(format!(
                                "conflict rebasing diff at position {} onto target branch; \
                                 update the review to resolve conflicts before merging",
                                diff.position
                            ))
                        }
                        other => ReviewError::GitError(other),
                    })?;

                self.review_repo
                    .update_revision_sha(revision.id, &new_sha, &new_parent_sha)
                    .await?;

                self.git_client
                    .update_ref(
                        owner,
                        repo,
                        &get_current_ref(&review.id.to_string()[..8], diff.position),
                        &new_sha,
                    )
                    .await?;

                new_parent_sha = new_sha;
            }
            new_parent_sha
        };

        self.git_client
            .update_ref(
                owner,
                repo,
                &get_target_ref(&review.target_branch),
                &merge_commit_sha,
            )
            .await?;

        for (diff, _) in &diff_revisions {
            self.review_repo
                .update_diff(diff.id, Some(DiffStatus::Merged), None)
                .await?;
        }

        let all_merged = diffs
            .iter()
            .all(|d| d.status == DiffStatus::Merged || d.position <= request.position);

        self.review_repo
            .update_review(
                review.id,
                if all_merged {
                    Some(ReviewStatus::Closed)
                } else {
                    None
                },
                None,
                None,
            )
            .await?;

        let updated = self.get_review_by_id(owner, repo, request.number).await?;

        Ok(updated.into())
    }

    async fn merge_review(
        &self,
        request: MergeReviewRequest,
    ) -> Result<ReviewResponse, ReviewError> {
        let owner = request.owner.as_ref();
        let repo = request.repo.as_ref();

        let review = self.get_review_by_id(owner, repo, request.number).await?;
        let diffs = review.diffs.unwrap_or_default();

        let last_position = diffs.iter().map(|d| d.position).max().ok_or_else(|| {
            NotFoundError::new(
                "diff",
                format!("{}/{}/review/{} has no diffs", owner, repo, request.number),
            )
        })?;

        let diff_request = MergeReviewDiffRequest::new(owner, repo, request.number, last_position)?;
        self.merge_review_diff(diff_request).await
    }

    async fn update_review_diff(
        &self,
        request: UpdateReviewDiffRequest,
    ) -> Result<ReviewResponse, ReviewError> {
        let owner = request.owner.as_ref();
        let repo = request.repo.as_ref();

        let review = self.get_review_by_id(owner, repo, request.number).await?;
        let review_number = review.number;

        let diffs = review.diffs.as_ref().or_not_found(
            "diff",
            format!(
                "{}/{}/review/{}/diff/{}",
                owner, repo, review_number, request.position
            ),
        )?;

        let diff = diffs
            .iter()
            .find(|d| d.position == request.position)
            .or_not_found(
                "diff",
                format!(
                    "{}/{}/review/{}/diff/{}",
                    owner, repo, review_number, request.position
                ),
            )?;

        self.review_repo
            .update_diff(diff.id, None, request.message)
            .await?;

        self.review_repo
            .update_review(review.id, None, None, None)
            .await?;

        let updated = self.get_review_by_id(owner, repo, request.number).await?;

        Ok(updated.into())
    }

    async fn add_review_reviewer(
        &self,
        request: AddReviewReviewerReqeuest,
    ) -> Result<ReviewerResponse, ReviewError> {
        let user = self
            .user_repo
            .get(request.user_name.as_ref())
            .await?
            .or_not_found("user", request.user_name.as_ref())?;

        let review = self
            .get_review_by_id(
                request.owner.as_ref(),
                request.repo.as_ref(),
                request.number,
            )
            .await?;

        if user.id == review.author_id {
            return Err(ReviewError::CannotReviewOwnReview(
                request.user_name.as_ref().to_string(),
            ));
        }

        // TODO: add org admin check
        let reviewer = self
            .review_repo
            .add_reviewer(review.id, user.id)
            .await?
            .ok_or_else(|| ConflictError::new("reviewer", request.user_name.as_ref()))?;

        Ok(reviewer.into())
    }

    async fn remove_review_reviewer(
        &self,
        request: RemoveReviewReviewerRequest,
    ) -> Result<(), ReviewError> {
        let user = self
            .user_repo
            .get(request.reviewer_name.as_ref())
            .await?
            .or_not_found("user", request.reviewer_name.as_ref())?;

        let review = self
            .get_review_by_id(
                request.owner.as_ref(),
                request.repo.as_ref(),
                request.number,
            )
            .await?;

        let removed = self.review_repo.remove_reviewer(review.id, user.id).await?;
        if !removed {
            return Err(NotFoundError::new("reviewer", request.reviewer_name.as_ref()).into());
        }

        Ok(())
    }

    async fn update_review_comment(
        &self,
        request: UpdateReviewCommentRequest,
    ) -> Result<ReviewCommentResponse, ReviewError> {
        let updated = self
            .review_repo
            .update_comment(request.comment_id, &request.body)
            .await?;

        Ok(updated.into())
    }

    async fn resolve_review_comment(
        &self,
        request: ResolveReviewCommentRequest,
    ) -> Result<ReviewCommentResponse, ReviewError> {
        let comment = self
            .review_repo
            .get_comment(request.comment_id)
            .await?
            .or_not_found("comment", request.comment_id.to_string())?;

        if comment.parent_id.is_some() {
            return Err(InputError::new(
                "comment",
                "Cannot resolve a reply directly, resolve the parent comment instead",
            )
            .into());
        }

        self.review_repo
            .resolve_comment(request.comment_id, request.resolved)
            .await?;

        let updated = self
            .review_repo
            .get_comment(request.comment_id)
            .await?
            .or_not_found("comment", request.comment_id.to_string())?;

        Ok(updated.into())
    }

    async fn create_review_comments(
        &self,
        request: CreateReviewCommentsRequest,
    ) -> Result<ReviewResponse, ReviewError> {
        let owner = request.owner.as_ref();
        let repo = request.repo.as_ref();

        let review = self.get_review_by_id(owner, repo, request.number).await?;

        let diffs = review.diffs.as_ref().map(|d| d.as_slice()).unwrap_or(&[]);
        let diff = diffs
            .iter()
            .find(|d| d.position == request.position)
            .or_not_found("diff", format!("position {}", request.position))?;

        for comment in request.comments {
            self.review_repo
                .create_comment(
                    review.id,
                    diff.id,
                    comment.revision_id,
                    request.reviewer_id,
                    &comment.body,
                    comment.parent_id,
                    comment.file_path,
                    comment.line_number_start,
                    comment.line_number_end,
                    comment.start_character,
                    comment.end_character,
                    comment.side,
                )
                .await?;
        }

        self.review_repo
            .update_review(review.id, None, None, None)
            .await?;

        let updated = self.get_review_by_id(owner, repo, request.number).await?;

        Ok(updated.into())
    }

    async fn approve_review_diff(
        &self,
        request: ApproveReviewDiffRequest,
    ) -> Result<ReviewResponse, ReviewError> {
        let owner = request.owner.as_ref();
        let repo = request.repo.as_ref();

        let review = self.get_review_by_id(owner, repo, request.number).await?;

        let reviewers = review.reviewers.as_ref().map(|r| r.as_slice()).unwrap_or(&[]);
        if !reviewers.iter().any(|r| r.reviewer_id == request.reviewer_id) {
            return Err(ReviewError::CannotApproveOwnDiff);
        }

        let diffs = review.diffs.as_ref().map(|d| d.as_slice()).unwrap_or(&[]);
        let diff = diffs
            .iter()
            .find(|d| d.position == request.position)
            .or_not_found("diff", format!("position {}", request.position))?;

        let revisions = diff.revisions.as_ref().map(|r| r.as_slice()).unwrap_or(&[]);
        let revision = revisions
            .first()
            .or_not_found("revision", format!("diff at position {}", request.position))?;

        self.review_repo
            .create_verdict(diff.id, revision.id, request.reviewer_id, Verdict::Approved)
            .await?;

        self.review_repo
            .update_review(review.id, None, None, None)
            .await?;

        let updated = self.get_review_by_id(owner, repo, request.number).await?;

        Ok(updated.into())
    }

    async fn reject_review_diff(
        &self,
        request: RejectReviewDiffRequest,
    ) -> Result<ReviewResponse, ReviewError> {
        let owner = request.owner.as_ref();
        let repo = request.repo.as_ref();

        let review = self.get_review_by_id(owner, repo, request.number).await?;

        let reviewers = review.reviewers.as_ref().map(|r| r.as_slice()).unwrap_or(&[]);
        if !reviewers.iter().any(|r| r.reviewer_id == request.reviewer_id) {
            return Err(ReviewError::CannotRejectOwnDiff);
        }

        let diffs = review.diffs.as_ref().map(|d| d.as_slice()).unwrap_or(&[]);
        let diff = diffs
            .iter()
            .find(|d| d.position == request.position)
            .or_not_found("diff", format!("position {}", request.position))?;

        let revisions = diff.revisions.as_ref().map(|r| r.as_slice()).unwrap_or(&[]);
        let revision = revisions
            .first()
            .or_not_found("revision", format!("diff at position {}", request.position))?;

        self.review_repo
            .create_verdict(diff.id, revision.id, request.reviewer_id, Verdict::Rejected)
            .await?;

        self.review_repo
            .update_review(review.id, None, None, None)
            .await?;

        let updated = self.get_review_by_id(owner, repo, request.number).await?;

        Ok(updated.into())
    }
}
