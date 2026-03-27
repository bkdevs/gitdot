"use client";

import type {
  RepositoryBlobResource,
  RepositoryCommitResource,
} from "gitdot-api";
import type { Root } from "hast";
import { useSearchParams } from "next/navigation";
import { useEffect, useState } from "react";
import { useWorkerContext } from "@/(main)/context/worker";
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
  const { highlightFile } = useWorkerContext();
  const [_blobs, setBlobs] = useState<RepositoryBlobResource[]>([]);
  const [_hasts, setHasts] = useState<Record<string, Root>>({});
  const params = useSearchParams();
  const ref = params.get("ref");

  useEffect(() => {
    const refs = commits.map((c) => c.sha);
    if (refs.length === 0) return;

    async function fetchBlobs() {
      const res = await getRepositoryBlobsAction(owner, repo, refs, path);
      const blobs = res?.blobs ?? [];
      setBlobs(blobs);
      return blobs;
    }

    async function highlightBlobs(blobs: RepositoryBlobResource[]) {
      const fileBlobs = blobs.filter((b) => b.type === "file");
      const entries = await Promise.all(
        fileBlobs.map(async (b) => [b.commit_sha, await highlightFile(b.path, b.content)] as const),
      );
      setHasts(Object.fromEntries(entries));
    }

    fetchBlobs().then((blobs) => blobs && highlightBlobs(blobs));
  }, [commits, owner, path, repo, highlightFile]);

  const open = useRightSidebar();
  if (!open) return null;

  const selectedCommitSha = ref ?? commits[0]?.sha.substring(0, 7) ?? "";

  return (
    <div className="w-64 h-full border-l flex flex-col">
      <div className="flex-1 overflow-auto scrollbar-none">
        {commits.map((commit) => (
          <FileCommit
            key={commit.sha}
            commit={commit}
            isSelected={selectedCommitSha === commit.sha.substring(0, 7)}
            isLatest={commit.sha === commits[0]?.sha}
            owner={owner}
            repo={repo}
            path={path}
          />
        ))}
      </div>
    </div>
  );
}

function FileCommit({
  commit,
  isSelected,
  isLatest,
  owner,
  repo,
  path,
}: {
  commit: {
    sha: string;
    message: string;
    author: { id?: string; name: string; email: string };
    date: string;
  };
  isSelected: boolean;
  isLatest: boolean;
  owner: string;
  repo: string;
  path: string;
}) {
  const params = useSearchParams();
  const newParams = new URLSearchParams(params);

  if (isLatest) {
    newParams.delete("ref");
  } else {
    newParams.set("ref", commit.sha.substring(0, 7));
    newParams.delete("lines");
  }

  const queryString = newParams.toString();
  const href = queryString
    ? `/${owner}/${repo}/${path}?${queryString}`
    : `/${owner}/${repo}/${path}`;

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
