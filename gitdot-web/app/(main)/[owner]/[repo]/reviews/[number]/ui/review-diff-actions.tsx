"use client";

import type { DiffStatus, RevisionResource } from "gitdot-api";
import { useState } from "react";
import { useUserContext } from "@/(main)/context/user";
import { cn } from "@/util";
import { useTypewriter } from "@/hooks/use-typewriter";
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
      {isAuthor && review.status === "draft" ? (
        <>
          <PublishButton />
          <ReviewButton draftCount={draftCount} disabled={status === "merged"} />
        </>
      ) : (
        <>
          <MergeButton disabled={!canAct} />
          <ReviewButton draftCount={draftCount} disabled={!canAct} />
        </>
      )}
    </div>
  );
}

function PublishButton() {
  const { activeDiff, publishActiveDiff } = useReviewContext();
  const [pending, setPending] = useState(false);
  const isPublished = activeDiff.status !== "draft";
  const typewritten = useTypewriter(pending ? "Publishing..." : "", 35);

  return (
    <button
      type="button"
      disabled={pending || isPublished}
      onClick={async () => {
        setPending(true);
        await publishActiveDiff();
        setPending(false);
      }}
      className={cn(
        "text-xs font-mono px-2.5 py-1 rounded-xs border border-primary w-full bg-primary text-primary-foreground underline decoration-transparent hover:decoration-current transition-all duration-200 disabled:cursor-not-allowed disabled:hover:decoration-transparent",
        isPublished && !pending && "opacity-50",
      )}
    >
      {pending ? (typewritten || " ") : isPublished ? "Published" : "Publish"}
    </button>
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
