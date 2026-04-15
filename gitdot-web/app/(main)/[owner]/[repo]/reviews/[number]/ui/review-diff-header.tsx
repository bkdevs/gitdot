"use client";

import { Fragment, useEffect, useRef, useState } from "react";
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
  const sentinelRef = useRef<HTMLDivElement>(null);
  const [isStuck, setIsStuck] = useState(false);

  useEffect(() => {
    const sentinel = sentinelRef.current;
    if (!sentinel) return;

    // Find the nearest scrollable ancestor to use as the IntersectionObserver root.
    let root: Element | null = sentinel.parentElement;
    while (root && root !== document.documentElement) {
      const { overflowY } = getComputedStyle(root);
      if (overflowY === "auto" || overflowY === "scroll") break;
      root = root.parentElement;
    }
    if (!root || root === document.documentElement) return;

    const observer = new IntersectionObserver(
      ([entry]) => setIsStuck(!entry.isIntersecting),
      { root: root as Element, threshold: 0 },
    );

    observer.observe(sentinel);
    return () => observer.disconnect();
  }, []);

  return (
    <div className="shrink-0">
      {diffs.map((diff, i) => {
        const isActive = i === activeIndex;
        const isBelow = i > activeIndex;

        return (
          <Fragment key={diff.id}>
            {/* Sentinel placed just before the active row. When it exits the
                scroll container's viewport, the active row has reached top:0
                and is stuck — time to collapse the rows below. */}
            {isActive && <div ref={sentinelRef} />}
            <Link
              href={`/${owner}/${repo}/reviews/${number}?diff=${diff.position}`}
              style={
                isBelow
                  ? {
                      height: isStuck ? 0 : undefined,
                      opacity: isStuck ? 0 : undefined,
                      transition: "height 0.2s ease, opacity 0.2s ease",
                    }
                  : undefined
              }
              className={cn(
                "w-full flex items-center gap-1.5 px-2 h-8 text-left border-b border-border cursor-pointer transition-colors overflow-hidden",
                isActive
                  ? "sticky top-0 z-10 bg-sidebar text-foreground"
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
          </Fragment>
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
      return (
        <span className="text-xs shrink-0 text-red-500">changes requested</span>
      );
    case "merged":
      return (
        <span className="text-xs shrink-0 text-muted-foreground underline">
          merged
        </span>
      );
  }
}
