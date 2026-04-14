"use client";

import {
  type ResourcePromisesType,
  type ResourceRequestsType,
  useResolvePromises,
} from "@/(main)/[owner]/[repo]/resources";
import { OverlayScroll } from "@/ui/scroll";
import type { Resources } from "./layout";
import { ReviewActions } from "./ui/review-actions";
import { ReviewSidebar } from "./ui/review-sidebar";

type ResourceRequests = ResourceRequestsType<Resources>;
type ResourcePromises = ResourcePromisesType<Resources>;

export function LayoutClient({
  owner,
  repo,
  requests,
  promises,
  children,
}: {
  owner: string;
  repo: string;
  requests: ResourceRequests;
  promises: ResourcePromises;
  children: React.ReactNode;
}) {
  const resolvedPromises = useResolvePromises(owner, repo, requests, promises);
  return (
    <>
      <ReviewSidebar owner={owner} repo={repo} promises={resolvedPromises} />
      <OverlayScroll>{children}</OverlayScroll>
      <ReviewActions promises={resolvedPromises} />
    </>
  );
}
