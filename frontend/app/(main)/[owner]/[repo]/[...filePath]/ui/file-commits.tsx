"use client";

import { usePathname, useSearchParams } from "next/navigation";
import type { RepositoryCommits } from "@/lib/dto";
import Link from "@/ui/link";
import { timeAgo } from "@/util";

function FileCommit({
  commit,
  isSelected,
  href,
}: {
  commit: {
    sha: string;
    message: string;
    author: string | { id?: string; name: string; email: string };
    date: string;
  };
  isSelected: boolean;
  href: string;
}) {
  const author =
    typeof commit.author === "string" ? commit.author : commit.author.name;

  return (
    // TODO: if we move this to be a client-side component and fetches we can safe on things
    //  1: making it a single call to fetch all files (getRepositoryFiles)
    //  2: avoid re-fetching getRepositoryFileCommits for each of the prelinks
    // we likely need to do this at the point that we also consider movingn into more diff-based functionality and etc.
    <Link
      href={href}
      prefetch={true}
      className={`flex w-full border-b hover:bg-accent/50 select-none cursor-default py-2 px-2 ${
        isSelected ? "bg-sidebar" : ""
      }`}
    >
      <div className="flex flex-col w-full justify-start items-start min-w-0">
        <div className="text-sm truncate mb-0.5 w-full">{commit.message}</div>

        <div className="text-xs text-muted-foreground flex items-center gap-1 w-full min-w-0">
          <span className="truncate min-w-0">{author}</span>
          <span className="shrink-0">•</span>
          <span className="shrink-0">{commit.sha.substring(0, 7)}</span>
          <span className="ml-auto shrink-0">
            {timeAgo(new Date(commit.date))}
          </span>
        </div>
      </div>
    </Link>
  );
}

export function FileCommits({
  commits,
  selectedCommitSha,
}: {
  commits: RepositoryCommits;
  selectedCommitSha: string;
}) {
  const pathname = usePathname();
  const params = useSearchParams();

  const getCommitHref = (sha: string) => {
    const isLatest = sha === commits.commits[0]?.sha;
    const newParams = new URLSearchParams(params);

    if (isLatest) {
      newParams.delete("ref");
    } else {
      newParams.set("ref", sha.substring(0, 7));
      newParams.delete("lines");
    }

    const queryString = newParams.toString();
    return queryString ? `${pathname}?${queryString}` : pathname;
  };

  return (
    <div className="w-64 h-full border-l flex flex-col">
      <div className="flex-1 overflow-auto scrollbar-none">
        {commits.commits.map((commit) => (
          <FileCommit
            key={commit.sha}
            commit={commit}
            isSelected={selectedCommitSha === commit.sha.substring(0, 7)}
            href={getCommitHref(commit.sha)}
          />
        ))}
      </div>
    </div>
  );
}
