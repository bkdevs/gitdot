import type { DiffResource } from "gitdot-api";
import { Suspense } from "react";
import { renderReviewDiffAction } from "@/actions";
import { Loading } from "@/ui/loading";
import { ReviewBody } from "./review-body";

export function ReviewDiffContent({
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

  const diffEntries = renderReviewDiffAction(
    owner,
    repo,
    number,
    diff.position,
  );

  return (
    <Suspense fallback={<Loading />}>
      <ReviewBody diffEntries={diffEntries} />
    </Suspense>
  );
}
