"use client";

import { Undo2 } from "lucide-react";
import { UserImage } from "@/(main)/[owner]/ui/user-image";
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
import { timeAgo } from "@/util";
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
            <CommitSidebarHeader owner={owner} repo={repo} />
            <Suspense>
              <CommitSidebarContent
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

function CommitSidebarHeader({ owner, repo }: { owner: string; repo: string }) {
  return (
    <Link
      href={`/${owner}/${repo}/commits`}
      className="sticky top-0 bg-background flex items-center justify-between border-b px-2 h-9 z-10 hover:bg-accent/50 cursor-default"
    >
      <h3 className="text-xs font-semibold text-muted-foreground uppercase tracking-wide">
        Commits
      </h3>
      <Undo2 size={14} className="text-muted-foreground -translate-y-px" />
    </Link>
  );
}

function CommitSidebarContent({
  owner,
  repo,
  promises,
}: {
  owner: string;
  repo: string;
  promises: ResourcePromises;
}) {
  const { sha } = useParams<{
    owner: string;
    repo: string;
    sha: string | undefined;
  }>();

  const commits = use(promises.commits);
  if (!commits) return null;

  return commits.map((commit) => {
    const isActive = sha === commit.sha.substring(0, 7);
    const author = commit.author.name;
    return (
      <Link
        key={commit.sha}
        href={`/${owner}/${repo}/commits/${commit.sha.substring(0, 7)}`}
        className={`flex w-full border-b hover:bg-accent/50 select-none cursor-default py-2 px-2 ${
          isActive && "bg-sidebar"
        }`}
        prefetch={true}
        data-sidebar-item
        data-sidebar-item-active={isActive ? "true" : undefined}
      >
        <div className="flex flex-row w-full gap-2 min-w-0">
          <div className="shrink-0 pt-0.5">
            <UserImage user={commit.author} px={20} />
          </div>
          <div className="flex flex-col flex-1 justify-start items-start min-w-0">
            <div className="text-sm truncate mb-0.5 w-full">{commit.message}</div>
            <div className="text-xs text-muted-foreground flex items-center gap-1 w-full min-w-0">
              <span className="truncate min-w-0 underline cursor-pointer">{author}</span>
              <span className="shrink-0">{timeAgo(new Date(commit.date))}</span>
            </div>
          </div>
        </div>
      </Link>
    );
  });
}
