"use client";

import type { ReviewResource } from "gitdot-api";
import { Suspense, use } from "react";
import type { DiffEntry } from "@/actions";
import { Loading } from "@/ui/loading";
import { ReviewDiffBody } from "../../[number]/ui/review-diff-body";
import { ReviewDiffHeader } from "../../[number]/ui/review-diff-header";

export function PageClient({
  owner,
  repo,
  position,
  reviewPromise,
  diffPromise,
}: {
  owner: string;
  repo: string;
  position: number;
  reviewPromise: Promise<ReviewResource | null>;
  diffPromise: Promise<DiffEntry[]>;
}) {
  return (
    <Suspense fallback={<Loading />}>
      <PageContent
        owner={owner}
        repo={repo}
        position={position}
        reviewPromise={reviewPromise}
        diffPromise={diffPromise}
      />
    </Suspense>
  );
}

function PageContent({
  owner,
  repo,
  position,
  reviewPromise,
  diffPromise,
}: {
  owner: string;
  repo: string;
  position: number;
  reviewPromise: Promise<ReviewResource | null>;
  diffPromise: Promise<DiffEntry[]>;
}) {
  const review = use(reviewPromise);
  if (!review) return null;

  const activeDiff = review.diffs.find((d) => d.position === position);
  if (!activeDiff) return null;

  return (
    <div data-diff-top className="flex flex-col w-full min-h-full">
      <ReviewDiffHeader diffs={review.diffs} position={position} />
      <Suspense fallback={<Loading />}>
        <ReviewDiffBody
          owner={owner}
          repo={repo}
          review={review}
          diffPromise={diffPromise}
          diff={activeDiff}
        />
      </Suspense>
    </div>
  );
}
