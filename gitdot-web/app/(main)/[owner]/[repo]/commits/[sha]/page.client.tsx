"use client";

import { Suspense, use } from "react";
import {
  type ResourcePromisesType,
  type ResourceRequestsType,
  useResolvePromises,
} from "@/(main)/[owner]/[repo]/resources";
import type { DiffEntry } from "@/actions";
import { Loading } from "@/ui/loading";
import type { Resources } from "./page";
import { CommitBody } from "./ui/commit-body";
import { CommitHeader } from "./ui/commit-header";
import { CommitShortcuts } from "./ui/commit-shortcuts";

type ResourceRequests = ResourceRequestsType<Resources>;
type ResourcePromises = ResourcePromisesType<Resources>;

// same TODO as file page, resolveResources is invoked repeatedly as requests and promises
// are updated by SSR (Promises is ->) so this is called multiple times on render...
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
    <Suspense fallback={<Loading />}>
      <PageContent promises={resolvedPromises} diffEntriesPromise={diffEntriesPromise} />
    </Suspense>
  );
}

function PageContent({
  promises,
  diffEntriesPromise,
}: {
  promises: ResourcePromises;
  diffEntriesPromise: Promise<DiffEntry[]>;
}) {
  const commit = use(promises.commit);
  if (!commit) return null;

  return (
    <div data-diff-top className="flex flex-col w-full">
      <CommitHeader commit={commit} stats={commit.diffs} />
      <Suspense fallback={<Loading />}>
        <CommitBody diffEntriesPromise={diffEntriesPromise} />
      </Suspense>
      <CommitShortcuts />
    </div>
  );
}
