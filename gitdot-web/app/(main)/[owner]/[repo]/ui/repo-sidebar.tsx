"use client";

import { Sidebar, SidebarContent } from "@/ui/sidebar";
import { usePathname } from "next/navigation";
import { Suspense } from "react";
import { RepoSidebarCommits } from "./sidebar/repo-sidebar-commits";
import { RepoSidebarFiles } from "./sidebar/repo-sidebar-files";
import { RepoSidebarNav } from "./sidebar/repo-sidebar-nav";

const SIDEBAR_WIDTH = "15rem";

export function RepoSidebar({
  owner,
  repo,
  showSettings,
}: {
  owner: string;
  repo: string;
  showSettings?: boolean;
}) {
  const pathname = usePathname();
  const path = pathname.replace(`/${owner}/${repo}`, "") || "/";

  const getSidebarContent = () => {
    if (/^\/commits\/[^/]+/.test(path)) {
      return <RepoSidebarCommits />;
    }

    const isNavRoute =
      path === "/" ||
      path === "/files" ||
      path === "/commits" ||
      path.startsWith("/commits") ||
      path === "/questions" ||
      path.startsWith("/questions/") ||
      path === "/builds" ||
      path.startsWith("/builds/") ||
      path === "/settings" ||
      path.startsWith("/settings/");

    if (!isNavRoute && path !== "/") {
      const currentPath = path.slice(1);
      return (
        <RepoSidebarFiles owner={owner} repo={repo} currentPath={currentPath} />
      );
    }

    return (
      <RepoSidebarNav
        owner={owner}
        repo={repo}
        currentPath={path}
        showSettings={showSettings}
      />
    );
  };

  return (
    <div className="hidden md:flex flex-col h-full border-r shrink-0">
      <Sidebar
        className="bg-background h-full!"
        style={{ width: SIDEBAR_WIDTH }}
      >
        <SidebarContent className="overflow-auto">
          <Suspense fallback={<div>Loading</div>}>
            {getSidebarContent()}
          </Suspense>
        </SidebarContent>
      </Sidebar>
    </div>
  );
}
