"use client";

import { useRouter, usePathname } from "next/navigation";
import type { FileHistoryEntry } from "@/lib/dto";
import { timeAgo } from "@/util";

export function FileCommits({
  repo,
  filePath,
  history,
  selectedCommitSha,
}: {
  repo: string;
  filePath: string;
  history: FileHistoryEntry[];
  selectedCommitSha: string;
}) {
  const router = useRouter();
  const pathname = usePathname();

  const handleCommitClick = (sha: string) => {
    const params = new URLSearchParams(window.location.search);

    // Check if clicking the currently selected commit (latest)
    const isLatest = sha === history[0]?.commit.sha;

    if (isLatest) {
      // Remove commit param to show latest
      params.delete("commit");
    } else {
      // Set commit param for historical view
      params.set("commit", sha);
      // Clear lines param when viewing historical commit
      params.delete("lines");
    }

    const newUrl = params.toString()
      ? `${pathname}?${params.toString()}`
      : pathname;

    router.push(newUrl);
  };

  return (
    <div className="w-64 h-full border-l flex flex-col">
      <div className="flex-1 overflow-auto scrollbar-none">
        {history.map((entry) => (
          <FileCommit
            key={entry.commit.sha}
            commit={entry.commit}
            isSelected={selectedCommitSha === entry.commit.sha}
            onClick={() => handleCommitClick(entry.commit.sha)}
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
  commit: { sha: string; message: string; author: string; date: string };
  isSelected: boolean;
  onClick: () => void;
}) {
  return (
    <button
      type="button"
      className={`flex w-full border-b hover:bg-accent/50 select-none py-2 px-2 text-left ${
        isSelected ? "bg-sidebar" : ""
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
