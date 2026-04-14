import type { ReviewResource } from "gitdot-api";
import { fetchResources } from "@/provider/server";
import { PageClient } from "./page.client";

export type Resources = {
  review: ReviewResource | null;
};

export default async function Page({
  params,
}: {
  params: Promise<{
    owner: string;
    repo: string;
    number: number;
    position: number;
  }>;
}) {
  const { owner, repo, number, position } = await params;
  const { requests, promises } = fetchResources(owner, repo, {
    review: (p) => p.getReview(number),
  });

  return (
    <PageClient
      owner={owner}
      repo={repo}
      number={number}
      position={position}
      requests={requests}
      promises={promises}
    />
  );
}
