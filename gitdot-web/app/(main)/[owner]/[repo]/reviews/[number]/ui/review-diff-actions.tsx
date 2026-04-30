"use client";

import type { DiffStatus, RevisionResource } from "gitdot-api";
import { useState } from "react";
import { useUserContext } from "@/(main)/context/user";
import { useReviewContext } from "../context";
import { ReviewDiffReviewDialog } from "./review-diff-review-dialog";

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

  const isAuthor = user?.id === review.author?.id;
  const isReviewer = review.reviewers.some((r) => r.user?.id === user?.id);
  const draftCount = activeDiffDraftComments.length;

  const canAct =
    status !== "merged" &&
    review.status !== "closed" &&
    ((isAuthor && review.status === "draft") ||
      (isReviewer && review.status === "open"));

  return (
    <div className="flex flex-col gap-1 w-full">
      <MergeButton disabled={!canAct} />
      <ReviewButton draftCount={draftCount} disabled={!canAct} />
    </div>
  );
}

function ReviewButton({
  draftCount,
  disabled,
}: {
  draftCount: number;
  disabled: boolean;
}) {
  const [open, setOpen] = useState(false);
  return (
    <>
      <button
        type="button"
        onClick={() => !disabled && setOpen(true)}
        disabled={disabled}
        className="text-xs font-mono px-2.5 py-1 rounded-xs border border-border bg-background w-full underline decoration-transparent hover:decoration-current hover:bg-accent transition-all duration-200 disabled:opacity-40 disabled:cursor-not-allowed disabled:hover:bg-background disabled:hover:decoration-transparent"
      >
        {draftCount > 0 ? `Review (${draftCount})` : "Review"}
      </button>
      <ReviewDiffReviewDialog open={open} setOpen={setOpen} />
    </>
  );
}

function MergeButton({ disabled }: { disabled: boolean }) {
  return (
    <button
      type="button"
      disabled={disabled}
      className="text-xs font-mono px-2.5 py-1 rounded-xs border border-primary w-full bg-primary text-primary-foreground opacity-50 cursor-not-allowed"
    >
      Merge
    </button>
  );
}
