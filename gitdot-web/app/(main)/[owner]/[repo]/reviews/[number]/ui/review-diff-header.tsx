"use client";

import type { DiffStatus, ReviewDiffResource } from "gitdot-api";
import Link from "@/ui/link";
import { cn } from "@/util";

export function ReviewDiffHeader({
  diffs,
  position,
  owner,
  repo,
  number,
}: {
  diffs: ReviewDiffResource[];
  position: number;
  owner: string;
  repo: string;
  number: number;
}) {
  const activeIndex = diffs.findIndex((d) => d.position === position);

  return (
    <div className="shrink-0 overflow-hidden transition-all duration-200">
      {diffs.map((diff, i) => {
        const isActive = i === activeIndex;
        return (
          <Link
            key={diff.id}
            href={`/${owner}/${repo}/reviews/${number}?diff=${diff.position}`}
            className={cn(
              "w-full flex items-center gap-1.5 px-2 h-8 text-left border-b border-border cursor-pointer transition-colors",
              isActive
                ? "bg-sidebar text-foreground"
                : "text-muted-foreground hover:text-foreground hover:bg-sidebar/60",
            )}
          >
            <span className="font-mono text-sm text-muted-foreground shrink-0 w-5 text-right">
              {i + 1}.
            </span>
            <span className="text-xs flex-1 truncate">
              {diff.message.split("\n")[0]}
            </span>
            <ReviewDiffStatus status={diff.status} />
          </Link>
        );
      })}
    </div>
  );
}

function ReviewDiffStatus({ status }: { status: DiffStatus }) {
  switch (status) {
    case "open":
      return <span className="text-xs shrink-0 text-foreground">open</span>;
    case "approved":
      return <span className="text-xs shrink-0 text-green-600">approved</span>;
    case "changes_requested":
      return <span className="text-xs shrink-0 text-red-500">changes</span>;
    case "merged":
      return <span className="text-xs shrink-0 text-muted-foreground underline">merged</span>;
  }
}
