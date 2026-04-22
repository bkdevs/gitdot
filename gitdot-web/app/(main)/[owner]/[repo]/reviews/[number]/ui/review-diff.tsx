import type { ReviewResource } from "gitdot-api";
import { Suspense } from "react";
import type { DiffEntry } from "@/actions";
import { Loading } from "@/ui/loading";
import { ReviewDiffBody } from "./review-diff-body";
import { ReviewDiffHeader } from "./review-diff-header";

export function ReviewDiff({
  owner,
  repo,
  position,
  review,
  diffEntriesPromise,
}: {
  owner: string;
  repo: string;
  position: number;
  review: ReviewResource;
  diffEntriesPromise: Promise<DiffEntry[]>;
}) {
  const activeDiff = review.diffs.find((d) => d.position === position);
  if (!activeDiff) return null;

  return (
    <div data-diff-top className="flex flex-col w-full min-h-full pb-8">
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
  );
}
