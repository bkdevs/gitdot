"use client";

import type { ReviewResource } from "gitdot-api";
import { Suspense, use } from "react";
import type { DiffEntry } from "@/actions";
import { Loading } from "@/ui/loading";
import { ReviewSplashPage } from "@/ui/review-splash-page";
import { Sidebar } from "@/ui/sidebar";
import { ReviewProvider } from "../../[number]/context";
import { ReviewDiffBody } from "../../[number]/ui/review-diff-body";
import { ReviewDiffHeader } from "../../[number]/ui/review-diff-header";
import { ReviewSummary } from "../../[number]/ui/review-summary";

export function PageClient({
  owner,
  repo,
  position,
  reviewPromise,
  diffEntriesPromise,
}: {
  owner: string;
  repo: string;
  position: number;
  reviewPromise: Promise<ReviewResource | null>;
  diffEntriesPromise: Promise<DiffEntry[]>;
}) {
  return (
    <Suspense>
      <PageContent
        owner={owner}
        repo={repo}
        position={position}
        reviewPromise={reviewPromise}
        diffEntriesPromise={diffEntriesPromise}
      />
    </Suspense>
  );
}

function PageContent({
  owner,
  repo,
  position,
  reviewPromise,
  diffEntriesPromise,
}: {
  owner: string;
  repo: string;
  position: number;
  reviewPromise: Promise<ReviewResource | null>;
  diffEntriesPromise: Promise<DiffEntry[]>;
}) {
  const review = use(reviewPromise);
  if (!review) return null;
  console.log(review);

  if (!review.title && !review.description) {
    return <ReviewSplashPage owner={owner} repo={repo} review={review} />;
  }

  const activeDiff = review.diffs.find((d) => d.position === position);
  if (!activeDiff) return null;

  return (
    <ReviewProvider owner={owner} repo={repo} review={review}>
      <div className="flex flex-1 min-w-0 h-full">
        <Sidebar containerClassName="w-[30%] grow-0" style={{ width: "100%" }}>
          <ReviewSummary review={review} />
        </Sidebar>
        <div className="flex flex-1 scrollbar-thin overflow-y-auto items-start">
          <div data-diff-top className="flex flex-col w-full min-h-full">
            <ReviewDiffHeader diffs={review.diffs} position={position} />
            <Suspense fallback={<Loading />}>
              <ReviewDiffBody
                owner={owner}
                repo={repo}
                review={review}
                diffEntriesPromise={diffEntriesPromise}
                diff={activeDiff}
              />
            </Suspense>
          </div>
        </div>
      </div>
    </ReviewProvider>
  );
}
