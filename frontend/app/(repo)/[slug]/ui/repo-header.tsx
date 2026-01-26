"use client";

import { Circle } from "lucide-react";
import Link from "next/link";
import { usePathname } from "next/navigation";

export function RepoHeader({ repo }: { repo: string }) {
  const pathname = usePathname();
  const segments = pathname.split("/").filter(Boolean);
  const [repoSegment, ...pathSegments] = segments;

  if (repoSegment !== repo) {
    throw Error("RepoHeader should only be used under repo paths!");
  }

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

  pathSegments.forEach((segment, index) => {
    const path = `/${segments.slice(0, index + 2).join("/")}`;
    pathLinks.push(
      <span key={`sep-${segment}`}>/</span>,
      <Link
        className="hover:underline"
        href={path}
        key={segment}
        prefetch={true}
      >
        {segment}
      </Link>,
    );
  });

  return (
    <div className="shrink-0 flex flex-row w-full h-9 items-center border-b bg-sidebar">
      <div className="flex-1 ml-2.75 text-sm font-mono flex items-center">
        <Circle className="size-2 fill-current mr-3.25" />
        {pathLinks}
      </div>
    </div>
  );
}
