"use client";

import type { ReviewResource } from "gitdot-api";
import { ReviewSummaryBody } from "./review-summary-body";
import { ReviewSummaryHeader } from "./review-summary-header";

export function ReviewSummary({ review }: { review: ReviewResource }) {
  return (
    <div className="flex flex-col w-full h-full overflow-auto scrollbar-thin">
      <ReviewSummaryHeader review={review} />
      <ReviewSummaryBody review={review} />
    </div>
  );
}
