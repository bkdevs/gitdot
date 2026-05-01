"use client";

import type { ReviewCommentResource } from "gitdot-api";
import { useMemo } from "react";
import { useReviewContext } from "../context";
import { ReviewCommentThread } from "./review-comment-thread";

export function ReviewSummaryComments() {
  const { activeDiffComments, activeDiff } = useReviewContext();

  const threads = useMemo(() => {
    const byId = new Map(activeDiffComments.map((c) => [c.id, c]));
    const roots: ReviewCommentResource[] = [];
    const replies = new Map<string, ReviewCommentResource[]>();

    for (const c of activeDiffComments) {
      if (c.parent_id === null || !byId.has(c.parent_id)) {
        roots.push(c);
      } else {
        const bucket = replies.get(c.parent_id) ?? [];
        bucket.push(c);
        replies.set(c.parent_id, bucket);
      }
    }

    roots.sort((a, b) => {
      const pathA = a.file_path ?? "";
      const pathB = b.file_path ?? "";
      if (pathA !== pathB) return pathA.localeCompare(pathB);
      return (
        (a.line_number_start ?? Infinity) - (b.line_number_start ?? Infinity)
      );
    });

    return roots.map((root) => {
      const chain: ReviewCommentResource[] = [root];
      let current = root;
      while (true) {
        const children = replies.get(current.id);
        if (!children?.length) break;
        chain.push(...children);
        current = children[children.length - 1];
      }
      return chain;
    });
  }, [activeDiffComments]);

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
