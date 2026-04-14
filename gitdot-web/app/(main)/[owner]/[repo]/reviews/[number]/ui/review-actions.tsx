"use client";

import type { ReviewCommentResource, ReviewResource } from "gitdot-api";
import { Suspense, use } from "react";
import type { ResourcePromisesType } from "@/(main)/[owner]/[repo]/resources";
import { UserImage } from "@/(main)/[owner]/ui/user-image";
import { useRightSidebar } from "@/(main)/hooks/use-sidebar";
import { timeAgo } from "@/util";
import type { Resources } from "../layout";

type ResourcePromises = ResourcePromisesType<Resources>;

export function ReviewActions({ promises }: { promises: ResourcePromises }) {
  const open = useRightSidebar();
  if (!open) return null;

  return (
    <div className="w-64 h-full border-l flex flex-col overflow-auto">
      <Suspense>
        <ReviewActionsContent promises={promises} />
      </Suspense>
    </div>
  );
}

function ReviewActionsContent({ promises }: { promises: ResourcePromises }) {
  const review = use(promises.review);
  if (!review) return null;

  const sorted = [...review.comments].sort(
    (a, b) =>
      new Date(b.created_at).getTime() - new Date(a.created_at).getTime(),
  );

  return (
    <>
      <ReviewActionButtons />
      <ReviewReviewers review={review} />
      <ReviewComments comments={sorted} />
    </>
  );
}

function ReviewActionButtons() {
  return (
    <div className="flex border-b border-border shrink-0">
      <button
        type="button"
        className="flex-1 h-8 text-xs hover:bg-accent/50 cursor-default select-none border-r border-border"
      >
        Comment
      </button>
      <button
        type="button"
        className="flex-1 h-8 text-xs hover:bg-accent/50 cursor-default select-none border-r border-border"
      >
        Approve
      </button>
      <button
        type="button"
        className="flex-1 h-8 text-xs bg-primary text-primary-foreground hover:bg-primary/90 cursor-default select-none"
      >
        Merge
      </button>
    </div>
  );
}

function ReviewReviewers({ review }: { review: ReviewResource }) {
  return (
    <div className="flex flex-col border-b">
      <span className="px-2 pt-2 pb-1.5 text-xs font-semibold text-muted-foreground uppercase tracking-wide">
        Reviewers
      </span>
      {review.reviewers.length === 0 ? (
        <span className="px-2 pb-2 text-xs text-muted-foreground">None</span>
      ) : (
        <div className="flex flex-col pb-1">
          {review.reviewers.map((r) => (
            <div
              key={r.id}
              className="flex items-center gap-1.5 px-2 py-1 text-xs text-muted-foreground"
            >
              <UserImage userId={r.user?.id} px={16} />
              <span>{r.user?.name ?? "unknown"}</span>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}

function ReviewComments({ comments }: { comments: ReviewCommentResource[] }) {
  if (comments.length === 0) return null;

  return (
    <div className="flex flex-col">
      <span className="px-2 pt-2 text-xs font-semibold text-muted-foreground uppercase tracking-wide">
        Comments
      </span>
      {comments.map((comment) => (
        <div
          key={comment.id}
          className="flex flex-col gap-1 px-2 py-2 border-b"
        >
          <div className="flex items-center gap-1 text-xs text-muted-foreground">
            <UserImage userId={comment.author?.id} px={14} />
            <span className="truncate">
              {comment.author?.name ?? "unknown"}
            </span>
            <span className="ml-auto shrink-0">
              {timeAgo(new Date(comment.created_at))}
            </span>
          </div>
          {comment.file_path && (
            <span className="text-xs font-mono text-muted-foreground truncate">
              {comment.file_path}
              {comment.line_number_start != null &&
                `:${comment.line_number_start}`}
            </span>
          )}
          <p className="text-xs leading-relaxed line-clamp-4">{comment.body}</p>
          {comment.resolved && (
            <span className="text-xs text-green-600">resolved</span>
          )}
        </div>
      ))}
    </div>
  );
}
