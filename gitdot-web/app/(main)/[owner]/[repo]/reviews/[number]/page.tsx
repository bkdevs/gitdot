import type { ReviewResource } from "gitdot-api";
import { renderReviewDiffAction } from "@/actions";
import { fetchResources } from "@/provider/server";
import { PageClient } from "./page.client";

export type Resources = {
  review: ReviewResource | null;
};

export default async function Page({
  params,
  searchParams,
}: {
  params: Promise<{ owner: string; repo: string; number: string }>;
  searchParams: Promise<{ diff?: string }>;
}) {
  const { owner, repo, number } = await params;
  const { diff } = await searchParams;

  const position = Number(diff ?? 1);

  const { requests, promises } = fetchResources(owner, repo, {
    review: (p) => p.getReview(Number(number)),
  });
  const diffEntriesPromise = renderReviewDiffAction(
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
      requests={requests}
      promises={promises}
      diffEntriesPromise={diffEntriesPromise}
    />
  );
}
