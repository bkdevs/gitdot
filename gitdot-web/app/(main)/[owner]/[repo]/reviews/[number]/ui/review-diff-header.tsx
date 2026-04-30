"use client";

import type { DiffStatus, ReviewStatus } from "gitdot-api";
import { useState } from "react";
import { cn } from "@/util";
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
    return <span className="text-xs font-mono text-green-600">approved</span>;
  }
  if (status === "merged") {
    return <span className="text-xs font-mono text-muted-foreground underline">merged</span>;
  }
  return <span className="text-xs font-mono text-foreground">open</span>;
}

export function ReviewDiffHeader({
  title,
  index,
  author,
  status,
}: {
  title: string;
  index: number;
  author: { id: string; name: string } | null;
  status: DiffStatus;
}) {
  const [view, setView] = useState<DiffView>("code");
  const { review } = useReviewContext();

  return (
    <div className="sticky top-0 z-10 shrink-0 h-16 flex items-stretch border-b border-border bg-background pl-4 pt-1 min-w-0">
      <div className="flex flex-col justify-center min-w-0 flex-1">
        <span className="text-sm leading-tight line-clamp-2">{title}</span>
        <div className="flex items-center gap-1.5 text-xs font-mono text-muted-foreground">
          <span>diff #{index}/N{author ? ` by ${author.name}` : ""}</span>
          <span>·</span>
          <DiffStatusBadge status={status} reviewStatus={review.status} />
        </div>
      </div>
      <div className="flex flex-col items-end justify-end">
        <div className="flex flex-row items-end">
          {(["code", "conversation"] as DiffView[]).map((v) => (
            <button
              key={v}
              type="button"
              onClick={() => setView(v)}
              className={cn(
                "px-3 py-1 text-xs capitalize transition-colors border-b-2 -mb-px",
                view === v
                  ? "text-foreground border-b-foreground"
                  : "text-muted-foreground hover:text-foreground border-b-transparent",
              )}
            >
              {v}
            </button>
          ))}
        </div>
      </div>
    </div>
  );
}
