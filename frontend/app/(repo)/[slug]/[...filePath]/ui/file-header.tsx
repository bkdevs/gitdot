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
    <div className="flex flex-row w-full h-9 items-center border-b px-2 text-sm font-mono">
      {filePath.split("/").pop()}
    </div>
  );
}
