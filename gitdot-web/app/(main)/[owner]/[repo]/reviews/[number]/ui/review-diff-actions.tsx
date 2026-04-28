"use client";

import type {
  DiffStatus,
  ReviewCommentResource,
  RevisionResource,
} from "gitdot-api";
import { useUserContext } from "@/(main)/context/user";
import { cn } from "@/util";
import { useReviewContext } from "../context";

export function ReviewDiffActions({
  position,
  status,
  revision,
}: {
  position: number;
  status: DiffStatus;
  revision: RevisionResource | undefined;
}) {
  const { review, activeDiffDraftComments } = useReviewContext();
  const { user } = useUserContext();

  if (status === "merged" || review.status === "closed") return null;

  const isAuthor = user?.id === review.author?.id;
  const isReviewer = review.reviewers.some((r) => r.user?.id === user?.id);

  if (isAuthor) {
    return (
      <AuthorActions
        position={position}
        isDraft={review.status === "draft"}
        approved={status === "open"}
        activeDiffDraftComments={activeDiffDraftComments}
      />
    );
  }

  if (isReviewer && review.status === "open") {
    return (
      <ReviewerActions
        position={position}
        approved={
          revision?.verdicts.some((v) => v.reviewer_id === user?.id) ?? false
        }
        activeDiffDraftComments={activeDiffDraftComments}
      />
    );
  }

  return null;
}

function AuthorActions({
  isDraft,
  approved,
  activeDiffDraftComments,
}: {
  position: number;
  isDraft: boolean;
  approved: boolean;
  activeDiffDraftComments: ReviewCommentResource[];
}) {
  return (
    <div className="flex flex-col gap-1 w-full">
      {isDraft ? (
        <>
          <ApproveButton approved={approved} />
          <RejectButton />
        </>
      ) : (
        <CommentButton />
      )}
    </div>
  );
}

function ReviewerActions({
  approved,
  activeDiffDraftComments,
}: {
  position: number;
  approved: boolean;
  activeDiffDraftComments: ReviewCommentResource[];
}) {
  return (
    <div className="flex flex-col gap-1 w-full">
      <ApproveButton approved={approved} />
      <RejectButton />
    </div>
  );
}

function ApproveButton({ approved }: { approved: boolean }) {
  return (
    <button
      type="button"
      disabled={approved}
      className={cn(
        "text-xs font-mono px-2.5 py-1 underline decoration-transparent hover:decoration-current rounded-xs border border-primary w-full disabled:cursor-not-allowed bg-primary text-primary-foreground",
        approved ? "opacity-50" : "hover:bg-primary/90",
      )}
    >
      {approved ? "Approved" : "Approve"}
    </button>
  );
}

function RejectButton() {
  return (
    <button
      type="button"
      className="text-xs font-mono px-2.5 py-1 rounded-xs border border-border bg-background hover:bg-accent w-full transition-all duration-200"
    >
      Reject
    </button>
  );
}

function CommentButton() {
  return (
    <button
      type="button"
      className="text-xs font-mono px-2.5 py-1 rounded-xs border border-border bg-background hover:bg-accent w-full transition-all duration-200"
    >
      Comment
    </button>
  );
}
