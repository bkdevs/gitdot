import { getReview } from "gitdot-client";
import { renderReviewDiff } from "gitdot-dal/server";
import { PageClient } from "./page.client";

export default async function Page({
  params,
  searchParams,
}: {
  params: Promise<{ owner: string; repo: string; number: string }>;
  searchParams: Promise<{ diff?: string; comment?: string }>;
}) {
  const { owner, repo, number } = await params;
  const { diff } = await searchParams;

  const position = Number(diff ?? 1);

  const review = await getReview(owner, repo, Number(number));
  if (!review) return null;

  const diffEntriesPromise = renderReviewDiff(
    owner,
    repo,
    Number(number),
    position,
  );

  return (
    <PageClient
      owner={owner}
      repo={repo}
      number={Number(number)}
      position={position}
      review={review}
      diffEntriesPromise={diffEntriesPromise}
    />
  );
}
