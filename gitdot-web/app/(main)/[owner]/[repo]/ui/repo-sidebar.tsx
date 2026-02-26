"use client";

import { usePathname } from "next/navigation";
import type { RepositoryCommit, RepositoryTreeEntry } from "@/lib/dto";
import { Sidebar, SidebarContent } from "@/ui/sidebar";
import { RepoSidebarCommits } from "./sidebar/repo-sidebar-commits";
import { RepoSidebarFiles } from "./sidebar/repo-sidebar-files";
import { RepoSidebarNav } from "./sidebar/repo-sidebar-nav";

const SIDEBAR_WIDTH = "15rem";

export function RepoSidebar({
  owner,
  repo,
  folders,
  entries,
  commits,
  showSettings,
}: {
  owner: string;
  repo: string;
  folders: Map<string, string[]>;
  entries: Map<string, RepositoryTreeEntry>;
  commits: RepositoryCommit[];
  showSettings?: boolean;
}) {
  const pathname = usePathname();
  const path = pathname.replace(`/${owner}/${repo}`, "") || "/";

  const getSidebarContent = () => {
    if (/^\/commits\/[^/]+/.test(path)) {
      return <RepoSidebarCommits commits={commits} />;
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
        <RepoSidebarFiles
          owner={owner}
          repo={repo}
          folders={folders}
          entries={entries}
          currentPath={currentPath}
        />
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
          {getSidebarContent()}
        </SidebarContent>
      </Sidebar>
    </div>
  );
}
