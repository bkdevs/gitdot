"use client";

import type { RevisionResource } from "gitdot-api";
import { timeAgo } from "@/util/date";

export function ReviewDiffActions({
  revision,
}: {
  revision: RevisionResource | undefined;
}) {
  return (
    <div className="shrink-0 flex flex-col justify-between items-end self-stretch pb-2">
      {revision && (
        <div className="flex flex-row gap-4">
          <div className="flex flex-col gap-0.5">
            <span className="text-muted-foreground/50 uppercase tracking-wide" style={{ fontSize: "10px" }}>Revision</span>
            <span className="text-xs font-mono text-muted-foreground">#{revision.number}</span>
          </div>
          <div className="flex flex-col gap-0.5">
            <span className="text-muted-foreground/50 uppercase tracking-wide" style={{ fontSize: "10px" }}>Commit</span>
            <span className="text-xs font-mono text-muted-foreground">{revision.commit_hash.slice(0, 7)}</span>
          </div>
          <div className="flex flex-col gap-0.5">
            <span className="text-muted-foreground/50 uppercase tracking-wide" style={{ fontSize: "10px" }}>Authored</span>
            <span className="text-xs font-mono text-muted-foreground">{timeAgo(new Date(revision.created_at))}</span>
          </div>
        </div>
      )}
      <div className="flex flex-col gap-1 w-full">
        <button
          type="button"
          className="text-xs font-mono px-2.5 py-1 bg-primary text-primary-foreground hover:bg-primary/90 hover:underline hover:decoration-current cursor-pointer transition-all duration-200 rounded-xs border border-primary w-full"
        >
          Approve
        </button>
        <button
          type="button"
          className="text-xs font-mono px-2.5 py-1 bg-background hover:bg-muted text-muted-foreground hover:text-foreground cursor-pointer transition-colors rounded-xs border border-border w-full"
        >
          Comment
        </button>
      </div>
    </div>
  );
}
