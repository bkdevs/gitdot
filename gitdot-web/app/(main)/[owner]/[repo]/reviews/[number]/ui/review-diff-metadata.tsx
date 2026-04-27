"use client";

import type { RevisionResource } from "gitdot-api";
import { timeAgo } from "@/util/date";

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
