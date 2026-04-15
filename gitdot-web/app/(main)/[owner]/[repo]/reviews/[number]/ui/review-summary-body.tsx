"use client";

import type { ReviewResource } from "gitdot-api";
import { ReviewSummaryRationale } from "./review-summary-rationale";
import { ReviewSummaryReviewers } from "./review-summary-reviewers";

export function ReviewSummaryBody({
  owner,
  repo,
  review,
}: {
  owner: string;
  repo: string;
  review: ReviewResource;
}) {
  return (
    <div className="flex-1 overflow-y-auto px-6 pt-4 flex flex-col gap-8">
      <ReviewSummaryRationale description={review.description} />
      <ReviewSummaryReviewers
        owner={owner}
        repo={repo}
        number={review.number}
        reviewers={review.reviewers}
        diffs={review.diffs}
        author={review.author}
      />
    </div>
  );
}
