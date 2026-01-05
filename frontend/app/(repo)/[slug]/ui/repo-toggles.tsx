"use client";

import { CircleDot, Code, GitPullRequest } from "lucide-react";
import Link from "next/link";
import { useParams, usePathname } from "next/navigation";

type Tab = "code" | "issues" | "pulls";

export function RepoToggles() {
  const pathname = usePathname();
  const params = useParams();
  const slug = params.slug as string;

  const getActiveTab = (): Tab => {
    if (pathname.includes("/issues")) return "issues";
    if (pathname.includes("/pulls")) return "pulls";
    return "code";
  };

  const activeTab = getActiveTab();

  const buttonClass = (tab: Tab) =>
    `px-2 h-full flex items-center rounded-xs transition-colors border-l border-border ${
      activeTab === tab
        ? "bg-sidebar-accent text-sidebar-accent-foreground"
        : "text-sidebar-foreground/60 hover:bg-sidebar-accent/50 hover:text-sidebar-accent-foreground"
    }`;

  const getHref = (tab: Tab): string => {
    const base = `/${slug}`;
    if (tab === "code") return base;
    return `${base}/${tab}/1`;
  };

  return (
    <div className="flex flex-row items-stretch h-full">
      <Link
        href={getHref("code")}
        className={buttonClass("code")}
        aria-label="Code"
      >
        <Code
          className="h-4 w-4"
          strokeWidth={activeTab === "code" ? 2.5 : 1.5}
        />
      </Link>
      <Link
        href={getHref("issues")}
        className={buttonClass("issues")}
        aria-label="Issues"
      >
        <CircleDot
          className="h-4 w-4"
          strokeWidth={activeTab === "issues" ? 2.5 : 1.5}
        />
      </Link>
      <Link
        href={getHref("pulls")}
        className={buttonClass("pulls")}
        aria-label="Pull Requests"
      >
        <GitPullRequest
          className="h-4 w-4"
          strokeWidth={activeTab === "pulls" ? 2.5 : 1.5}
        />
      </Link>
    </div>
  );
}
