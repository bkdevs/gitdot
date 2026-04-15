import type { ReviewResource } from "gitdot-api";
import { fetchResources } from "@/provider/server";
import { PageClient } from "./page.client";

export type Resources = {
  review: ReviewResource | null;
};

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; repo: string; number: string }>;
}) {
  const { owner, repo, number } = await params;
  const { requests, promises } = fetchResources(owner, repo, {
    review: (p) => p.getReview(Number(number)),
  });

  return (
    <PageClient
      owner={owner}
      repo={repo}
      number={Number(number)}
      requests={requests}
      promises={promises}
    />
  );
}
