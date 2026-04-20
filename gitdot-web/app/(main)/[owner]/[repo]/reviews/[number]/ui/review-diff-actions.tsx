"use client";

import type { DiffStatus, ReviewResource, RevisionResource } from "gitdot-api";
import { useState } from "react";
import { judgeDiffAction, mergeDiffAction } from "@/actions/review";
import { useTypewriter } from "@/hooks/use-typewriter";
import { cn } from "@/util";
import { timeAgo } from "@/util/date";

export function ReviewDiffActions({
  owner,
  repo,
  review,
  position,
  status: initialStatus,
  revision,
}: {
  owner: string;
  repo: string;
  review: ReviewResource;
  position: number;
  status: DiffStatus;
  revision: RevisionResource | undefined;
}) {
  const [status, setStatus] = useState<DiffStatus>(initialStatus);

  return (
    <div className="shrink-0 flex flex-col justify-between items-end self-stretch gap-2 pb-2">
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
      {review.status === "draft" && status !== "merged" && (
        <div className="flex flex-col gap-1 w-full">
          {status === "approved" ? (
            <MergeButton
              onMerge={async () => {
                await Promise.all([
                  mergeDiffAction(owner, repo, review.number, position),
                  new Promise((r) => setTimeout(r, 1600)),
                ]);
                setStatus("merged");
              }}
            />
          ) : (
            <ApproveButton
              onApprove={async () => {
                await Promise.all([
                  judgeDiffAction(owner, repo, review.number, position, {
                    verdict: "approve",
                  }),
                  new Promise((r) => setTimeout(r, 1600)),
                ]);
                setStatus("approved");
              }}
            />
          )}
        </div>
      )}
    </div>
  );
}

function ApproveButton({ onApprove }: { onApprove: () => Promise<void> }) {
  const [approving, setApproving] = useState(false);
  const typed = useTypewriter(approving ? "Approving..." : "", 50);

  return (
    <button
      type="button"
      disabled={approving}
      onClick={async () => {
        setApproving(true);
        await onApprove();
      }}
      className={cn(
        "text-xs font-mono px-2.5 py-1 text-primary-foreground underline decoration-transparent hover:decoration-current transition-all duration-200 rounded-xs border border-primary w-full disabled:cursor-not-allowed",
        approving ? "bg-primary/90" : "bg-primary hover:bg-primary/90",
      )}
    >
      {approving ? (
        <span className="inline-block w-[12ch] text-left">{typed || "A"}</span>
      ) : (
        "Approve"
      )}
    </button>
  );
}

function MergeButton({ onMerge }: { onMerge: () => Promise<void> }) {
  const [merging, setMerging] = useState(false);
  const typed = useTypewriter(merging ? "Merging..." : "", 50);

  return (
    <button
      type="button"
      disabled={merging}
      onClick={async () => {
        setMerging(true);
        await onMerge();
      }}
      className={cn(
        "text-xs font-mono px-2.5 py-1 text-primary-foreground underline decoration-transparent hover:decoration-current transition-all duration-200 rounded-xs border border-primary w-full disabled:cursor-not-allowed",
        merging ? "bg-primary/90" : "bg-primary hover:bg-primary/90",
      )}
    >
      {merging ? (
        <span className="inline-block w-[10ch] text-left">{typed || "M"}</span>
      ) : (
        "Merge"
      )}
    </button>
  );
}
