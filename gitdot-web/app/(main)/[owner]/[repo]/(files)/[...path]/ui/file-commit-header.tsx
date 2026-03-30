"use client";

import Link from "@/ui/link";
import { formatDateTime } from "@/util";
import type { RepositoryCommitResource } from "gitdot-api";
import { File, GitCommitHorizontal, X } from "lucide-react";
import { useParams, usePathname, useSearchParams } from "next/navigation";
import { useFileViewerContext } from "./file-viewer-context";

export function FileCommitHeader({
  commit,
}: {
  commit: RepositoryCommitResource;
}) {
  const author = commit.author.name;
  const { setSelectedSha } = useFileViewerContext();
  const pathname = usePathname();
  const searchParams = useSearchParams();
  const params = useParams<{ owner: string; repo: string }>();

  const handleDismiss = () => {
    setSelectedSha(null);
  };

  const newParams = new URLSearchParams(searchParams.toString());
  newParams.set("ref", commit.sha);
  const fileUrl = `${pathname}?${newParams.toString()}`;
  const commitUrl = `/${params.owner}/${params.repo}/commits/${commit.sha}`;

  return (
    <div className="flex flex-row w-full sticky top-0 z-10 bg-background shrink-0 border-border border-b py-1 px-2 min-h-16">
      <div className="flex flex-col min-w-0">
        <div className="flex items-center gap-1 text-xs text-muted-foreground">
          <span>{author}</span>
          <span>•</span>
          <span>{formatDateTime(new Date(commit.date))}</span>
        </div>
        <div className="text-sm text-primary mb-0.5">{commit.message}</div>
        <div className="flex">
          <span className="text-muted-foreground hover:text-foreground hover:underline cursor-pointer text-xs font-mono">
            {commit.sha.slice(0, 7)}
          </span>
        </div>
      </div>
      <div className="flex flex-row gap-4 mt-auto ml-auto shrink-0">
        <Link href={commitUrl} target="_blank" className="flex flex-row text-muted-foreground hover:text-foreground hover:underline items-center gap-1">
          <GitCommitHorizontal className="size-2.5" />
          <span className="text-xs font-mono">open commit</span>
        </Link>
        <Link href={fileUrl} target="_blank" className="flex flex-row text-muted-foreground hover:text-foreground hover:underline items-center gap-1">
          <File className="size-2.5" />
          <span className="text-xs font-mono">open file</span>
        </Link>
        <button type="button" onClick={handleDismiss} className="flex flex-row text-muted-foreground hover:text-foreground hover:underline cursor-pointer items-center gap-1">
          <X className="size-2.5" />
          <span className="text-xs font-mono">dismiss</span>
        </button>
      </div>
    </div>
  );
}
