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
}: {
  owner: string;
  repo: string;
  folders: Map<string, string[]>;
  entries: Map<string, RepositoryTreeEntry>;
  commits: RepositoryCommit[];
}) {
  const pathname = usePathname();

  const getSidebarContent = () => {
    const path = pathname.replace(`/${owner}/${repo}`, "") || "/";

    if (/^\/commits\/[^/]+/.test(path)) {
      return <RepoSidebarCommits owner={owner} repo={repo} commits={commits} />;
    }

    const isNavRoute =
      path === "/" ||
      path === "/files" ||
      path === "/commits" ||
      path.startsWith("/commits") ||
      path === "/questions" ||
      path.startsWith("/questions/");

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

    return <RepoSidebarNav owner={owner} repo={repo} currentPath={path} />;
  };

  return (
    <div className="hidden md:flex md:flex-col h-full border-r shrink-0">
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
