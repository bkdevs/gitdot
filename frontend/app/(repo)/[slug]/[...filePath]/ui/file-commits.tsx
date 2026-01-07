"use client";

import { useState } from "react";
import type { RepositoryCommit } from "@/lib/dto";
import { timeAgo } from "@/util";

export function FileCommits({ commits }: { commits: RepositoryCommit[] }) {
  const [selectedSha, setSelectedSha] = useState<string | null>(null);

  return (
    <div className="w-64 h-full border-l flex flex-col">
      <div className="flex-1 overflow-auto scrollbar-none">
        {commits.map((commit) => (
          <FileCommit
            key={commit.sha}
            commit={commit}
            isSelected={selectedSha === commit.sha}
            onClick={() => setSelectedSha(commit.sha)}
          />
        ))}
      </div>
    </div>
  );
}

function FileCommit({
  commit,
  isSelected,
  onClick,
}: {
  commit: RepositoryCommit;
  isSelected: boolean;
  onClick: () => void;
}) {
  return (
    <button
      type="button"
      className={`flex w-full border-b hover:bg-accent/50 select-none py-2 px-2 text-left ${
        isSelected ? "bg-accent" : ""
      }`}
      onClick={onClick}
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
    </button>
  );
}
