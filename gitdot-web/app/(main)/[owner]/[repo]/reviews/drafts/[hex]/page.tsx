import { renderReviewDiffAction } from "@/actions";
import { getReview } from "@/dal/review";
import { PageClient } from "./page.client";

export default async function Page({
  params,
  searchParams,
}: {
  params: Promise<{ owner: string; repo: string; hex: string }>;
  searchParams: Promise<{ diff?: string }>;
}) {
  const { owner, repo, hex } = await params;
  const { diff } = await searchParams;

  const position = Number(diff ?? 1);

  const reviewPromise = getReview(owner, repo, hex);
  const diffEntriesPromise = renderReviewDiffAction(owner, repo, hex, position);

  return (
    <PageClient
      owner={owner}
      repo={repo}
      position={position}
      reviewPromise={reviewPromise}
      diffEntriesPromise={diffEntriesPromise}
    />
  );
}
