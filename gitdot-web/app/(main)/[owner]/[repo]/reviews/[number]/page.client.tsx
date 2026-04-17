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
import type { Resources } from "./page";
import { ReviewDiff } from "./ui/review-diff";
import { ReviewLayoutToggles } from "./ui/review-layout-toggles";
import { ReviewSummary } from "./ui/review-summary";

type ResourceRequests = ResourceRequestsType<Resources>;
type ResourcePromises = ResourcePromisesType<Resources>;

export type PageLayout = "default" | "summary" | "diff";

export function PageClient({
  owner,
  repo,
  number,
  position,
  requests,
  promises,
  diffPromise,
}: {
  owner: string;
  repo: string;
  number: number;
  position: number;
  requests: ResourceRequests;
  promises: ResourcePromises;
  diffPromise: Promise<DiffEntry[]>;
}) {
  const resolvedPromises = useResolvePromises(owner, repo, requests, promises);

  return (
    <Suspense fallback={<Loading className="px-6!" />}>
      <PageContent
        owner={owner}
        repo={repo}
        position={position}
        promises={resolvedPromises}
        diffPromise={diffPromise}
      />
    </Suspense>
  );
}

function PageContent({
  owner,
  repo,
  position,
  promises,
  diffPromise,
}: {
  owner: string;
  repo: string;
  position: number;
  promises: ResourcePromises;
  diffPromise: Promise<DiffEntry[]>;
}) {
  const [layout, setLayout] = useState<PageLayout>("default");

  useShortcuts(
    useMemo(
      () => [
        {
          name: "Toggle diffs",
          description: "diffs",
          keys: ["["],
          execute: () => setLayout((v) => (v === "diff" ? "default" : "diff")),
        },
        {
          name: "Toggle summary",
          description: "summary",
          keys: ["]"],
          execute: () =>
            setLayout((v) => (v === "summary" ? "default" : "summary")),
        },
      ],
      [],
    ),
  );

  const review = use(promises.review);
  if (!review) return null;

  return (
    <div className="flex flex-1 min-w-0 h-full">
      <div
        className={cn(
          "flex-col h-full! border-r shrink-0",
          layout === "summary"
            ? "flex-1"
            : layout === "diff"
              ? "hidden"
              : "w-[30%] grow-0",
        )}
      >
        <ReviewSummary owner={owner} repo={repo} review={review} />
      </div>
      <div
        className={cn(
          "flex flex-1 scrollbar-thin overflow-y-auto items-start",
          layout === "summary" && "hidden",
        )}
      >
        <ReviewDiff
          owner={owner}
          repo={repo}
          position={position}
          review={review}
          diffPromise={diffPromise}
        />
      </div>
      <div className="fixed bottom-6 left-0">
        <ReviewLayoutToggles layout={layout} setLayout={setLayout} />
      </div>
    </div>
  );
}
