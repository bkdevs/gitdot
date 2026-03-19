use async_trait::async_trait;

use crate::{
    client::{DiffClient, DifftClient, Git2Client, GitClient},
    dto::{
        AddReviewerRequest, GetReviewDiffRequest, GetReviewRequest, ListReviewsRequest,
        MergeDiffRequest, ProcessReviewRequest, PublishReviewRequest, RemoveReviewerRequest,
        ResolveReviewCommentRequest, ReviewCommentResponse, ReviewDiffResponse, ReviewResponse,
        ReviewerResponse, SubmitAction, SubmitReviewRequest, UpdateReviewCommentRequest,
    },
    error::ReviewError,
    model::{DiffStatus, ReviewStatus, Verdict},
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
    async fn update_review(
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

    async fn submit_review(
        &self,
        request: SubmitReviewRequest,
    ) -> Result<ReviewResponse, ReviewError>;

    async fn update_review_comment(
        &self,
        request: UpdateReviewCommentRequest,
    ) -> Result<ReviewCommentResponse, ReviewError>;

    async fn resolve_review_comment(
        &self,
        request: ResolveReviewCommentRequest,
    ) -> Result<ReviewCommentResponse, ReviewError>;

    async fn merge_diff(&self, request: MergeDiffRequest) -> Result<ReviewResponse, ReviewError>;
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
            .list_reviews(request.owner.as_ref(), request.repo.as_ref(), request.viewer_id)
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

    async fn update_review(
        &self,
        request: ProcessReviewRequest,
    ) -> Result<ReviewResponse, ReviewError> {
        let owner = request.owner.as_ref();
        let repo = request.repo.as_ref();
        let review_number = request
            .review_number
            .ok_or_else(|| ReviewError::InvalidRefName("missing review number".to_string()))?
            as i32;

        let review = self
            .review_repo
            .get_review(owner, repo, review_number)
            .await?
            .ok_or_else(|| {
                ReviewError::ReviewNotFound(format!("{}/{}/review/{}", owner, repo, review_number))
            })?;

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
                let latest_revision = revisions.first().ok_or_else(|| {
                    ReviewError::RevisionNotFound(format!(
                        "{}/{}/review/{}/diff/{} has no revisions",
                        owner, repo, review_number, diff_position
                    ))
                })?;

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
                            &get_revision_ref(review_number, diff_position, latest_revision.number),
                            &commit.sha,
                        )
                        .await?;

                    self.git_client
                        .update_ref(
                            owner,
                            repo,
                            &get_current_ref(review_number, diff_position),
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
                            &get_revision_ref(review_number, diff_position, new_revision_number),
                            &commit.sha,
                        )
                        .await?;

                    self.git_client
                        .update_ref(
                            owner,
                            repo,
                            &get_current_ref(review_number, diff_position),
                            &commit.sha,
                        )
                        .await?;

                    self.review_repo
                        .update_diff_status(existing_diff.id, DiffStatus::Open)
                        .await?;
                }
            } else {
                // New diff position — create diff + revision
                let commit_title = commit
                    .message
                    .lines()
                    .next()
                    .unwrap_or(&commit.message)
                    .to_string();

                let diff = self
                    .review_repo
                    .create_diff(review.id, diff_position, &commit_title, &commit_title)
                    .await?;

                self.review_repo
                    .create_revision(diff.id, 1, &commit.sha, &previous_sha)
                    .await?;

                self.git_client
                    .create_ref(
                        owner,
                        repo,
                        &get_revision_ref(review_number, diff_position, 1),
                        &commit.sha,
                    )
                    .await?;

                self.git_client
                    .create_ref(
                        owner,
                        repo,
                        &get_current_ref(review_number, diff_position),
                        &commit.sha,
                    )
                    .await?;
            }

            previous_sha = commit.sha.clone();
        }

        self.git_client
            .update_ref(owner, repo, &get_head_ref(review_number), &request.new_sha)
            .await?;

        self.review_repo.touch_review(review.id).await?;

        let updated = self
            .review_repo
            .get_review(owner, repo, review_number)
            .await?
            .ok_or_else(|| {
                ReviewError::ReviewNotFound(format!("{}/{}/review/{}", owner, repo, review_number))
            })?;

        Ok(updated.into())
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
            let diff = self
                .diff_client
                .diff_files(left.as_ref(), right.as_ref())
                .await?;
            files.push(diff);
        }

        Ok(ReviewDiffResponse { files })
    }

    async fn submit_review(
        &self,
        request: SubmitReviewRequest,
    ) -> Result<ReviewResponse, ReviewError> {
        let owner = request.owner.as_ref();
        let repo = request.repo.as_ref();

        let review = self
            .review_repo
            .get_review(owner, repo, request.number)
            .await?
            .ok_or_else(|| {
                ReviewError::ReviewNotFound(format!("{}/{}/review/{}", owner, repo, request.number))
            })?;

        if review.status != ReviewStatus::InProgress {
            return Err(ReviewError::ReviewNotPublishable(
                "Review must be in progress to submit a review".to_string(),
            ));
        }

        let diffs = review.diffs.as_ref().map(|d| d.as_slice()).unwrap_or(&[]);
        let diff = diffs
            .iter()
            .find(|d| d.position == request.position)
            .ok_or_else(|| ReviewError::DiffNotFound(format!("position {}", request.position)))?;

        let revisions = diff.revisions.as_ref().map(|r| r.as_slice()).unwrap_or(&[]);
        let latest_revision = revisions
            .first()
            .ok_or_else(|| ReviewError::RevisionNotFound("No revisions found".to_string()))?;

        // Create verdict and update diff status
        if request.action != SubmitAction::Comment {
            let verdict = match request.action {
                SubmitAction::Approve => Verdict::Approved,
                SubmitAction::RequestChanges => Verdict::ChangesRequested,
                SubmitAction::Comment => unreachable!(),
            };
            self.review_repo
                .create_verdict(diff.id, latest_revision.id, request.reviewer_id, verdict)
                .await?;

            // Update diff status: once approved, stays approved.
            // Otherwise, reflect the new action directly.
            if diff.status != DiffStatus::Approved {
                let new_status = match request.action {
                    SubmitAction::Approve => DiffStatus::Approved,
                    SubmitAction::RequestChanges => DiffStatus::ChangesRequested,
                    SubmitAction::Comment => unreachable!(),
                };
                self.review_repo
                    .update_diff_status(diff.id, new_status)
                    .await?;
            }
        }

        for comment in &request.comments {
            self.review_repo
                .create_comment(
                    review.id,
                    diff.id,
                    latest_revision.id,
                    request.reviewer_id,
                    &comment.body,
                    comment.parent_id,
                    comment.file_path.clone(),
                    comment.line_number_start,
                    comment.line_number_end,
                    comment.side.clone(),
                )
                .await?;
        }

        self.review_repo.touch_review(review.id).await?;

        let updated = self
            .review_repo
            .get_review(owner, repo, request.number)
            .await?
            .ok_or_else(|| {
                ReviewError::ReviewNotFound(format!("{}/{}/review/{}", owner, repo, request.number))
            })?;

        Ok(updated.into())
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
            .ok_or_else(|| ReviewError::CommentNotFound(request.comment_id.to_string()))?;

        if comment.parent_id.is_some() {
            return Err(ReviewError::InvalidComment(
                "Cannot resolve a reply directly, resolve the parent comment instead".to_string(),
            ));
        }

        self.review_repo
            .resolve_comment(request.comment_id, request.resolved)
            .await?;

        let updated = self
            .review_repo
            .get_comment(request.comment_id)
            .await?
            .ok_or_else(|| ReviewError::CommentNotFound(request.comment_id.to_string()))?;

        Ok(updated.into())
    }

    async fn merge_diff(&self, request: MergeDiffRequest) -> Result<ReviewResponse, ReviewError> {
        let owner = request.owner.as_ref();
        let repo = request.repo.as_ref();

        let review = self
            .review_repo
            .get_review(owner, repo, request.number)
            .await?
            .ok_or_else(|| {
                ReviewError::ReviewNotFound(format!("{}/{}/review/{}", owner, repo, request.number))
            })?;

        if review.status != ReviewStatus::InProgress {
            return Err(ReviewError::DiffNotMergeable(
                "review must be in progress to merge diffs".to_string(),
            ));
        }

        let diffs = review.diffs.unwrap_or_default();
        let diffs_to_merge: Vec<_> = diffs
            .iter()
            .filter(|d| d.position <= request.position && d.status != DiffStatus::Merged)
            .collect();

        if diffs_to_merge.is_empty() {
            return Err(ReviewError::DiffNotFound(format!(
                "no open diffs found at or before position {}",
                request.position
            )));
        }

        for diff in &diffs_to_merge {
            if diff.status != DiffStatus::Approved {
                return Err(ReviewError::DiffNotMergeable(format!(
                    "diff at position {} has status '{}', expected 'approved'",
                    diff.position,
                    serde_json::to_string(&diff.status)
                        .unwrap_or_default()
                        .trim_matches('"')
                )));
            }
        }

        let mut diff_revisions = Vec::new();
        for diff in &diffs_to_merge {
            let revisions = diff.revisions.as_ref().cloned().unwrap_or_default();
            let revision = revisions.first().ok_or_else(|| {
                ReviewError::RevisionNotFound(format!(
                    "diff at position {} has no revisions",
                    diff.position
                ))
            })?;
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
                        &get_current_ref(review.number, diff.position),
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
                .update_diff_status(diff.id, DiffStatus::Merged)
                .await?;
        }

        let all_merged = diffs
            .iter()
            .all(|d| d.status == DiffStatus::Merged || d.position <= request.position);
        if all_merged {
            self.review_repo.close_review(review.id).await?;
        }

        self.review_repo.touch_review(review.id).await?;

        let updated = self
            .review_repo
            .get_review(owner, repo, request.number)
            .await?
            .ok_or_else(|| {
                ReviewError::ReviewNotFound(format!("{}/{}/review/{}", owner, repo, request.number))
            })?;

        Ok(updated.into())
    }
}
