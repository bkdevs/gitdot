"use client";

import { Suspense, use } from "react";
import {
  type ResourcePromisesType,
  type ResourceRequestsType,
  useResolvePromises,
} from "@/(main)/[owner]/[repo]/resources";
import type { DiffEntry } from "@/actions";
import { Loading } from "@/ui/loading";
import { OverlayScroll } from "@/ui/scroll";
import type { Resources } from "./page";
import { CommitBody } from "./ui/commit-body";
import { CommitHeader } from "./ui/commit-header";
import { CommitShortcuts } from "./ui/commit-shortcuts";

type ResourceRequests = ResourceRequestsType<Resources>;
type ResourcePromises = ResourcePromisesType<Resources>;

export function PageClient({
  owner,
  repo,
  requests,
  promises,
  diffEntriesPromise,
}: {
  owner: string;
  repo: string;
  requests: ResourceRequests;
  promises: ResourcePromises;
  diffEntriesPromise: Promise<DiffEntry[]>;
}) {
  const resolvedPromises = useResolvePromises(owner, repo, requests, promises);

  return (
    <OverlayScroll>
      <Suspense fallback={<Loading />}>
        <PageContent
          owner={owner}
          repo={repo}
          promises={resolvedPromises}
          diffEntriesPromise={diffEntriesPromise}
        />
      </Suspense>
    </OverlayScroll>
  );
}

function PageContent({
  owner,
  repo,
  promises,
  diffEntriesPromise,
}: {
  owner: string;
  repo: string;
  promises: ResourcePromises;
  diffEntriesPromise: Promise<DiffEntry[]>;
}) {
  const commit = use(promises.commit);
  if (!commit) return null;

  return (
    <div
      data-diff-top
      className="max-w-5xl mx-auto w-full px-4 py-6 flex flex-col gap-6"
    >
      <CommitHeader commit={commit} owner={owner} repo={repo} />
      <Suspense fallback={<Loading />}>
        <CommitBody diffEntriesPromise={diffEntriesPromise} />
      </Suspense>
      <CommitShortcuts />
    </div>
  );
}
