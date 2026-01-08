"use client";

import Link from "next/link";

export function FolderHeader({
  repo,
  folderPath,
}: {
  repo: string;
  folderPath: string;
}) {
  return <div className="flex flex-row w-full h-9 items-center border-b"></div>;
}
