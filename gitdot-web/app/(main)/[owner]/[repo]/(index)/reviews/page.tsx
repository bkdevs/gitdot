import type { ReviewResource } from "gitdot-api";
import { fetchResources } from "@/provider/server";
import { PageClient } from "./page.client";

export type Resources = {
  reviews: ReviewResource[] | null;
};

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; repo: string }>;
}) {
  const { owner, repo } = await params;
  const { requests, promises } = fetchResources(owner, repo, {
    reviews: (p) => p.getReviews(),
  });

  return (
    <PageClient owner={owner} repo={repo} requests={requests} promises={promises} />
  );
}
