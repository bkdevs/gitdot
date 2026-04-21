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
      <div className="flex flex-col gap-4 -ml-2">
        {sorted.map((comment) => (
            <ReviewSummaryComment key={comment.id} comment={comment} />
          ))}
      </div>
    </section>
  );
}

function formatLocation(
  lineStart: number,
  lineEnd: number | null,
  charStart: number | null,
  charEnd: number | null,
): string {
  const multiLine = lineEnd != null && lineEnd !== lineStart;
  if (multiLine) return `:${lineStart}-${lineEnd}`;
  if (charStart != null && charEnd != null) return `:${lineStart}:${charStart}-${charEnd}`;
  return `:${lineStart}`;
}

function ReviewSummaryComment({ comment }: { comment: ReviewCommentResource }) {
  const name = comment.author?.name ?? comment.author_id;
  return (
    <div className="border-l border-transparent hover:border-foreground pl-1.5 transition-colors duration-200 cursor-pointer">
      <div className="flex flex-col gap-0.5">
        <div className="flex items-center gap-1">
          <AvatarBeam name={name} size={14} />
          <span className="text-xs text-muted-foreground">{name}</span>
          {comment.file_path && (
            <span className="text-xs text-muted-foreground">
              {comment.file_path.split("/").pop()}
              {comment.line_number_start != null &&
                formatLocation(comment.line_number_start, comment.line_number_end, comment.start_character, comment.end_character)}
            </span>
          )}
          <span className="ml-auto text-xs text-muted-foreground">
            {timeAgo(new Date(comment.created_at))}
          </span>
        </div>
        <span className="text-sm text-foreground">{comment.body}</span>
      </div>
    </div>
  );
}
