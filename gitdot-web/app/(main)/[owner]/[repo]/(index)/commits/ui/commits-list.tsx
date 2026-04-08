"use client";

import { useVirtualizer } from "@tanstack/react-virtual";
import type { RepositoryCommitResource } from "gitdot-api";
import { useParams } from "next/navigation";
import { memo, useRef } from "react";
import { UserImage } from "@/(main)/[owner]/ui/user-image";
import Link from "@/ui/link";
import { formatDateTime, timeAgo } from "@/util";

import { CommitPathSummary } from "./commit-path-summary";

export function CommitsList({
  commits,
}: {
  commits: RepositoryCommitResource[];
}) {
  const parentRef = useRef<HTMLDivElement>(null);
  const virtualizer = useVirtualizer({
    count: commits.length,
    getScrollElement: () => parentRef.current,
    estimateSize: () => 73, // h-18 + border-b
    overscan: 10,
  });

  return (
    <div ref={parentRef} className="flex-1 overflow-y-auto scrollbar-thin">
      <div style={{ height: virtualizer.getTotalSize(), position: "relative" }}>
        {virtualizer.getVirtualItems().map((virtualItem) => (
          <div
            key={virtualItem.key}
            style={{
              position: "absolute",
              top: 0,
              left: 0,
              width: "100%",
              height: `${virtualItem.size}px`,
              transform: `translateY(${virtualItem.start}px)`,
            }}
          >
            <CommitRow commit={commits[virtualItem.index]} />
          </div>
        ))}
      </div>
    </div>
  );
}

const CommitRow = memo(function CommitRow({
  commit,
}: {
  commit: RepositoryCommitResource;
}) {
  const { owner, repo } = useParams<{ owner: string; repo: string }>();

  return (
    <Link
      key={commit.sha}
      href={`/${owner}/${repo}/commits/${commit.sha.substring(0, 7)}`}
      data-page-item
      tabIndex={-1}
      className="flex w-full border-b cursor-default focus:bg-accent/50 focus:outline-none select-none"
      prefetch={true}
    >
      <div className="flex flex-row w-full h-18 justify-between items-start p-2 gap-2">
        <div className="flex flex-col w-full h-full min-w-0">
          <span className="text-xs text-muted-foreground">
            {formatDateTime(new Date(commit.date))}
          </span>
          <div className="text-sm text-foreground truncate pb-1">
            {commit.message.split("\n")[0]}
          </div>
          <div className="flex items-center text-muted-foreground text-xs gap-1">
            <UserImage user={commit.author} px={16} />
            <span className="truncate min-w-0 underline transition-colors hover:text-foreground cursor-pointer">
              {commit.author.name}
            </span>
            <span>{timeAgo(new Date(commit.date))}</span>
          </div>
        </div>
        <div className="flex flex-col shrink-0 items-end gap-1">
          <CommitPathSummary
            diffs={commit.diffs}
            totalFiles={commit.diffs.length}
          />
        </div>
      </div>
    </Link>
  );
});
