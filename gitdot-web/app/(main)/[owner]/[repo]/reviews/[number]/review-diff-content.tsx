import type { DiffResource } from "gitdot-api";
import { Suspense } from "react";
import { renderReviewDiffAction } from "@/actions";
import { getReviewDiff } from "@/dal";
import { Loading } from "@/ui/loading";
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
    <Suspense fallback={<Loading />}>
      <ReviewBody diffData={diffData} />
    </Suspense>
  );
}
