"use client";

import type { ReviewResource } from "gitdot-api";
import type { DiffEntry } from "gitdot-dal/client";
import { useMemo, useState } from "react";
import { useShortcuts } from "@/(main)/context/shortcuts";
import { cn } from "@/util";
import { ReviewProvider, useReviewContext } from "./context";
import { ReviewDiff } from "./ui/review-diff";
import { ReviewSummary } from "./ui/review-summary";

export type PageLayout = "split" | "summary" | "diffs";

export function PageClient({
  owner,
  repo,
  position,
  review,
  diffEntriesPromise,
}: {
  owner: string;
  repo: string;
  number: number;
  position: number;
  review: ReviewResource;
  diffEntriesPromise: Promise<DiffEntry[]>;
}) {
  const [layout, setLayout] = useState<PageLayout>("split");

  useShortcuts(
    useMemo(
      () => [
        {
          name: "Toggle diffs",
          description: "diffs",
          keys: ["["],
          execute: () => setLayout((v) => (v === "diffs" ? "split" : "diffs")),
        },
        {
          name: "Toggle summary",
          description: "summary",
          keys: ["]"],
          execute: () =>
            setLayout((v) => (v === "summary" ? "split" : "summary")),
        },
      ],
      [],
    ),
  );

  return (
    <ReviewProvider owner={owner} repo={repo} review={review}>
      <ReviewPage
        layout={layout}
        setLayout={setLayout}
        position={position}
        diffEntriesPromise={diffEntriesPromise}
      />
    </ReviewProvider>
  );
}

function ReviewPage({
  layout,
  position,
  diffEntriesPromise,
}: {
  layout: PageLayout;
  setLayout: (layout: PageLayout) => void;
  position: number;
  diffEntriesPromise: Promise<DiffEntry[]>;
}) {
  const { review } = useReviewContext();

  return (
    <div
      className={cn(
        "relative grid flex-1 min-w-0 h-full overflow-hidden",
        layout === "split" && "grid-cols-[25%_1fr]",
        layout === "summary" && "grid-cols-1",
        layout === "diffs" && "grid-cols-1",
      )}
    >
      <div
        className={cn(
          "flex flex-col min-h-0 border-r",
          layout === "diffs" && "hidden",
        )}
      >
        <div
          className={cn(
            "flex flex-col flex-1 min-h-0",
            layout === "summary" && "max-w-2xl mx-auto w-full",
          )}
        >
          <ReviewSummary review={review} />
        </div>
      </div>
      <div
        className={cn(
          "overflow-y-auto scrollbar-thin min-h-0",
          layout === "summary" && "hidden",
        )}
      >
        <ReviewDiff
          position={position}
          review={review}
          diffEntriesPromise={diffEntriesPromise}
        />
      </div>
    </div>
  );
}
