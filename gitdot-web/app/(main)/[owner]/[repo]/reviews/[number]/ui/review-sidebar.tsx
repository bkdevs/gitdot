"use client";

import type { ReviewDiffResource } from "gitdot-api";
import { Undo2 } from "lucide-react";
import { useParams } from "next/navigation";
import { Suspense, use } from "react";
import type { ResourcePromisesType } from "@/(main)/[owner]/[repo]/resources";
import { UserImage } from "@/(main)/[owner]/ui/user-image";
import { UserSlug } from "@/(main)/[owner]/ui/user-slug";
import Link from "@/ui/link";
import { timeAgo } from "@/util";
import { Sidebar, SidebarContent } from "@/ui/sidebar";
import type { Resources } from "../layout";

type ResourcePromises = ResourcePromisesType<Resources>;

export function ReviewSidebar({
  owner,
  repo,
  promises,
}: {
  owner: string;
  repo: string;
  promises: ResourcePromises;
}) {
  return (
    <Sidebar>
      <SidebarContent className="overflow-auto">
        <div className="flex flex-col w-full">
          <Suspense>
            <ReviewSidebarContent
              owner={owner}
              repo={repo}
              promises={promises}
            />
          </Suspense>
        </div>
      </SidebarContent>
    </Sidebar>
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

  return (
    <>
      <Link
        href={`/${owner}/${repo}/reviews`}
        className="sticky top-0 bg-background flex items-center justify-between px-2 h-9 z-10 hover:bg-accent/50 cursor-default"
      >
        <h3 className="text-xs font-semibold text-muted-foreground uppercase tracking-wide">
          Reviews/{review.number}
        </h3>
        <Undo2 size={14} className="text-muted-foreground -translate-y-px" />
      </Link>
      <Link
        href={`/${owner}/${repo}/reviews/${review.number}`}
        className={`sticky top-9 border-t border-b px-2 pt-1 pb-2 z-10 hover:bg-accent/50 cursor-default flex flex-col gap-0.5 ${position ? "bg-background" : "bg-sidebar"}`}
      >
        <p className="text-xs leading-snug line-clamp-2">{review.title}</p>
        <div className="flex items-center justify-between text-xs text-muted-foreground">
          <div className="flex items-center gap-1 min-w-0">
            <UserImage userId={review.author?.id} px={14} />
            {review.author && <UserSlug user={review.author} />}
          </div>
          <span className="shrink-0">{timeAgo(new Date(review.created_at))}</span>
        </div>
      </Link>
      <span className="flex items-center h-9 px-2 text-xs font-semibold text-muted-foreground uppercase tracking-wide border-b">
        Diffs
      </span>
      <div className="border-b">
        {review.diffs.map((diff, i) => {
          const isActive = position === String(diff.position);
          const isFirst = i === 0;
          const isLast = i === review.diffs.length - 1;
          return (
            <ReviewDiffRow
              key={diff.id}
              owner={owner}
              repo={repo}
              reviewNumber={review.number}
              diff={diff}
              isActive={isActive}
              isFirst={isFirst}
              isLast={isLast}
            />
          );
        })}
      </div>
    </>
  );
}

function ReviewDiffRow({
  owner,
  repo,
  reviewNumber,
  diff,
  isActive,
  isFirst,
  isLast,
}: {
  owner: string;
  repo: string;
  reviewNumber: number;
  diff: ReviewDiffResource;
  isActive: boolean;
  isFirst: boolean;
  isLast: boolean;
}) {
  const title = diff.message.split("\n")[0];
  return (
    <Link
      href={`/${owner}/${repo}/reviews/${reviewNumber}/diffs/${diff.position}`}
      prefetch={true}
      className={`flex flex-col py-1 w-full h-16 border-t border-b hover:bg-accent/50 select-none cursor-default px-2 ${isActive ? `bg-sidebar border-border ${isFirst ? "border-t-transparent" : ""} ${isLast ? "border-b-transparent" : ""}` : "border-transparent"}`}
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
