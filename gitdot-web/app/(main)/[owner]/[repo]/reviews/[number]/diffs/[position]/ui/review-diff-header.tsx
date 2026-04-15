"use client";

import type { ReviewDiffResource } from "gitdot-api";
import { cn } from "@/util";
import Link from "@/ui/link";

function diffStatusColor(status: string): string {
  switch (status) {
    case "open":
      return "text-blue-500";
    case "approved":
      return "text-green-600";
    case "changes_requested":
      return "text-amber-500";
    case "merged":
      return "text-purple-500";
    case "closed":
      return "text-muted-foreground";
    default:
      return "text-muted-foreground";
  }
}

function diffStatusLabel(status: string): string {
  switch (status) {
    case "open":
      return "Open";
    case "approved":
      return "Approved";
    case "changes_requested":
      return "Changes";
    case "merged":
      return "Merged";
    case "closed":
      return "Closed";
    default:
      return status;
  }
}

function StatusBadge({ status }: { status: string }) {
  return (
    <span className={cn("text-xs shrink-0", diffStatusColor(status))}>
      {diffStatusLabel(status)}
    </span>
  );
}

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
    <div className="shrink-0 border-b border-border overflow-hidden transition-all duration-200">
      {diffs.map((diff, i) => {
        const isActive = i === activeIndex;
        return (
          <Link
            key={diff.id}
            href={`/${owner}/${repo}/reviews/${number}/diffs/${diff.position}`}
            className={cn(
              "w-full flex items-center gap-1.5 px-2 h-8 text-left border-b border-border last:border-b-0 cursor-pointer transition-colors",
              isActive
                ? "bg-sidebar text-foreground"
                : "text-muted-foreground hover:text-foreground hover:bg-sidebar/60",
            )}
          >
            <span className="font-mono text-sm text-muted-foreground shrink-0 w-5 text-right">
              {i + 1}.
            </span>
            <span className="text-xs flex-1 truncate">{diff.message}</span>
            <StatusBadge status={diff.status} />
          </Link>
        );
      })}
    </div>
  );
}
