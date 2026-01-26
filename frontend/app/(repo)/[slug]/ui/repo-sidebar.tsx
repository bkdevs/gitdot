"use client";

import { usePathname } from "next/navigation";
import type { RepositoryCommit, RepositoryTreeEntry } from "@/lib/dto";
import { Sidebar, SidebarContent } from "@/ui/sidebar";
import { RepoSidebarCommits } from "./sidebar/repo-sidebar-commits";
import { RepoSidebarFiles } from "./sidebar/repo-sidebar-files";
import { RepoSidebarNav } from "./sidebar/repo-sidebar-nav";

const SIDEBAR_WIDTH = "15rem";

export function RepoSidebar({
  repo,
  folders,
  entries,
  commits,
}: {
  repo: string;
  folders: Map<string, string[]>;
  entries: Map<string, RepositoryTreeEntry>;
  commits: RepositoryCommit[];
}) {
  const pathname = usePathname();

  const getSidebarContent = () => {
    const pathWithoutRepo = pathname.replace(`/${repo}`, "") || "/";

    if (/^\/commits\/[^/]+/.test(pathWithoutRepo)) {
      return <RepoSidebarCommits repo={repo} commits={commits} />;
    }

    const isNavRoute =
      pathWithoutRepo === "/" ||
      pathWithoutRepo === "/files" ||
      pathWithoutRepo === "/commits" ||
      pathWithoutRepo.startsWith("/commits") ||
      pathWithoutRepo === "/questions" ||
      pathWithoutRepo.startsWith("/questions/");

    if (!isNavRoute && pathWithoutRepo !== "/") {
      const currentPath = pathWithoutRepo.slice(1);
      return (
        <RepoSidebarFiles
          repo={repo}
          folders={folders}
          entries={entries}
          currentPath={currentPath}
        />
      );
    }

    return <RepoSidebarNav repo={repo} currentPath={pathWithoutRepo} />;
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
