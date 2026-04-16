"use client";

import type { ReviewResource } from "gitdot-api";
import { Suspense, use } from "react";
import type { DiffEntry } from "@/actions";
import { Loading } from "@/ui/loading";
import { ReviewSplashPage } from "@/ui/review-splash-page";
import { Sidebar } from "@/ui/sidebar";
import { ReviewDiffBody } from "../../[number]/ui/review-diff-body";
import { ReviewDiffHeader } from "../../[number]/ui/review-diff-header";
import { ReviewSummary } from "../../[number]/ui/review-summary";

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
    <Suspense>
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

  if (!review.title && !review.description) {
    return <ReviewSplashPage owner={owner} repo={repo} review={review} />;
  }

  const activeDiff = review.diffs.find((d) => d.position === position);
  if (!activeDiff) return null;

  return (
    <div className="flex flex-1 min-w-0 h-full">
      <Sidebar containerClassName="w-[30%] grow-0" style={{ width: "100%" }}>
        <ReviewSummary
          owner={owner}
          repo={repo}
          promises={{ review: reviewPromise }}
        />
      </Sidebar>
      <div className="flex flex-1 scrollbar-thin overflow-y-auto items-start">
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
      </div>
    </div>
  );
}
