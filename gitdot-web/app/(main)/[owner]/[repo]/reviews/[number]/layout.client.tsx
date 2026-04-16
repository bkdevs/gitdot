"use client";

import { Suspense } from "react";
import {
  type ResourcePromisesType,
  type ResourceRequestsType,
  useResolvePromises,
} from "@/(main)/[owner]/[repo]/resources";
import { Sidebar } from "@/ui/sidebar";
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
      <Sidebar containerClassName="w-[30%] grow-0" style={{ width: "100%" }}>
        <Suspense>
          <ReviewSummary
            owner={owner}
            repo={repo}
            promises={resolvedPromises}
          />
        </Suspense>
      </Sidebar>
      <div className="flex flex-1 scrollbar-thin overflow-y-auto items-start">
        {children}
      </div>
    </div>
  );
}
