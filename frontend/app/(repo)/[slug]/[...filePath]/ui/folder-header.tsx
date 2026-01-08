"use client";

import Link from "next/link";

export function FolderHeader({
  repo,
  folderPath,
}: {
  repo: string;
  folderPath: string;
}) {
  var path = "";
  const pathSegments = folderPath.split("/");
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
    </div>
  );
}
