import { Suspense } from "react";
import { getReview } from "@/dal";
import { Loading } from "@/ui/loading";
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
      <Suspense key={diff.id} fallback={<Loading />}>
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
