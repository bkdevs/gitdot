"use client";

import { usePathname } from "next/navigation";
import { Fragment } from "react";
import type { RepositoryCommit } from "@/lib/dto";
import Link from "@/ui/link";
import { formatDateKey, formatTime } from "@/util";
import { groupCommitsByDate } from "../../util/commit";

export function RepoSidebarCommits({
  commits,
}: {
  commits: RepositoryCommit[];
}) {
  const pathname = usePathname();
  const [owner, repo, section, currentSha] = pathname
    .split("/")
    .filter(Boolean);

  if (section !== "commits") {
    throw new Error(`Expected commits route, got: ${pathname}`);
  }

  const commitsByDate = groupCommitsByDate(commits);

  return (
    <div className="flex flex-col w-full">
      {commitsByDate.map(([date, dateCommits]) => (
        <Fragment key={date}>
          <div className="sticky top-0 bg-background flex items-center border-b px-2 h-9 z-10">
            <h3 className="text-xs font-semibold text-muted-foreground uppercase tracking-wide">
              {formatDateKey(date)}
            </h3>
          </div>
          {dateCommits.map((commit) => {
            const isActive = currentSha === commit.sha.substring(0, 7);
            const author = typeof commit.author === "string" ? commit.author : commit.author.name;
            return (
              <Link
                key={commit.sha}
                href={`/${owner}/${repo}/commits/${commit.sha.substring(0, 7)}`}
                className={`flex w-full border-b hover:bg-accent/50 select-none cursor-default py-2 px-2 ${
                  isActive && "bg-sidebar"
                }`}
                prefetch={true}
              >
                <div className="flex flex-col w-full justify-start items-start min-w-0">
                  <div className="text-sm truncate mb-0.5 w-full">
                    {commit.message}
                  </div>
                  <div className="text-xs text-muted-foreground flex items-center gap-1 w-full min-w-0">
                    <span className="truncate min-w-0">{author}</span>
                    <span className="shrink-0">•</span>
                    <span className="shrink-0">
                      {formatTime(new Date(commit.date))}
                    </span>
                  </div>
                </div>
              </Link>
            );
          })}
        </Fragment>
      ))}
    </div>
  );
}
