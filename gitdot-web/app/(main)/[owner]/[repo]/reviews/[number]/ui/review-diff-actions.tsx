"use client";

import type { DiffStatus, RevisionResource } from "gitdot-api";
import { useState } from "react";
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
  const { review } = useReviewContext();
  const { user } = useUserContext();

  if (status === "merged" || review.status === "closed") return null;

  const isAuthor = user?.id === review.author?.id;
  const isReviewer = review.reviewers.some((r) => r.user?.id === user?.id);

  if (isAuthor && review.status === "draft") {
    return <AuthorActions published={status === "open"} />;
  }

  if (isReviewer && review.status === "open") {
    const userVerdict = revision?.verdicts.find(
      (v) => v.reviewer_id === user?.id,
    );
    return (
      <ReviewerActions
        approved={userVerdict?.verdict === "approved"}
        rejected={userVerdict?.verdict === "rejected"}
      />
    );
  }

  return null;
}

function AuthorActions({ published }: { published: boolean }) {
  const { publishActiveDiff } = useReviewContext();
  const [pending, setPending] = useState(false);

  return (
    <div className="flex flex-col gap-1 w-full">
      <button
        type="button"
        disabled={published || pending}
        onClick={async () => {
          setPending(true);
          await publishActiveDiff();
          setPending(false);
        }}
        className={cn(
          "text-xs font-mono px-2.5 py-1 underline decoration-transparent hover:decoration-current rounded-xs border border-primary w-full disabled:cursor-not-allowed bg-primary text-primary-foreground",
          published || pending ? "opacity-50" : "hover:bg-primary/90",
        )}
      >
        {published ? "Approved" : "Approve"}
      </button>
    </div>
  );
}

function ReviewerActions({
  approved,
  rejected,
}: {
  approved: boolean;
  rejected: boolean;
}) {
  const { approveActiveDiff, rejectActiveDiff } = useReviewContext();
  const [approvePending, setApprovePending] = useState(false);
  const [rejectPending, setRejectPending] = useState(false);

  return (
    <div className="flex flex-col gap-1 w-full">
      <button
        type="button"
        disabled={approved || approvePending}
        onClick={async () => {
          setApprovePending(true);
          await approveActiveDiff();
          setApprovePending(false);
        }}
        className={cn(
          "text-xs font-mono px-2.5 py-1 underline decoration-transparent hover:decoration-current rounded-xs border border-primary w-full disabled:cursor-not-allowed bg-primary text-primary-foreground",
          approved || approvePending ? "opacity-50" : "hover:bg-primary/90",
        )}
      >
        {approved ? "Approved" : "Approve"}
      </button>
      <button
        type="button"
        disabled={rejected || rejectPending}
        onClick={async () => {
          setRejectPending(true);
          await rejectActiveDiff();
          setRejectPending(false);
        }}
        className={cn(
          "text-xs font-mono px-2.5 py-1 rounded-xs border border-border bg-background w-full disabled:cursor-not-allowed transition-all duration-200",
          rejected || rejectPending ? "opacity-50" : "hover:bg-accent",
        )}
      >
        {rejected ? "Rejected" : "Reject"}
      </button>
    </div>
  );
}
