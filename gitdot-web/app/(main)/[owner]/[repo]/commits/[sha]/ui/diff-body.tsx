"use client";

import { preferSplit } from "@/(main)/[owner]/[repo]/util";
import type { DiffData } from "@/actions";
import { cn } from "@/util";
import { DiffCreated } from "./diff-created";
import { DiffSplit } from "./diff-split";
import { DiffUnified } from "./diff-unified";
import { DiffUnilateral } from "./diff-unilateral";

export function DiffBody({
  data,
  layout = "heuristic",
  className,
}: {
  data: DiffData;
  layout?: "split" | "unified" | "heuristic";
  className?: string;
}) {
  const useSplit =
    data.kind === "split" &&
    (layout === "split" ||
      (layout === "heuristic" &&
        preferSplit(data.leftSpans, data.rightSpans, data.hunks)));

  return (
    <div className={cn("w-full cursor-default", className)}>
      {data.kind === "split" &&
        (useSplit ? (
          <DiffSplit
            leftSpans={data.leftSpans}
            rightSpans={data.rightSpans}
            hunks={data.hunks}
          />
        ) : (
          <DiffUnified
            leftSpans={data.leftSpans}
            rightSpans={data.rightSpans}
            hunks={data.hunks}
          />
        ))}
      {data.kind === "unilateral" && (
        <DiffUnilateral
          spans={data.spans}
          hunks={data.hunks}
          side={data.side}
        />
      )}
      {data.kind === "created" && <DiffCreated spans={data.spans} />}
      {data.kind === "deleted" && (
        <div className="text-sm font-mono px-2 text-primary/50">
          File deleted.
        </div>
      )}
      {(!data || data.kind === "no-change") && (
        <div className="text-sm font-mono px-2">No changes made</div>
      )}
    </div>
  );
}
