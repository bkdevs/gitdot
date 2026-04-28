"use client";

import type { ReviewCommentResource } from "gitdot-api";
import { Send } from "lucide-react";
import { useMemo, useState } from "react";
import { pluralize } from "@/util";
import { useReviewContext } from "../context";
import { ReviewCommentThread } from "./review-comment-thread";

export function ReviewSummaryComments() {
  const {
    activeDiffComments,
    activeDiffDraftComments,
    activeDiff,
    publishActiveDiffComments,
  } = useReviewContext();
  const [pending, setPending] = useState(false);

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

    return roots.map((root) => [root, ...(replies.get(root.id) ?? [])]);
  }, [activeDiffComments]);

  return (
    <section className="flex flex-col gap-0.5">
      <h2 className="text-xs font-semibold uppercase tracking-wider text-muted-foreground">
        Comments on Diff {activeDiff.position}
      </h2>
      {threads.length === 0 ? (
        <span className="text-xs text-muted-foreground pt-1">
          no comments yet
        </span>
      ) : (
        <div className="flex flex-col gap-2">
          {threads.map((thread) => (
            <ReviewCommentThread key={thread[0].id} comments={thread} />
          ))}
        </div>
      )}
      {activeDiffDraftComments.length > 0 && (
        <div className="flex justify-start pt-1.5">
          <button
            type="button"
            onClick={async () => {
              setPending(true);
              await publishActiveDiffComments();
              setPending(false);
            }}
            disabled={pending}
            className="flex items-center gap-1 text-xs text-muted-foreground hover:text-foreground hover:underline decoration-current transition-colors cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <Send className="size-3" />
            {`Publish ${pluralize(activeDiffDraftComments.length, "comment")}`}
          </button>
        </div>
      )}
    </section>
  );
}
