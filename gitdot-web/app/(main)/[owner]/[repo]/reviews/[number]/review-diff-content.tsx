import type { DiffResource } from "gitdot-api";
import { Suspense } from "react";
import { renderReviewDiffAction } from "@/actions";
import { getReviewDiff } from "@/dal";
import { ReviewBody } from "./review-body";

export async function ReviewDiffContent({
  owner,
  repo,
  number,
  diff,
}: {
  owner: string;
  repo: string;
  number: number;
  diff: DiffResource;
}) {
  const revision = diff.revisions[0];
  if (!revision) {
    return (
      <p className="text-sm text-muted-foreground px-4 py-2">No revisions</p>
    );
  }

  const diffResponse = await getReviewDiff(owner, repo, number, diff.position);
  if (!diffResponse) {
    return (
      <p className="text-sm text-muted-foreground px-4 py-2">
        Failed to load diff
      </p>
    );
  }

  const diffData = renderReviewDiffAction(diffResponse.files);

  return (
    <Suspense
      fallback={
        <div className="flex flex-row w-full h-9 shrink-0 items-center p-2 font-mono text-sm text-muted-foreground">
          loading...
        </div>
      }
    >
      <ReviewBody diffData={diffData} />
    </Suspense>
  );
}
