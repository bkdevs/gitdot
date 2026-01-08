"use client";

import Link from "next/link";

export function FolderHeader({
  repo,
  folderPath,
}: {
  repo: string;
  folderPath: string;
}) {
  return (
    <div className="flex flex-row w-full h-9 items-center border-b px-2 text-sm font-mono">
      <span className="text-sm font-mono">{folderPath.split("/").pop()}/</span>
    </div>
  );
}
