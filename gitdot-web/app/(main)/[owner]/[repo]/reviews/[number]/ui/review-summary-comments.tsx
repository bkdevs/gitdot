"use client";

import type { ReviewCommentResource } from "gitdot-api";
import { useMemo } from "react";
import { AvatarBeam } from "@/ui/avatar-beam";
import { timeAgo } from "@/util";
import { useReviewContext } from "../context";

export function ReviewSummaryComments() {
  const { comments } = useReviewContext();
  const sorted = useMemo(
    () =>
      [...comments].sort(
        (a, b) =>
          new Date(b.created_at).getTime() - new Date(a.created_at).getTime(),
      ),
    [comments],
  );

  return (
    <section className="flex flex-col gap-1.5">
      <h2 className="text-xs font-semibold uppercase tracking-wider text-muted-foreground">
        Comments
      </h2>
      <div className="flex flex-col gap-6">
        {sorted.map((comment) => (
          <ReviewSummaryComment key={comment.id} comment={comment} />
        ))}
      </div>
    </section>
  );
}

function formatLocation(lineStart: number, lineEnd: number | null): string {
  if (lineEnd != null && lineEnd !== lineStart)
    return `:L${lineStart}-${lineEnd}`;
  return `:L${lineStart}`;
}

function ReviewSummaryComment({ comment }: { comment: ReviewCommentResource }) {
  const name = comment.author?.name ?? comment.author_id;
  return (
    <div className="flex flex-col cursor-pointer">
      {comment.file_path && (
        <span className="text-xs font-mono text-muted-foreground truncate">
          {comment.file_path}
          {comment.line_number_start != null &&
            formatLocation(comment.line_number_start, comment.line_number_end)}
        </span>
      )}
      <div className="border-l border-transparent hover:border-black -ml-1.5 pl-1.5 transition-colors duration-200 flex flex-col gap-1 pb-1">
        <span className="text-sm text-foreground">{comment.body}</span>
        <div className="flex items-center gap-1 ml-auto">
          <AvatarBeam name={name} size={14} />
          <span className="text-xs text-muted-foreground">{name}</span>
          <span className="text-xs text-muted-foreground">
            {timeAgo(new Date(comment.created_at))}
          </span>
        </div>
      </div>
    </div>
  );
}
