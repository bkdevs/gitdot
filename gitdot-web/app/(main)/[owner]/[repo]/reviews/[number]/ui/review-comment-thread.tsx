"use client";

import type { ReviewCommentResource } from "gitdot-api";
import { cn } from "@/util";
import { useReviewContext } from "../context";
import { ReviewComment } from "./review-comment";

export function ReviewCommentThread({
  comments,
}: {
  comments: ReviewCommentResource[];
}) {
  const { activeComment } = useReviewContext();
  const isThreadActive = comments.some((c) => c.id === activeComment?.id);

  const root = comments[0];
  const fileName = root.file_path?.split("/").at(-1) ?? null;
  const { line_number_start: start, line_number_end: end } = root;
  const lineLabel =
    start != null
      ? end != null && end !== start
        ? `${start}–${end}`
        : `${start}`
      : null;

  return (
    <div
      className={cn(
        "flex flex-col -mx-1.5 px-1.5 rounded transition-colors duration-200",
        isThreadActive && "bg-diff-orange",
      )}
    >
      {fileName && (
        <span className="text-xs font-mono text-muted-foreground">
          {fileName}
          {lineLabel ? `:${lineLabel}` : ""}
        </span>
      )}
      {comments.map((comment) => (
        <ReviewComment key={comment.id} comment={comment} />
      ))}
    </div>
  );
}
