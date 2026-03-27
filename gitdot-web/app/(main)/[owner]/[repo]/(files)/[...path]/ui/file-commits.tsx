"use client";

import type {
  RepositoryBlobsResource,
  RepositoryCommitResource,
} from "gitdot-api";
import { usePathname, useSearchParams } from "next/navigation";
import { useEffect, useState } from "react";
import { useRightSidebar } from "@/(main)/hooks/use-sidebar";
import { getRepositoryBlobsAction } from "@/actions/repository";
import Link from "@/ui/link";
import { timeAgo } from "@/util";

export function FileCommits({
  commits,
  owner,
  repo,
  path,
}: {
  commits: RepositoryCommitResource[];
  owner: string;
  repo: string;
  path: string;
}) {
  const pathname = usePathname();
  const params = useSearchParams();
  const [blobs, setBlobs] = useState<RepositoryBlobsResource | null>(null);

  useEffect(() => {
    const refs = commits.map((c) => c.sha);
    if (refs.length === 0) return;
    getRepositoryBlobsAction(owner, repo, refs, path).then(setBlobs);
  }, [commits, owner, path, repo]);

  useEffect(() => {
    console.log(blobs);
  }, [blobs]);

  const open = useRightSidebar();
  if (!open) return null;

  const ref = params.get("ref");
  const selectedCommitSha = ref ?? commits[0]?.sha.substring(0, 7) ?? "";

  const getCommitHref = (sha: string) => {
    const isLatest = sha === commits[0]?.sha;
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
        {commits.map((commit) => (
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

function FileCommit({
  commit,
  isSelected,
  href,
}: {
  commit: {
    sha: string;
    message: string;
    author: { id?: string; name: string; email: string };
    date: string;
  };
  isSelected: boolean;
  href: string;
}) {
  const author = commit.author.name;

  return (
    <Link
      href={href}
      tabIndex={-1}
      className={`flex w-full border-b focus:bg-accent/50 select-none cursor-default py-2 px-2 focus:outline-none ${
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
