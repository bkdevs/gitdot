"use client";

import type { ReviewCommentResource } from "gitdot-api";
import { ReviewComment } from "./review-comment";

export function ReviewCommentThread({
  comments,
}: {
  comments: ReviewCommentResource[];
}) {
  return (
    <div className="flex flex-col">
      {comments.map((comment) => (
        <ReviewComment key={comment.id} comment={comment} />
      ))}
    </div>
  );
}
