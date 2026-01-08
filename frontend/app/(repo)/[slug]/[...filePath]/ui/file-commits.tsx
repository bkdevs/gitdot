"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";
import { timeAgo } from "@/util";
import { RepositoryCommits } from "@/lib/dto";

function FileCommit({
  commit,
  isSelected,
  href,
}: {
  commit: { sha: string; message: string; author: string; date: string };
  isSelected: boolean;
  href: string;
}) {
  return (
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
          <span className="truncate min-w-0">{commit.author}</span>
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

  const getCommitHref = (sha: string) => {
    const params = new URLSearchParams(window.location.search);

    const isLatest = sha === commits.commits[0]?.sha;

    if (isLatest) {
      params.delete("ref");
    } else {
      params.set("ref", sha.substring(0, 7));
      params.delete("lines");
    }

    return params.toString()
      ? `${pathname}?${params.toString()}`
      : pathname;
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
