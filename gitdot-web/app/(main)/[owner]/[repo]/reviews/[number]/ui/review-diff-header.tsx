"use client";

import type { DiffStatus, ReviewDiffResource, ReviewStatus } from "gitdot-api";
import { usePathname } from "next/navigation";
import Link from "@/ui/link";
import { cn } from "@/util";
import { useReviewContext } from "../context";

type DisplayDiffStatus = "open" | "approved" | "merged";

export function ReviewDiffHeader({
  diffs,
  position,
}: {
  diffs: ReviewDiffResource[];
  position: number;
}) {
  const pathname = usePathname();
  const { review } = useReviewContext();
  const activeIndex = diffs.findIndex((d) => d.position === position);

  return (
    <>
      {diffs.map((diff, i) => {
        const isActive = i === activeIndex;
        return (
          <Link
            key={diff.id}
            href={`${pathname}?diff=${diff.position}`}
            className={cn(
              "w-full flex items-center gap-1.5 px-2 h-8 text-left border-b border-border cursor-default",
              isActive
                ? "sticky top-0 z-10 bg-sidebar text-foreground"
                : "text-muted-foreground hover:bg-sidebar",
            )}
            prefetch={true}
          >
            <span
              className={cn(
                "font-mono text-sm shrink-0 w-5 text-right",
                isActive ? "text-foreground" : "text-muted-foreground",
              )}
            >
              {i + 1}.
            </span>
            <span className="text-xs flex-1 truncate">
              {diff.message.split("\n")[0]}
            </span>
            <ReviewDiffStatus
              status={readableDiffStatus(diff.status, review.status)}
            />
          </Link>
        );
      })}
    </>
  );
}

function ReviewDiffStatus({ status }: { status: DisplayDiffStatus }) {
  switch (status) {
    case "approved":
      return <span className="text-xs shrink-0 text-green-600">approved</span>;
    case "open":
      return <span className="text-xs shrink-0 text-foreground">open</span>;
    case "merged":
      return (
        <span className="text-xs shrink-0 text-muted-foreground underline">
          merged
        </span>
      );
  }
}

function readableDiffStatus(
  status: DiffStatus,
  reviewStatus: ReviewStatus,
): DisplayDiffStatus {
  if (reviewStatus === "draft" && status === "open") return "approved";
  if (status === "merged") return "merged";
  return "open";
}
