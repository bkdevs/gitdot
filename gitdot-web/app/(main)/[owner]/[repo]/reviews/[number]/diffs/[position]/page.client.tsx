"use client";

import { Suspense, use } from "react";
import {
  type ResourcePromisesType,
  type ResourceRequestsType,
  useResolvePromises,
} from "@/(main)/[owner]/[repo]/resources";
import { Loading } from "@/ui/loading";
import type { Resources } from "./page";

type ResourceRequests = ResourceRequestsType<Resources>;
type ResourcePromises = ResourcePromisesType<Resources>;

export function PageClient({
  owner,
  repo,
  number,
  position,
  requests,
  promises,
}: {
  owner: string;
  repo: string;
  number: number;
  position: number;
  requests: ResourceRequests;
  promises: ResourcePromises;
}) {
  const resolvedPromises = useResolvePromises(owner, repo, requests, promises);
  return (
    <Suspense fallback={<Loading />}>
      <PageContent
        owner={owner}
        repo={repo}
        number={number}
        position={position}
        promises={resolvedPromises}
      />
    </Suspense>
  );
}

function PageContent({
  position,
  promises,
}: {
  owner: string;
  repo: string;
  number: number;
  position: number;
  promises: ResourcePromises;
}) {
  const review = use(promises.review);
  if (!review) return null;

  const diff = review.diffs.find((d) => d.position === Number(position));
  if (!diff) return null;

  return <>{JSON.stringify(diff)}</>;
}
