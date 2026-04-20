"use client";

import { preferSplit } from "@/(main)/[owner]/[repo]/util";
import type { DiffSpans } from "@/actions";
import { cn } from "@/util";
import { DiffCreated } from "./diff-created";
import { DiffSplit } from "./diff-split";
import { DiffUnified } from "./diff-unified";
import { DiffUnilateral } from "./diff-unilateral";

export function DiffBody({
  spans,
  layout = "heuristic",
  className,
}: {
  spans: DiffSpans;
  layout?: "split" | "unified" | "heuristic";
  className?: string;
}) {
  const useSplit =
    spans.kind === "split" &&
    (layout === "split" ||
      (layout === "heuristic" &&
        preferSplit(spans.leftSpans, spans.rightSpans, spans.hunks)));

  return (
    <div className={cn("w-full", className)}>
      {spans.kind === "split" &&
        (useSplit ? (
          <DiffSplit
            leftSpans={spans.leftSpans}
            rightSpans={spans.rightSpans}
            hunks={spans.hunks}
          />
        ) : (
          <DiffUnified
            leftSpans={spans.leftSpans}
            rightSpans={spans.rightSpans}
            hunks={spans.hunks}
          />
        ))}
      {spans.kind === "unilateral" && (
        <DiffUnilateral
          spans={spans.spans}
          hunks={spans.hunks}
          side={spans.side}
        />
      )}
      {spans.kind === "created" && <DiffCreated spans={spans.spans} />}
      {spans.kind === "deleted" && (
        <div className="text-sm font-mono px-2 text-primary/50">
          File deleted.
        </div>
      )}
      {(!spans || spans.kind === "no-change") && (
        <div className="text-sm font-mono px-2">No changes made</div>
      )}
    </div>
  );
}
