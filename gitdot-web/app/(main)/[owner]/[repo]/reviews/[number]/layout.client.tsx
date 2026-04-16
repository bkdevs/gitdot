"use client";

import { Suspense } from "react";
import {
  type ResourcePromisesType,
  type ResourceRequestsType,
  useResolvePromises,
} from "@/(main)/[owner]/[repo]/resources";
import type { Resources } from "./layout";
import { ReviewSummary } from "./ui/review-summary";

type ResourceRequests = ResourceRequestsType<Resources>;
type ResourcePromises = ResourcePromisesType<Resources>;

export function LayoutClient({
  owner,
  repo,
  requests,
  promises,
  children,
}: {
  owner: string;
  repo: string;
  requests: ResourceRequests;
  promises: ResourcePromises;
  children: React.ReactNode;
}) {
  const resolvedPromises = useResolvePromises(owner, repo, requests, promises);
  return (
    <div className="flex flex-1 min-w-0 h-full">
      <div className="w-[30%] grow-0 shrink-0 border-r h-full">
        <Suspense>
          <ReviewSummary
            owner={owner}
            repo={repo}
            promises={resolvedPromises}
          />
        </Suspense>
      </div>
      <div className="flex flex-1 overflow-y-auto items-start">{children}</div>
    </div>
  );
}
