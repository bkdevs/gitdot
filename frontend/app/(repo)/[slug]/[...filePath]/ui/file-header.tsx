"use client";

import type { RepositoryCommit } from "@/lib/dto";
import { timeAgo } from "@/util";

export function FileHeader({
  filePath,
  latestCommit,
}: {
  filePath: string;
  latestCommit: RepositoryCommit;
}) {
  return (
    <div className="flex flex-row w-full h-9 items-center border-b px-2 justify-between">
      <span className="text-sm font-mono">{filePath.split("/").pop()}</span>
      <span className="text-sm font-mono">
        {latestCommit.message.trim()}{" "}
        <span className="text-primary/60">
          ({latestCommit.sha.slice(0, 7)}){" "}
        </span>{" "}
        • {latestCommit.author} {timeAgo(new Date(latestCommit.date))}
      </span>
    </div>
  );
}
