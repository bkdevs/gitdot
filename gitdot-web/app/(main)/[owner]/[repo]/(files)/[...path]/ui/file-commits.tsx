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
import { computeCommitDiffs } from "../util";
import { useFileViewerContext } from "./file-viewer-context";

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
  const { setHast } = useFileViewerContext();
  const [blobHasts, setBlobHasts] = useState<Record<string, Root>>({});
  const [diffHasts, setDiffHasts] = useState<Record<string, Root>>({});

  const params = useSearchParams();
  const ref = params.get("ref");

  useEffect(() => {
    const oldest = commits[commits.length - 1];
    const refs = [
      ...commits.map((c) => c.sha),
      ...(oldest ? [oldest.parent_sha] : []),
    ];
    if (refs.length === 0) return;

    async function fetchBlobs() {
      const res = await getRepositoryBlobsAction(owner, repo, refs, path);
      return res?.blobs ?? [];
    }

    async function highlightBlobs(blobs: RepositoryBlobResource[]) {
      const entries = await Promise.all(
        blobs
          .filter((b) => b.type === "file")
          .map(
            async (b) =>
              [b.commit_sha, await highlightFile(b.path, b.content)] as const,
          ),
      );
      const hastsMap = Object.fromEntries(entries);

      setBlobHasts(hastsMap);
      setDiffHasts(computeCommitDiffs(commits, blobs, hastsMap));
    }

    fetchBlobs().then((blobs) => blobs && highlightBlobs(blobs));
  }, [commits, owner, path, repo, highlightFile]);

  const [hoveredSha, setHoveredSha] = useState<string | null>(null);

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
            isHovered={hoveredSha === commit.sha}
            isLatest={commit.sha === commits[0]?.sha}
            owner={owner}
            repo={repo}
            path={path}
            onHover={() => {
              setHoveredSha(commit.sha);
              setHast(diffHasts[commit.sha] ?? blobHasts[commit.sha]);
            }}
          />
        ))}
      </div>
    </div>
  );
}

function FileCommit({
  commit,
  isSelected,
  isHovered,
  isLatest,
  owner,
  repo,
  path,
  onHover,
}: {
  commit: {
    sha: string;
    message: string;
    author: { id?: string; name: string; email: string };
    date: string;
  };
  isSelected: boolean;
  isHovered: boolean;
  isLatest: boolean;
  owner: string;
  repo: string;
  path: string;
  onHover: () => void;
}) {
  const params = useSearchParams();
  const newParams = new URLSearchParams(params);
  if (isLatest) {
    newParams.delete("ref");
  } else {
    newParams.set("ref", commit.sha.substring(0, 7));
  }
  newParams.delete("lines");

  const queryString = newParams.toString();
  const href = queryString
    ? `/${owner}/${repo}/${path}?${queryString}`
    : `/${owner}/${repo}/${path}`;

  return (
    <Link
      href={href}
      tabIndex={-1}
      className={`flex w-full border-b hover:bg-accent/50 focus:bg-accent/50 select-none cursor-default py-2 px-2 focus:outline-none ${
        isSelected ? "bg-sidebar" : isHovered ? "bg-accent/50" : ""
      }`}
      onMouseEnter={onHover}
    >
      <div className="flex flex-col w-full justify-start items-start min-w-0">
        <div className="text-sm truncate mb-0.5 w-full">{commit.message}</div>

        <div className="text-xs text-muted-foreground flex items-center gap-1 w-full min-w-0">
          <span className="truncate min-w-0">{commit.author.name}</span>
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
