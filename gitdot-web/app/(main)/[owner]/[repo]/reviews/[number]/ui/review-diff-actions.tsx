"use client";

import type { DiffStatus, RevisionResource } from "gitdot-api";
import { useState } from "react";
import { useUserContext } from "@/(main)/context/user";
import { cn } from "@/util";
import { timeAgo } from "@/util/date";
import { useReviewContext } from "../context";

export function ReviewDiffMetadata({
  revision,
}: {
  revision: RevisionResource | undefined;
}) {
  if (!revision) return null;

  return (
    <div className="flex flex-row gap-4">
      <div className="flex flex-col gap-0.5">
        <span
          className="text-muted-foreground/50 uppercase tracking-wide"
          style={{ fontSize: "10px" }}
        >
          Revision
        </span>
        <span className="text-xs font-mono text-muted-foreground">
          #{revision.number}
        </span>
      </div>
      <div className="flex flex-col gap-0.5">
        <span
          className="text-muted-foreground/50 uppercase tracking-wide"
          style={{ fontSize: "10px" }}
        >
          Commit
        </span>
        <span className="text-xs font-mono text-muted-foreground">
          {revision.commit_hash.slice(0, 7)}
        </span>
      </div>
      <div className="flex flex-col gap-0.5">
        <span
          className="text-muted-foreground/50 uppercase tracking-wide"
          style={{ fontSize: "10px" }}
        >
          Authored
        </span>
        <span className="text-xs font-mono text-muted-foreground">
          {timeAgo(new Date(revision.created_at))}
        </span>
      </div>
    </div>
  );
}

export function ReviewDiffActions({
  position,
  status,
  revision,
}: {
  position: number;
  status: DiffStatus;
  revision: RevisionResource | undefined;
}) {
  const { review, reviewDiff } = useReviewContext();
  const { user } = useUserContext();

  const isAuthor = user?.id === review.author?.id;

  if (status === "merged" || review.status === "closed") return null;
  if (review.status === "in_progress" && isAuthor) return null;

  const approved =
    review.status === "draft"
      ? status === "open"
      : revision?.verdicts.some((v) => v.reviewer_id === user?.id) ?? false;

  return (
    <div className="flex flex-col gap-1 w-full">
      <ApproveButton
        approved={approved}
        onApprove={async () => {
          await reviewDiff(position, { action: "approve", comments: [] });
        }}
      />
      <ReviewButton />
    </div>
  );
}

function ApproveButton({
  approved,
  onApprove,
}: {
  approved: boolean;
  onApprove: () => Promise<void>;
}) {
  const [isPending, setIsPending] = useState(false);

  return (
    <button
      type="button"
      disabled={approved || isPending}
      onClick={async () => {
        setIsPending(true);
        await onApprove();
        setIsPending(false);
      }}
      className={cn(
        "text-xs font-mono px-2.5 py-1 underline decoration-transparent hover:decoration-current rounded-xs border border-primary w-full disabled:cursor-not-allowed bg-primary text-primary-foreground",
        approved || isPending ? "opacity-50" : "hover:bg-primary/90",
      )}
    >
      {approved ? "Approved" : "Approve"}
    </button>
  );
}

function ReviewButton() {
  return (
    <button
      type="button"
      className="text-xs font-mono px-2.5 py-1 rounded-xs border border-border bg-background hover:bg-accent w-full transition-all duration-200"
    >
      Review
    </button>
  );
}
