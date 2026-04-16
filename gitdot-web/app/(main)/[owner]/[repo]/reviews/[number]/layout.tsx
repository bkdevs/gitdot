import type { ReviewResource } from "gitdot-api";
import { fetchResources } from "@/provider/server";
import { LayoutClient } from "./layout.client";

export type Resources = {
  review: ReviewResource | null;
};

export default async function Layout({
  params,
  children,
}: {
  params: Promise<{ owner: string; repo: string; number: string }>;
  children: React.ReactNode;
}) {
  const { owner, repo, number } = await params;

  const { requests, promises } = fetchResources(owner, repo, {
    review: (p) => p.getReview(Number(number)),
  });

  return (
    <LayoutClient
      owner={owner}
      repo={repo}
      requests={requests}
      promises={promises}
    >
      {children}
    </LayoutClient>
  );
}
