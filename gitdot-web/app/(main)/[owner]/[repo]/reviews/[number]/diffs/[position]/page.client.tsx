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
import { ReviewDiffBody } from "./ui/review-diff-body";
import { ReviewDiffHeader } from "./ui/review-diff-header";

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

  const diff = review.diffs.find((d) => d.position === Number(position));
  if (!diff) return null;

  return (
    <div data-diff-top className="flex flex-col w-full">
      <ReviewDiffHeader diff={diff} author={review.author} />
      <Suspense fallback={<Loading />}>
        <ReviewDiffBody diffPromise={diffPromise} />
      </Suspense>
    </div>
  );
}
