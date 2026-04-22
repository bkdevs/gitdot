"use client";

import { Suspense, use, useMemo, useState } from "react";
import {
  type ResourcePromisesType,
  type ResourceRequestsType,
  useResolvePromises,
} from "@/(main)/[owner]/[repo]/resources";
import { useShortcuts } from "@/(main)/context/shortcuts";
import type { DiffEntry } from "@/actions";
import { Loading } from "@/ui/loading";
import { cn } from "@/util";
import { ReviewProvider, useReviewContext } from "./context";
import type { Resources } from "./page";
import { ReviewActions } from "./ui/review-actions";
import { ReviewDiff } from "./ui/review-diff";
import { ReviewLayoutToggles } from "./ui/review-layout-toggles";
import { ReviewSplash } from "./ui/review-splash";
import { ReviewSummary } from "./ui/review-summary";

type ResourceRequests = ResourceRequestsType<Resources>;
type ResourcePromises = ResourcePromisesType<Resources>;

export type PageLayout = "split" | "summary" | "diffs";

export function PageClient({
  owner,
  repo,
  number,
  position,
  requests,
  promises,
  diffEntriesPromise,
}: {
  owner: string;
  repo: string;
  number: number;
  position: number;
  requests: ResourceRequests;
  promises: ResourcePromises;
  diffEntriesPromise: Promise<DiffEntry[]>;
}) {
  const resolvedPromises = useResolvePromises(owner, repo, requests, promises);

  return (
    <Suspense fallback={<Loading className="px-6!" />}>
      <PageContent
        owner={owner}
        repo={repo}
        position={position}
        promises={resolvedPromises}
        diffEntriesPromise={diffEntriesPromise}
      />
    </Suspense>
  );
}

function PageContent({
  owner,
  repo,
  position,
  promises,
  diffEntriesPromise,
}: {
  owner: string;
  repo: string;
  position: number;
  promises: ResourcePromises;
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

  const initialReview = use(promises.review);
  if (!initialReview) return null;

  return (
    <ReviewProvider owner={owner} repo={repo} review={initialReview}>
      <ReviewPage
        layout={layout}
        setLayout={setLayout}
        owner={owner}
        repo={repo}
        position={position}
        diffEntriesPromise={diffEntriesPromise}
      />
    </ReviewProvider>
  );
}

function ReviewPage({
  layout,
  setLayout,
  owner,
  repo,
  position,
  diffEntriesPromise,
}: {
  layout: PageLayout;
  setLayout: (layout: PageLayout) => void;
  owner: string;
  repo: string;
  position: number;
  diffEntriesPromise: Promise<DiffEntry[]>;
}) {
  const { review } = useReviewContext();

  if (!review.title && !review.description) {
    return <ReviewSplash />;
  }

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
          <ReviewActions />
        </div>
      </div>
      <div
        className={cn(
          "overflow-y-auto scrollbar-thin min-h-0",
          layout === "summary" && "hidden",
        )}
      >
        <ReviewDiff
          owner={owner}
          repo={repo}
          position={position}
          review={review}
          diffEntriesPromise={diffEntriesPromise}
        />
      </div>
      <div className="absolute bottom-0 right-0 z-10">
        <ReviewLayoutToggles layout={layout} setLayout={setLayout} />
      </div>
    </div>
  );
}
