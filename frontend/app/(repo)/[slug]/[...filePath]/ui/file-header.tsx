"use client";

import Link from "next/link";
import type { RepositoryCommit, RepositoryFile } from "@/lib/dto";

export function FileHeader({
  repo,
  file,
  commit,
}: {
  repo: string;
  file: RepositoryFile;
  commit: RepositoryCommit;
}) {
  var path = "";
  const pathSegments = file.path.split("/");
  const pathLinks: React.ReactNode[] = [];
  pathSegments.forEach((segment, index) => {
    path += `/${segment}`;
    pathLinks.push(
      <Link className="hover:underline" href={`/${repo}${path}`} key={segment}>
        {segment}
      </Link>,
    );
    if (index !== pathSegments.length - 1) {
      pathLinks.push(<span key={`${segment}-separator`}>/</span>);
    }
  });

  return (
    <div className="flex flex-row w-full h-9 items-center border-b">
      <div className="flex-1 ml-2 text-sm font-mono">{pathLinks}</div>
      <div className="border-l w-64 flex flex-row items-center h-full pl-2 text-sm">
        History
      </div>
    </div>
  );
}
