"use client";

import { CircleDot, Code, GitPullRequest } from "lucide-react";
import Link from "next/link";
import { useParams, usePathname } from "next/navigation";

type Tab = "code" | "issues" | "pulls";

export function RepoNavigation() {
  const pathname = usePathname();
  const params = useParams();
  const slug = params.slug as string;

  const getActiveTab = (): Tab => {
    if (pathname.includes("/issues")) return "issues";
    if (pathname.includes("/pulls")) return "pulls";
    return "code";
  };

  const activeTab = getActiveTab();

  const getHref = (tab: Tab): string => {
    const base = `/${slug}`;
    if (tab === "code") return base;
    return `${base}/${tab}/1`;
  };

  const rowClass = (tab: Tab) =>
    `flex items-center gap-2 px-3 py-2 text-sm transition-colors ${
      activeTab === tab
        ? "bg-sidebar-accent text-sidebar-accent-foreground font-medium"
        : "text-sidebar-foreground/60 hover:bg-sidebar-accent/50 hover:text-sidebar-accent-foreground"
    }`;

  return (
    <div className="flex flex-col border-b">
      <Link href={getHref("code")} className={rowClass("code")}>
        <Code
          className="h-4 w-4"
          strokeWidth={activeTab === "code" ? 2.5 : 1.5}
        />
        <span>Code</span>
      </Link>
      <Link href={getHref("issues")} className={rowClass("issues")}>
        <CircleDot
          className="h-4 w-4"
          strokeWidth={activeTab === "issues" ? 2.5 : 1.5}
        />
        <span>Issues</span>
      </Link>
      <Link href={getHref("pulls")} className={rowClass("pulls")}>
        <GitPullRequest
          className="h-4 w-4"
          strokeWidth={activeTab === "pulls" ? 2.5 : 1.5}
        />
        <span>Reviews</span>
      </Link>
    </div>
  );
}
