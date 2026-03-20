"use client";

import type { RepositoryCommitResource } from "gitdot-api";
import { useParams } from "next/navigation";
import Link from "@/ui/link";
import { formatDateTime, pluralize } from "@/util";
import { DiffStatBar } from "../[sha]/ui/diff-stat-bar";

export function CommitsList({
  commits,
}: {
  commits: RepositoryCommitResource[];
}) {
  return (
    <div className="flex flex-col flex-1 min-h-0 overflow-y-auto scrollbar-thin">
      {commits.map((commit) => (
        <CommitRow key={commit.sha} commit={commit} />
      ))}
    </div>
  );
}

function CommitRow({ commit }: { commit: RepositoryCommitResource }) {
  const { owner, repo } = useParams<{ owner: string; repo: string }>();

  const filesChanged = commit.diffs.length;
  const linesAdded = commit.diffs.reduce((sum, d) => sum + d.lines_added, 0);
  const linesRemoved = commit.diffs.reduce(
    (sum, d) => sum + d.lines_removed,
    0,
  );

  return (
    <Link
      key={commit.sha}
      href={`/${owner}/${repo}/commits/${commit.sha.substring(0, 7)}`}
      data-page-item
      tabIndex={-1}
      className="flex w-full border-b cursor-default focus:bg-accent/50 focus:outline-none select-none"
      prefetch={true}
    >
      <div
        className="grid w-full h-18 p-2 min-w-0"
        style={{ gridTemplateColumns: "1fr 40rem" }}
      >
        <div className="flex flex-col justify-start items-start min-w-0">
          <div className="text-xs text-muted-foreground flex items-center gap-1">
            <span className="truncate min-w-0 underline transition-colors hover:text-foreground cursor-pointer">{commit.author.name}</span>
            <span className="shrink-0">
              {formatDateTime(new Date(commit.date))}
            </span>
          </div>
          <div className="text-sm truncate mb-0.5 w-full">{commit.message.split("\n")[0]}</div>
          {filesChanged > 0 && (
            <div className="text-xs text-muted-foreground">
              {pluralize(filesChanged, "file")} changed,{" "}
              {pluralize(linesAdded, "line")} added,{" "}
              {pluralize(linesRemoved, "line")} removed
            </div>
          )}
        </div>
        <div className="flex flex-col h-full items-end justify-start gap-0.5 overflow-hidden">
          {commit.diffs.map((diff) => (
            <span
              key={diff.path}
              className="font-mono text-xs text-muted-foreground flex items-center w-full"
            >
              <span className="truncate flex-1 mr-2">{diff.path}</span>
              <span className="text-muted-foreground w-6 text-right mr-1.5 shrink-0">
                {diff.lines_added + diff.lines_removed}
              </span>
              <DiffStatBar
                added={diff.lines_added}
                removed={diff.lines_removed}
              />
            </span>
          ))}
        </div>
      </div>
    </Link>
  );
}
