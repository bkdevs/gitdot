"use client";

import type { DiffStatus, ReviewDiffResource, ReviewStatus } from "gitdot-api";
import { usePathname, useSearchParams } from "next/navigation";
import Link from "@/ui/link";
import { cn } from "@/util";
import { useReviewContext } from "../context";

type DisplayDiffStatus = "open" | "approved" | "merged";

export function ReviewSummaryDiffs({
  diffs,
}: {
  diffs: ReviewDiffResource[];
}) {
  const pathname = usePathname();
  const searchParams = useSearchParams();
  const { review } = useReviewContext();
  const positionParam = searchParams.get("diff");
  const activePosition = positionParam ? Number(positionParam) : diffs[0]?.position;
  const activeIndex = diffs.findIndex((d) => d.position === activePosition);

  return (
    <section className="flex flex-col gap-1.5">
      <h2 className="text-xs font-semibold uppercase tracking-wider text-muted-foreground">
        Diffs
      </h2>
      <div className="flex flex-col gap-0.5">
        {diffs.map((diff, i) => {
          const isActive = i === activeIndex;
          return (
            <Link
              key={diff.id}
              href={`${pathname}?diff=${diff.position}`}
              className={cn(
                "flex items-center gap-1.5 cursor-pointer transition-colors",
                isActive
                  ? "text-foreground underline decoration-current"
                  : "text-muted-foreground hover:text-foreground",
              )}
              prefetch={true}
            >
              <span className="font-mono text-sm shrink-0 w-5 text-left">
                {i + 1}.
              </span>
              <span className="text-sm flex-1 truncate">
                {diff.message.split("\n")[0]}
              </span>
              <DiffStatusBadge
                status={readableDiffStatus(diff.status, review.status)}
              />
            </Link>
          );
        })}
      </div>
    </section>
  );
}

function DiffStatusBadge({ status }: { status: DisplayDiffStatus }) {
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
