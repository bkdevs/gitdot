import type { ReviewResource } from "gitdot-api";
import { getReview } from "@/dal/review";
import { LayoutClient } from "./layout.client";

export type Resources = {
  review: ReviewResource | null;
};

export default async function Layout({
  params,
  children,
}: {
  params: Promise<{ owner: string; repo: string; hex: string }>;
  children: React.ReactNode;
}) {
  const { owner, repo, hex } = await params;

  const reviewPromise = getReview(owner, repo, hex);

  return (
    <LayoutClient owner={owner} repo={repo} reviewPromise={reviewPromise}>
      {children}
    </LayoutClient>
  );
}
