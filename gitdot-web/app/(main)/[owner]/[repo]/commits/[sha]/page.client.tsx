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
import { CommitSidebar } from "./ui/commit-sidebar";

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
      className="grid grid-cols-[1fr_minmax(0,64rem)_1fr] w-full"
    >
      <aside className="hidden xl:flex xl:flex-col sticky top-0 self-start max-h-screen overflow-y-auto py-4 pl-2 w-72 gap-4 justify-self-end">
        <CommitHeader commit={commit} owner={owner} repo={repo} />
        <Suspense fallback={null}>
          <CommitSidebar diffEntriesPromise={diffEntriesPromise} />
        </Suspense>
      </aside>
      <div className="w-full px-4 py-4 flex flex-col gap-6 min-w-0">
        <Suspense fallback={<Loading />}>
          <CommitBody diffEntriesPromise={diffEntriesPromise} />
        </Suspense>
      </div>
      <div />
      <CommitShortcuts />
    </div>
  );
}
