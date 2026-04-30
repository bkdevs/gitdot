"use client";

import type { ReviewResource } from "gitdot-api";
import { Suspense } from "react";
import type { DiffEntry } from "@/actions";
import { Loading } from "@/ui/loading";
import { ReviewDiffBody } from "./review-diff-body";
import { ReviewDiffHeader } from "./review-diff-header";

export function ReviewDiff({
  position,
  review,
  diffEntriesPromise,
}: {
  position: number;
  review: ReviewResource;
  diffEntriesPromise: Promise<DiffEntry[]>;
}) {
  const activeDiff = review.diffs.find((d) => d.position === position);
  if (!activeDiff) return null;

  const index = review.diffs.findIndex((d) => d.position === position) + 1;
  return (
    <div data-diff-top className="flex flex-col w-full min-h-full pb-8">
      <ReviewDiffHeader
        title={activeDiff.message.split("\n")[0]}
        index={index}
        author={review.author ?? null}
        status={activeDiff.status}
        createdAt={activeDiff.created_at}
        updatedAt={activeDiff.updated_at}
      />
      <Suspense fallback={<Loading />}>
        <ReviewDiffBody
          diffEntriesPromise={diffEntriesPromise}
          diff={activeDiff}
        />
      </Suspense>
    </div>
  );
}
