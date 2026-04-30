"use client";

import type { DiffStatus, RevisionResource } from "gitdot-api";
import { useState } from "react";
import { cn } from "@/util";
import { useReviewContext } from "../context";
import { ReviewDiffMergeDialog } from "./review-diff-merge-dialog";
import { ReviewDiffReviewDialog } from "./review-diff-review-dialog";

export function ReviewDiffActions({
  status,
}: {
  position: number;
  status: DiffStatus;
  revision: RevisionResource | undefined;
}) {
  const { review, activeDiff, activeDiffDraftComments } = useReviewContext();

  const isMerged = status === "merged";
  const latestRevision = activeDiff.revisions[activeDiff.revisions.length - 1];
  const isMergeable =
    status === "open" &&
    review.status === "open" &&
    review.reviewers.every((reviewer) =>
      latestRevision?.verdicts.some(
        (v) =>
          v.reviewer_id === reviewer.reviewer_id && v.verdict === "approve",
      ),
    );
  const draftCount = activeDiffDraftComments.length;

  return (
    <div className="flex flex-col gap-1 w-full">
      <MergeButton isMerged={isMerged} isMergeable={isMergeable} />
      <ReviewButton
        draftCount={draftCount}
        disabled={review.status === "closed"}
      />
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

function MergeButton({
  isMerged,
  isMergeable,
}: {
  isMerged: boolean;
  isMergeable: boolean;
}) {
  const [open, setOpen] = useState(false);
  return (
    <>
      <button
        type="button"
        disabled={!isMergeable}
        onClick={() => isMergeable && setOpen(true)}
        className={cn(
          "text-xs font-mono px-2.5 py-1 rounded-xs border border-primary w-full bg-primary text-primary-foreground transition-all duration-200",
          isMerged
            ? "opacity-50 cursor-not-allowed"
            : isMergeable
              ? "underline decoration-transparent hover:decoration-current"
              : "opacity-40 cursor-not-allowed",
        )}
      >
        Merge
      </button>
      <ReviewDiffMergeDialog open={open} setOpen={setOpen} />
    </>
  );
}
