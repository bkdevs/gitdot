"use client";

import type { ReviewCommentResource } from "gitdot-api";
import { AvatarBeam } from "@/ui/avatar-beam";
import { timeAgo } from "@/util";
import { useReviewContext } from "../context";

export function ReviewSummaryComments() {
  const { comments } = useReviewContext();
  return (
    <section className="flex flex-col gap-1.5">
      <h2 className="text-xs font-semibold uppercase tracking-wider text-muted-foreground">
        Comments
      </h2>
      <div className="flex flex-col gap-4 -ml-2">
        {comments.map((comment) => (
          <ReviewSummaryComment key={comment.id} comment={comment} />
        ))}
      </div>
    </section>
  );
}

function ReviewSummaryComment({ comment }: { comment: ReviewCommentResource }) {
  const name = comment.author?.name ?? comment.author_id;
  return (
    <div className="border-l border-transparent hover:border-foreground pl-1.5 transition-colors duration-200 cursor-pointer">
      <div className="flex flex-col gap-0.5">
        <div className="flex items-center gap-1">
          <AvatarBeam name={name} size={14} />
          <span className="text-xs text-muted-foreground">{name}</span>
          <span className="ml-auto text-xs text-muted-foreground">
            {timeAgo(new Date(comment.created_at))}
          </span>
        </div>
        <span className="text-sm text-foreground">{comment.body}</span>
      </div>
    </div>
  );
}
