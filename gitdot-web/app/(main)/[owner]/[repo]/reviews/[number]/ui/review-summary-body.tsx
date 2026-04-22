"use client";

import type { ReviewResource } from "gitdot-api";
import { ReviewSummaryComments } from "./review-summary-comments";
import { ReviewSummaryRationale } from "./review-summary-rationale";
import { ReviewSummaryReviewers } from "./review-summary-reviewers";

export function ReviewSummaryBody({ review }: { review: ReviewResource }) {
  return (
    <div className="flex-1 overflow-y-auto scrollbar-thin px-6 py-4 flex flex-col gap-8">
      <ReviewSummaryRationale description={review.description} />
      <ReviewSummaryReviewers />
      <ReviewSummaryComments />
    </div>
  );
}
