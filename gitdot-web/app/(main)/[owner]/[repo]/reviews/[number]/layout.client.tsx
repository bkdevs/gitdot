"use client";

import type { ReviewDiffResource } from "gitdot-api";
import { Undo2 } from "lucide-react";
import { useParams } from "next/navigation";
import { Suspense, use } from "react";
import {
  type ResourcePromisesType,
  type ResourceRequestsType,
  useResolvePromises,
} from "@/(main)/[owner]/[repo]/resources";
import Link from "@/ui/link";
import { OverlayScroll } from "@/ui/scroll";
import { Sidebar, SidebarContent } from "@/ui/sidebar";
import type { Resources } from "./layout";

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
      <Sidebar>
        <SidebarContent className="overflow-auto">
          <div className="flex flex-col w-full">
            <ReviewSidebarHeader owner={owner} repo={repo} />
            <Suspense>
              <ReviewSidebarContent
                owner={owner}
                repo={repo}
                promises={resolvedPromises}
              />
            </Suspense>
          </div>
        </SidebarContent>
      </Sidebar>
      <OverlayScroll>{children}</OverlayScroll>
    </>
  );
}

function ReviewSidebarHeader({ owner, repo }: { owner: string; repo: string }) {
  return (
    <Link
      href={`/${owner}/${repo}/reviews`}
      className="sticky top-0 bg-background flex items-center justify-between border-b px-2 h-9 z-10 hover:bg-accent/50 cursor-default"
    >
      <h3 className="text-xs font-semibold text-muted-foreground uppercase tracking-wide">
        REVIEW
      </h3>
      <Undo2 size={14} className="text-muted-foreground -translate-y-px" />
    </Link>
  );
}

function ReviewSidebarContent({
  owner,
  repo,
  promises,
}: {
  owner: string;
  repo: string;
  promises: ResourcePromises;
}) {
  const { position } = useParams<{ position: string | undefined }>();
  const review = use(promises.review);
  if (!review) return null;

  return review.diffs.map((diff) => {
    const isActive = position === String(diff.position);
    return (
      <ReviewDiffRow
        key={diff.id}
        owner={owner}
        repo={repo}
        reviewNumber={review.number}
        diff={diff}
        isActive={isActive}
      />
    );
  });
}

function ReviewDiffRow({
  owner,
  repo,
  reviewNumber,
  diff,
  isActive,
}: {
  owner: string;
  repo: string;
  reviewNumber: number;
  diff: ReviewDiffResource;
  isActive: boolean;
}) {
  const title = diff.message.split("\n")[0];
  return (
    <Link
      href={`/${owner}/${repo}/reviews/${reviewNumber}/diffs/${diff.position}`}
      prefetch={true}
      className={`flex flex-col py-1 w-full h-16 border-b hover:bg-accent/50 select-none cursor-default px-2 ${isActive ? "bg-sidebar" : ""}`}
      data-sidebar-item
      data-sidebar-item-active={isActive ? "true" : undefined}
    >
      <span className="text-xs line-clamp-2">
        <span className="font-mono text-muted-foreground">
          {diff.position}.
        </span>{" "}
        {title}
      </span>
      <div className="flex items-center justify-between w-full font-mono mt-auto">
        <span className="text-xs text-muted-foreground">
          {Math.random().toString(16).slice(2, 9)}
        </span>
        <span className={`text-xs ${diffStatusColor(diff.status)}`}>
          {diff.status}
        </span>
      </div>
    </Link>
  );
}

function diffStatusColor(status: string): string {
  switch (status) {
    case "open":
      return "text-blue-500";
    case "approved":
      return "text-green-600";
    case "changes_requested":
      return "text-amber-500";
    case "merged":
      return "text-green-600";
    default:
      return "text-muted-foreground";
  }
}
