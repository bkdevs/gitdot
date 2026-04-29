"use client";

import type { ReviewResource } from "gitdot-api";
import { ReviewSummaryRationale } from "./review-summary-rationale";
import { ReviewSummaryReviewers } from "./review-summary-reviewers";

export function ReviewSummaryBody({ review }: { review: ReviewResource }) {
  return (
    <div className="flex-1 overflow-y-auto scrollbar-thin px-4 py-4 flex flex-col gap-8">
      <ReviewSummaryRationale description={review.description} />
      <ReviewSummaryReviewers />
    </div>
  );
}
