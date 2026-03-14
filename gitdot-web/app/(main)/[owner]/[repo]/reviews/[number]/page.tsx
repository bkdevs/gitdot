import { Suspense } from "react";
import { getReview } from "@/dal";
import { ReviewDetail } from "./review-detail";
import { ReviewDiffContent } from "./review-diff-content";

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; repo: string; number: number }>;
}) {
  const { owner, repo, number } = await params;
  const review = await getReview(owner, repo, number);
  if (!review) return null;

  const diffContents: Record<number, React.ReactNode> = {};
  for (const diff of review.diffs) {
    diffContents[diff.position] = (
      <Suspense
        key={diff.id}
        fallback={
          <div className="flex flex-row w-full h-9 shrink-0 items-center p-2 font-mono text-sm text-muted-foreground">
            loading...
          </div>
        }
      >
        <ReviewDiffContent
          owner={owner}
          repo={repo}
          number={number}
          diff={diff}
        />
      </Suspense>
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
