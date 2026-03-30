"use client";

import { formatDateTime } from "@/util";
import type { RepositoryCommitResource } from "gitdot-api";

export function FileCommitHeader({
  commit,
}: {
  commit: RepositoryCommitResource;
}) {
  const author = commit.author.name;

  return <div className="sticky top-0 z-10 bg-background shrink-0 border-border border-b p-2 h-16">
    <div className="flex items-center gap-1 text-xs text-muted-foreground mb-1">
      <span>{author}</span>
      <span>•</span>
      <span>{formatDateTime(new Date(commit.date))}</span>
    </div>
    <div className="text-sm text-primary">{commit.message}</div>
  </div>
}
