"use client";

import { Suspense, use } from "react";
import {
  type ResourcePromisesType,
  type ResourceRequestsType,
  useResolvePromises,
} from "@/(main)/[owner]/[repo]/resources";
import { Loading } from "@/ui/loading";
import type { Resources } from "./page";
import { ReviewDetail } from "./review-detail";
import { ReviewDiffContent } from "./review-diff-content";

type ResourceRequests = ResourceRequestsType<Resources>;
type ResourcePromises = ResourcePromisesType<Resources>;

export function PageClient({
  owner,
  repo,
  number,
  requests,
  promises,
}: {
  owner: string;
  repo: string;
  number: number;
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
        promises={resolvedPromises}
      />
    </Suspense>
  );
}

function PageContent({
  owner,
  repo,
  number,
  promises,
}: {
  owner: string;
  repo: string;
  number: number;
  promises: ResourcePromises;
}) {
  const review = use(promises.review);
  if (!review) return null;

  const diffContents: Record<number, React.ReactNode> = {};
  for (const diff of review.diffs) {
    diffContents[diff.position] = (
      <ReviewDiffContent
        key={diff.id}
        owner={owner}
        repo={repo}
        number={number}
        diff={diff}
      />
    );
  }

  return (
    <ReviewDetail
      owner={owner}
      repo={repo}
      number={number}
      review={review}
      diffContents={diffContents}
    />
  );
}
