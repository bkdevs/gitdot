"use client";

import type { DiffStatus, RevisionResource } from "gitdot-api";
import { useState } from "react";
import { useUserContext } from "@/(main)/context/user";
import { cn } from "@/util";
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

  if (status === "merged" || review.status === "closed") return null;

  const isAuthor = user?.id === review.author?.id;
  const isReviewer = review.reviewers.some((r) => r.user?.id === user?.id);
  const draftCount = activeDiffDraftComments.length;

  if (isAuthor && review.status === "draft") {
    return <AuthorActions draftCount={draftCount} />;
  }

  if (isReviewer && review.status === "open") {
    const userVerdict = revision?.verdicts.find(
      (v) => v.reviewer_id === user?.id,
    );
    return <ReviewerActions draftCount={draftCount} />;
  }

  return null;
}

function ReviewButton({ draftCount }: { draftCount: number }) {
  const [open, setOpen] = useState(false);
  return (
    <>
      <button
        type="button"
        onClick={() => setOpen(true)}
        className="text-xs font-mono px-2.5 py-1 rounded-xs border border-border bg-background w-full underline decoration-transparent hover:decoration-current hover:bg-accent transition-all duration-200"
      >
        {draftCount > 0 ? `Review (${draftCount})` : "Review"}
      </button>
      <ReviewDiffReviewDialog open={open} setOpen={setOpen} />
    </>
  );
}

function MergeButton() {
  return (
    <button
      type="button"
      disabled
      className="text-xs font-mono px-2.5 py-1 rounded-xs border border-primary w-full bg-primary text-primary-foreground opacity-50 cursor-not-allowed"
    >
      Merge
    </button>
  );
}

function AuthorActions({ draftCount }: { draftCount: number }) {
  return (
    <div className="flex flex-col gap-1 w-full">
      <MergeButton />
      <ReviewButton draftCount={draftCount} />
    </div>
  );
}

function ReviewerActions({ draftCount }: { draftCount: number }) {
  return (
    <div className="flex flex-col gap-1 w-full">
      <MergeButton />
      <ReviewButton draftCount={draftCount} />
    </div>
  );
}
