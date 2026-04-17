"use client";

import type { ReviewResource } from "gitdot-api";
import { ReviewSummaryBody } from "./review-summary-body";
import { ReviewSummaryHeader } from "./review-summary-header";

export function ReviewSummary({
  owner,
  repo,
  review,
}: {
  owner: string;
  repo: string;
  review: ReviewResource;
}) {
  return (
    <div className="flex flex-col w-full h-full overflow-auto">
      <ReviewSummaryHeader review={review} />
      <ReviewSummaryBody owner={owner} repo={repo} review={review} />
    </div>
  );
}
