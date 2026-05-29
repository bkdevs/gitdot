"use client";

import type { DiffSpans } from "gitdot-dal/client";
import { preferSplit } from "@/(main)/[owner]/[repo]/util";
import { cn } from "@/util";
import { DiffCreated } from "./diff-created";
import { DiffSplit } from "./diff-split";
import { DiffUnified } from "./diff-unified";
import { DiffUnilateral } from "./diff-unilateral";
import { useDiffSelection } from "./hooks/use-diff-selection";

export function DiffBody({
  spans,
  layout = "heuristic",
  className,
}: {
  spans: DiffSpans;
  layout?: "split" | "unified" | "heuristic";
  className?: string;
}) {
  const { containerRef, handleMouseDown, handleMouseMove, handleMouseUp } =
    useDiffSelection();

  const useSplit =
    spans.kind === "split" &&
    (layout === "split" ||
      (layout === "heuristic" &&
        preferSplit(spans.leftSpans, spans.rightSpans, spans.hunks)));

  return (
    <div
      ref={containerRef}
      onMouseDown={handleMouseDown}
      onMouseMove={handleMouseMove}
      onMouseUp={handleMouseUp}
      className={cn(
        "w-full cursor-default select-none relative",
        "[&.is-dragging_.diff-token]:cursor-default",
        "[&.has-selection_.diff-token:not(.token-selected)]:opacity-40",
        "[&.has-selection_.diff-token:not(.token-selected)]:transition-opacity",
        "[&.has-selection_.diff-token:not(.token-selected)]:duration-200",
        "[&.has-selection_.diff-token.token-selected]:opacity-100",
        "[&.has-selection_.diff-token.token-selected]:transition-opacity",
        "[&.has-selection_.diff-token.token-selected]:duration-200",
        className,
      )}
    >
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
        <div className="text-sm font-mono px-2 text-foreground/50">
          File deleted.
        </div>
      )}
      {(!spans || spans.kind === "no-change") && (
        <div className="text-sm font-mono px-2">No changes made</div>
      )}
    </div>
  );
}
