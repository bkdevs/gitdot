"use client";

import { Suspense, use } from "react";
import {
  type ResourcePromisesType,
  type ResourceRequestsType,
  useResolvePromises,
} from "@/(main)/[owner]/[repo]/resources";
import { Loading } from "@/ui/loading";
import type { Resources } from "./page";
import { BuildsClient } from "./ui/builds-client";

type ResourceRequests = ResourceRequestsType<Resources>;
type ResourcePromises = ResourcePromisesType<Resources>;

export function PageClient({
  owner,
  repo,
  requests,
  promises,
}: {
  owner: string;
  repo: string;
  requests: ResourceRequests;
  promises: ResourcePromises;
}) {
  const resolvedPromises = useResolvePromises(owner, repo, requests, promises);
  return (
    <Suspense fallback={<Loading />}>
      <PageContent owner={owner} repo={repo} promises={resolvedPromises} />
    </Suspense>
  );
}

function PageContent({
  owner,
  repo,
  promises,
}: {
  owner: string;
  repo: string;
  promises: ResourcePromises;
}) {
  const builds = use(promises.builds);
  const commits = use(promises.commits);
  if (!builds || !commits) return null;
  return (
    <BuildsClient owner={owner} repo={repo} builds={builds} commits={commits} />
  );
}
