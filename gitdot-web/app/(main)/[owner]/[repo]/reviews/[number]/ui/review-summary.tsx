"use client";

import type { ReviewResource } from "gitdot-api";
import { ReviewActions } from "./review-actions";
import { ReviewSummaryBody } from "./review-summary-body";
import { ReviewSummaryHeader } from "./review-summary-header";

export function ReviewSummary({ review }: { review: ReviewResource }) {
  return (
    <div className="flex flex-col w-full flex-1 min-h-0 overflow-hidden">
      <ReviewSummaryHeader review={review} />
      <ReviewSummaryBody review={review} />
      <ReviewActions review={review} />
    </div>
  );
}
