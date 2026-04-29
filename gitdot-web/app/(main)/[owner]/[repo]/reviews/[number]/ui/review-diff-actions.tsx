"use client";

import type { DiffStatus, RevisionResource } from "gitdot-api";
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
  const draftCount = activeDiffDraftComments.length;

  if (isAuthor && review.status === "draft") {
    return (
      <AuthorActions published={status === "open"} draftCount={draftCount} />
    );
  }

  if (isReviewer && review.status === "open") {
    const userVerdict = revision?.verdicts.find(
      (v) => v.reviewer_id === user?.id,
    );
    return (
      <ReviewerActions
        approved={userVerdict?.verdict === "approved"}
        draftCount={draftCount}
      />
    );
  }

  return null;
}

function ReviewButton({ draftCount }: { draftCount: number }) {
  return (
    <button
      type="button"
      className="text-xs font-mono px-2.5 py-1 rounded-xs border border-border bg-background w-full underline decoration-transparent hover:decoration-current hover:bg-accent transition-all duration-200"
    >
      {draftCount > 0 ? `Review (${draftCount})` : "Review"}
    </button>
  );
}

function AuthorActions({
  published,
  draftCount,
}: {
  published: boolean;
  draftCount: number;
}) {
  return (
    <div className="flex flex-col gap-1 w-full">
      <button
        type="button"
        disabled={published}
        className={cn(
          "text-xs font-mono px-2.5 py-1 underline decoration-transparent hover:decoration-current rounded-xs border border-primary w-full disabled:cursor-not-allowed bg-primary text-primary-foreground",
          published ? "opacity-50" : "hover:bg-primary/90",
        )}
      >
        {published ? "Approved" : "Approve"}
      </button>
      <ReviewButton draftCount={draftCount} />
    </div>
  );
}

function ReviewerActions({
  approved,
  draftCount,
}: {
  approved: boolean;
  draftCount: number;
}) {
  return (
    <div className="flex flex-col gap-1 w-full">
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
      <ReviewButton draftCount={draftCount} />
    </div>
  );
}
