"use client";

import { useVirtualizer } from "@tanstack/react-virtual";
import type { RepositoryCommitResource } from "gitdot-api";
import { useParams } from "next/navigation";
import { memo, useRef } from "react";
import { UserImage } from "@/(main)/[owner]/ui/user/user-image";
import { UserSlug } from "@/(main)/[owner]/ui/user/user-slug";
import { useTimezone } from "@/(main)/context/timezone";
import { prefetchCommitDiff } from "@/(main)/ui/commit-dialog";
import { formatDateIso } from "@/util/date";

export function CommitsList({
  commits,
}: {
  commits: RepositoryCommitResource[];
}) {
  const parentRef = useRef<HTMLDivElement>(null);
  const virtualizer = useVirtualizer({
    count: commits.length,
    getScrollElement: () => parentRef.current,
    estimateSize: () => 29, // h-7 + border-b
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
  const tz = useTimezone();
  const { owner, repo } = useParams<{ owner: string; repo: string }>();
  const href = `/${owner}/${repo}/commits/${commit.sha.substring(0, 7)}`;

  return (
    <div
      data-page-item
      tabIndex={-1}
      onPointerEnter={() =>
        prefetchCommitDiff(commit.owner_name, commit.repo_name, commit.sha)
      }
      onClick={(e) => {
        if (e.metaKey || e.ctrlKey || e.shiftKey) {
          window.open(href, "_blank", "noopener,noreferrer");
          return;
        }
        window.dispatchEvent(
          new CustomEvent("openCommitDialog", { detail: { commit } }),
        );
      }}
      onAuxClick={(e) => {
        if (e.button === 1) {
          e.preventDefault();
          window.open(href, "_blank", "noopener,noreferrer");
        }
      }}
      className="flex w-full h-7 border-b items-center px-2 gap-2 cursor-default hover:bg-accent/50 focus:bg-accent/50 focus:outline-none select-none"
    >
      <div className="flex flex-row items-center gap-2 min-w-0 text-xs">
        <span className="text-muted-foreground shrink-0 tabular-nums">
          {formatDateIso(new Date(commit.date), tz)}
        </span>
        <span className="text-foreground truncate">
          {commit.message.split("\n")[0]}
        </span>
      </div>
      <div className="flex flex-row items-center gap-1.5 ml-auto shrink-0 text-xs text-muted-foreground">
        <UserImage
          userId={commit.author.id}
          username={commit.author.name ?? commit.author.git_name}
          updatedAt={commit.author.image_updated_at}
          px={14}
        />
        <UserSlug user={commit.author} />
      </div>
    </div>
  );
});
