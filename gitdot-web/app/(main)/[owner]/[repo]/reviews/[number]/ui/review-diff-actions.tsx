"use client";

import type { DiffStatus, ReviewResource, RevisionResource } from "gitdot-api";
import { useState } from "react";
import { reviewDiffAction } from "@/actions/review";
import { cn } from "@/util";
import { timeAgo } from "@/util/date";

export function ReviewDiffActions({
  owner,
  repo,
  review,
  position,
  status,
  revision,
}: {
  owner: string;
  repo: string;
  review: ReviewResource;
  position: number;
  status: DiffStatus;
  revision: RevisionResource | undefined;
}) {
  if (status === "merged" || review.status === "closed") return null;

  return (
    <div className="shrink-0 flex flex-col justify-between items-end self-stretch gap-4 pb-2">
      {revision && (
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
      )}
      <div className="flex flex-col gap-1 w-full">
        <ApproveButton
          onApprove={async () => {
            await reviewDiffAction(owner, repo, review.number, position, {
              action: "approve",
              comments: [],
            });
          }}
        />
        <ReviewButton />
      </div>
    </div>
  );
}

function ApproveButton({ onApprove }: { onApprove: () => Promise<void> }) {
  const [approved, setApproved] = useState(false);

  return (
    <button
      type="button"
      disabled={approved}
      onClick={async () => {
        setApproved(true);
        await onApprove();
      }}
      className={cn(
        "text-xs font-mono px-2.5 py-1 underline decoration-transparent hover:decoration-current rounded-xs border border-primary w-full disabled:cursor-not-allowed bg-primary text-primary-foreground",
        approved ? "opacity-50" : "hover:bg-primary/90",
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
