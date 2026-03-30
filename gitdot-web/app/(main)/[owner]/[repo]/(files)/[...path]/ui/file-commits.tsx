"use client";

import type { RepositoryCommitResource } from "gitdot-api";
import { useRightSidebar } from "@/(main)/hooks/use-sidebar";
import { formatDate, timeAgo } from "@/util";
import { DiffStatBar } from "../../../commits/[sha]/ui/diff-stat-bar";
import { useFileViewerContext } from "./file-viewer-context";

export function FileCommits({
  commits,
  path,
}: {
  commits: RepositoryCommitResource[];
  path: string;
}) {
  const { selectedSha, setSelectedSha, setHoveredSha } = useFileViewerContext();
  const commitStats = commits.map((c) => c.diffs.find((d) => d.path === path));

  const open = useRightSidebar();
  if (!open) return null;

  return (
    <div className="w-64 h-full border-l flex flex-col">
      <div
        className="flex-1 overflow-auto scrollbar-none"
        onMouseLeave={() => setHoveredSha(null)}
      >
        {commits.map((commit, i) => (
          <FileCommit
            key={commit.sha}
            commit={commit}
            diffStat={i === commits.length - 1 ? null : commitStats[i]}
            isSelected={selectedSha === commit.sha}
            onHover={() => setHoveredSha(commit.sha)}
            onClick={() =>
              setSelectedSha(selectedSha === commit.sha ? null : commit.sha)
            }
          />
        ))}
      </div>
    </div>
  );
}

function FileCommit({
  commit,
  diffStat,
  isSelected,
  onHover,
  onClick,
}: {
  commit: {
    sha: string;
    message: string;
    author: { id?: string; name: string; email: string };
    date: string;
  };
  diffStat: { lines_added: number; lines_removed: number } | null | undefined;
  isSelected: boolean;
  onHover: () => void;
  onClick: () => void;
}) {
  const date = new Date(commit.date);

  return (
    <button
      type="button"
      className={`flex w-full border-b select-none cursor-default text-left h-16 py-1 px-2 focus:outline-none hover:bg-accent/50 ${isSelected ? "bg-accent/50 shadow-[inset_2px_0_0_color-mix(in_oklch,var(--color-foreground)_60%,transparent)]" : ""}`}
      onMouseEnter={onHover}
      onClick={onClick}
    >
      <div className="flex flex-col w-full justify-start items-start min-w-0">
        <div className="text-xs text-muted-foreground flex items-center w-full min-w-0">
          <span className="shrink-0">{formatDate(date)}</span>
          <span className="ml-auto shrink-0">{timeAgo(date)}</span>
        </div>
        <div className="text-sm truncate pb-0.5 w-full">{commit.message}</div>
        <div className="text-xs text-muted-foreground flex items-center w-full min-w-0">
          <span className="truncate min-w-0 underline cursor-pointer">
            {commit.author.name}
          </span>
          <span className="ml-auto pl-2 shrink-0">
            {diffStat == null ? (
              <span className="text-green-600 font-mono">created</span>
            ) : (
              <DiffStatBar
                added={diffStat.lines_added}
                removed={diffStat.lines_removed}
              />
            )}
          </span>
        </div>
      </div>
    </button>
  );
}
