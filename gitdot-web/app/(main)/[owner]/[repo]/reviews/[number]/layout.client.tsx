"use client";

import { Undo2 } from "lucide-react";
import { Suspense } from "react";
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

function ReviewSidebarHeader({
  owner,
  repo,
}: {
  owner: string;
  repo: string;
}) {
  return (
    <Link
      href={`/${owner}/${repo}/reviews`}
      className="sticky top-0 bg-background flex items-center justify-between border-b px-2 h-9 z-10 hover:bg-accent/50 cursor-default"
    >
      <h3 className="text-xs font-semibold text-muted-foreground uppercase tracking-wide">
        Reviews
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
  return null;
}
