"use client";

import { Code2, GitCommit, Home, MessageCircleQuestion } from "lucide-react";
import Link from "next/link";
import { Fragment } from "react";
import { usePathname } from "next/navigation";
import type { RepositoryCommit, RepositoryTreeEntry } from "@/lib/dto";
import { Sidebar, SidebarContent } from "@/ui/sidebar";
import { formatDate, formatTime } from "@/util";
import { RepoSidebarFiles } from "./sidebar/repo-sidebar-files";
import { groupCommitsByDate } from "../util/commit";

const SIDEBAR_WIDTH = "15rem";

const navItems = [
  { path: "", label: "Home", icon: Home },
  { path: "files", label: "Files", icon: Code2 },
  { path: "commits", label: "Commits", icon: GitCommit },
  { path: "questions", label: "Questions", icon: MessageCircleQuestion },
];

function SidebarNav({ repo, currentPath }: { repo: string; currentPath: string }) {
  const getIsActive = (itemPath: string) => {
    if (itemPath === "") {
      return currentPath === "/" || currentPath === "";
    }
    return currentPath === `/${itemPath}` || currentPath.startsWith(`/${itemPath}/`);
  };

  return (
    <div className="flex flex-col w-full">
      {navItems.map((item) => {
        const isActive = getIsActive(item.path);
        return (
          <Link
            key={item.label}
            href={item.path ? `/${repo}/${item.path}` : `/${repo}`}
            className={`flex flex-row w-full px-2 h-9 items-center border-b select-none cursor-default text-sm hover:bg-accent/50 ${
              isActive ? "bg-sidebar" : ""
            }`}
            prefetch={true}
          >
            <item.icon className="size-4" />
            <span className="ml-2">{item.label}</span>
          </Link>
        );
      })}
    </div>
  );
}

function SidebarCommits({
  repo,
  commits,
}: {
  repo: string;
  commits: RepositoryCommit[];
}) {
  const commitsByDate = groupCommitsByDate(commits);

  return (
    <div className="flex flex-col w-full">
      {commitsByDate.map(([date, dateCommits]) => (
        <Fragment key={date}>
          <div className="sticky top-0 bg-background flex items-center border-b px-2 h-9 z-10">
            <h3 className="text-xs font-semibold text-muted-foreground uppercase tracking-wide">
              {formatDate(date)}
            </h3>
          </div>
          {dateCommits.map((commit) => (
            <Link
              key={commit.sha}
              href={`/${repo}/commits/${commit.sha.substring(0, 7)}`}
              className="flex w-full border-b hover:bg-accent/50 select-none cursor-default py-2 px-2"
              prefetch={true}
            >
              <div className="flex flex-col w-full justify-start items-start min-w-0">
                <div className="text-sm truncate mb-0.5 w-full">
                  {commit.message}
                </div>
                <div className="text-xs text-muted-foreground flex items-center gap-1 w-full min-w-0">
                  <span className="truncate min-w-0">{commit.author}</span>
                  <span className="shrink-0">•</span>
                  <span className="shrink-0">
                    {formatTime(new Date(commit.date))}
                  </span>
                </div>
              </div>
            </Link>
          ))}
        </Fragment>
      ))}
    </div>
  );
}

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
      return <SidebarCommits repo={repo} commits={commits} />;
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

    return <SidebarNav repo={repo} currentPath={pathWithoutRepo} />;
  };

  return (
    <div className="hidden md:flex md:flex-col h-full border-r shrink-0">
      <Sidebar className="bg-background h-full!" style={{ width: SIDEBAR_WIDTH }}>
        <SidebarContent className="overflow-auto">
          {getSidebarContent()}
        </SidebarContent>
      </Sidebar>
    </div>
  );
}
