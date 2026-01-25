"use client";

import { Circle } from "lucide-react";
import Link from "next/link";
import { usePathname } from "next/navigation";

export function RepoHeader({ repo }: { repo: string }) {
  const pathname = usePathname();

  const repoPrefix = `/${repo}`;
  const remainingPath = pathname.startsWith(repoPrefix)
    ? pathname.slice(repoPrefix.length)
    : "";

  const pathLinks: React.ReactNode[] = [
    <Link
      className="hover:underline"
      href={`/${repo}`}
      key="repo-root"
      prefetch={true}
    >
      {repo}
    </Link>,
  ];

  if (remainingPath && remainingPath !== "/") {
    const segments = remainingPath.replace(/^\//, "").split("/");
    let accumulatedPath = "";

    segments.forEach((segment, index) => {
      accumulatedPath += `/${segment}`;
      pathLinks.push(<span key={`${segment}-separator-${index}`}>/</span>);
      pathLinks.push(
        <Link
          className="hover:underline"
          href={`/${repo}${accumulatedPath}`}
          key={`${segment}-${index}`}
          prefetch={true}
        >
          {segment}
        </Link>,
      );
    });
  }

  return (
    <div className="shrink-0 flex flex-row w-full h-9 items-center border-b bg-sidebar">
      <div className="flex-1 ml-2.75 text-sm font-mono flex items-center">
        <Circle className="size-2 fill-current mr-3.25" />
        {pathLinks}
      </div>
    </div>
  );
}
