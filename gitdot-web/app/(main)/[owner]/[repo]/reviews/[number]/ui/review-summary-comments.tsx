"use client";

import { useReviewContext } from "../context";
import { ReviewCommentThread } from "./review-comment-thread";

export function ReviewSummaryComments() {
  const { activeDiffCommentThreads, activeDiff } = useReviewContext();
  const threads = activeDiffCommentThreads;

  return (
    <section className="flex flex-col gap-1.5">
      <h2 className="text-xs font-semibold uppercase tracking-wider text-muted-foreground">
        Comments on Diff {activeDiff.position}
      </h2>
      {threads.length === 0 ? (
        <span className="text-xs text-muted-foreground">no comments yet</span>
      ) : (
        <div className="flex flex-col gap-0">
          {threads.map((thread) => (
            <ReviewCommentThread key={thread[0].id} comments={thread} />
          ))}
        </div>
      )}
    </section>
  );
}
