"use client";

import { Suspense, use } from "react";
import {
  type ResourcePromisesType,
  type ResourceRequestsType,
  useResolvePromises,
} from "@/(main)/[owner]/[repo]/resources";
import type { DiffEntry } from "@/actions";
import { Loading } from "@/ui/loading";
import { ReviewDiffHeader } from "./ui/review-diff-header";
import type { Resources } from "./page";
import { ReviewDiffBody } from "./ui/review-diff-body";

type ResourceRequests = ResourceRequestsType<Resources>;
type ResourcePromises = ResourcePromisesType<Resources>;

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
    <Suspense fallback={<Loading />}>
      <PageContent
        owner={owner}
        repo={repo}
        number={number}
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
  number,
  position,
  promises,
  diffPromise,
}: {
  owner: string;
  repo: string;
  number: number;
  position: number;
  promises: ResourcePromises;
  diffPromise: Promise<DiffEntry[]>;
}) {
  const review = use(promises.review);
  if (!review) return null;

  return (
    <div data-diff-top className="flex flex-col w-full">
      <ReviewDiffHeader
        diffs={review.diffs}
        position={position}
        owner={owner}
        repo={repo}
        number={number}
      />
      <Suspense fallback={<Loading />}>
        <ReviewDiffBody diffPromise={diffPromise} />
      </Suspense>
    </div>
  );
}
