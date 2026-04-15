"use client";

import type { ReviewResource } from "gitdot-api";
import { MarkdownBody } from "@/(main)/[owner]/[repo]/ui/markdown/markdown-body";
import { ReviewFiles } from "./review-files";
import { ReviewReviewers } from "./review-reviewers";

function ReviewRationale({ description }: { description: string }) {
  return (
    <section className="flex flex-col gap-1.5">
      <h2 className="text-xs font-semibold uppercase tracking-wider text-muted-foreground">
        Rationale
      </h2>
      <div className="[&>p:last-child]:mb-0">
        <MarkdownBody content={description} />
      </div>
    </section>
  );
}

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
      <ReviewRationale description={review.description} />
      {/*<ReviewFiles />*/}
      <ReviewReviewers
        owner={owner}
        repo={repo}
        number={review.number}
        reviewers={review.reviewers}
        diffs={review.diffs}
      />
    </div>
  );
}
