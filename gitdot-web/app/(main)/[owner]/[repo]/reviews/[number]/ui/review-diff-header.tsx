"use client";

import { Fragment } from "react";
import type { DiffStatus, ReviewStatus } from "gitdot-api";
import { cn, timeAgo } from "@/util";
import { useReviewContext } from "../context";

type DiffView = "code" | "conversation";

function DiffStatusBadge({
  status,
  reviewStatus,
}: {
  status: DiffStatus;
  reviewStatus: ReviewStatus;
}) {
  if (reviewStatus === "draft" && status === "open") {
    return <span className="text-sm font-mono text-green-600">approved</span>;
  }
  if (status === "merged") {
    return (
      <span className="text-sm font-mono text-muted-foreground underline">
        merged
      </span>
    );
  }
  return <span className="text-sm font-mono text-foreground">open</span>;
}

export function ReviewDiffHeader({
  title,
  index,
  status,
  createdAt,
  updatedAt,
  view,
  onViewChange,
}: {
  title: string;
  index: number;
  author: { id: string; name: string } | null;
  status: DiffStatus;
  createdAt: string;
  updatedAt: string;
  view: DiffView;
  onViewChange: (v: DiffView) => void;
}) {
  const { review } = useReviewContext();

  return (
    <div className="sticky top-0 z-10 shrink-0 h-16 flex items-stretch border-b border-border bg-background pl-2 min-w-0">
      <div className="flex flex-col min-w-0 flex-1">
        <div className="flex-1 flex flex-col justify-center min-w-0">
          <span className="text-xs font-mono text-muted-foreground">
            diff {index}.
          </span>
          <span className="text-sm truncate">{title}</span>
        </div>
        <div className="flex flex-row items-end -ml-2">
          {(["code", "conversation"] as DiffView[]).map((v, i) => (
            <Fragment key={v}>
              {i > 0 && (
                <span className="inline-block w-px h-2.5 bg-muted-foreground/40 mx-0.5 mb-1.25" />
              )}
              <button
                type="button"
                onClick={() => onViewChange(v)}
                className={cn(
                  "px-3 py-0.5 text-xs capitalize transition-colors border-b-2 -mb-px",
                  view === v
                    ? "text-foreground border-b-foreground"
                    : "text-muted-foreground hover:text-foreground hover:bg-muted/50 border-b-transparent",
                )}
              >
                {v}
              </button>
            </Fragment>
          ))}
        </div>
      </div>
      <div className="flex flex-col justify-between items-end py-2 px-4">
        <DiffStatusBadge status={status} reviewStatus={review.status} />
        <span className="text-xs font-mono text-muted-foreground">
          {createdAt === updatedAt
            ? `created ${timeAgo(new Date(createdAt))}`
            : `last updated ${timeAgo(new Date(updatedAt))}`}
        </span>
      </div>
    </div>
  );
}
