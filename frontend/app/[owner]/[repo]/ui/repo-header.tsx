"use client";

import { Circle } from "lucide-react";
import Link from "next/link";
import { usePathname } from "next/navigation";

export function RepoHeader({ owner, repo }: { owner: string; repo: string }) {
  const pathname = usePathname();
  const segments = pathname.split("/").filter(Boolean);
  const [ownerSegment, repoSegment, ...pathSegments] = segments;

  if (repoSegment !== repo) {
    throw Error("RepoHeader should only be used under repo paths!");
  }

  const pathLinks: React.ReactNode[] = [
    <Link
      className="hover:underline"
      href={`/${owner}`}
      key="owner-segment"
      prefetch={true}
    >
      {owner}
    </Link>,
    <span key="owner-repo-separator">/</span>,
    <Link
      className="hover:underline"
      href={`/${ownerSegment}/${repoSegment}`}
      key="repo-segment"
      prefetch={true}
    >
      {repo}
    </Link>,
  ];

  pathSegments.forEach((segment, index) => {
    const path = `/${ownerSegment}/${repoSegment}/${pathSegments.slice(0, index + 1).join("/")}`;
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
