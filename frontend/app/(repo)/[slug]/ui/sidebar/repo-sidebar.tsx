"use client";

import { CircleDot, Code2, GitPullRequest, History } from "lucide-react";
import Link from "next/link";
import { usePathname } from "next/navigation";
import type { RepositoryCommit, RepositoryTreeEntry } from "@/lib/dto";
import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@/ui/sidebar";
import { cn } from "@/util";
import { RepoSidebarFiles } from "./repo-sidebar-files";
import { RepoSidebarHeader } from "./repo-sidebar-header";
import { RepoSidebarIssues } from "./repo-sidebar-issues";
import { RepoSidebarPulls } from "./repo-sidebar-pulls";
import { RepoSidebarCommits } from "./repo-sidebar-commits";

const SIDEBAR_ICON_WIDTH = "2.25rem";
const SIDEBAR_CONTENT_WIDTH = "15rem";

type ViewType = "code" | "commits" | "issues" | "pulls";

function getViewFromPathname(path: string): ViewType {
  if (path.includes("/commits")) return "commits";
  if (path.includes("/issues")) return "issues";
  if (path.includes("/pulls")) return "pulls";
  return "code";
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
  commits: RepositoryCommit[]
}) {
  const pathname = usePathname();
  const currentView = getViewFromPathname(pathname);
  const currentPath = pathname.replace(`/${repo}`, "").replace(/^\//, "");

  const navItems = [
    { id: "code" as const, icon: Code2, label: "Code", href: `/${repo}` },
    {
      id: "commits" as const,
      icon: History,
      label: "Commits",
      href: `/${repo}/commits`,
    },
    {
      id: "issues" as const,
      icon: CircleDot,
      label: "Issues",
      href: `/${repo}/issues/1`,
    },
    {
      id: "pulls" as const,
      icon: GitPullRequest,
      label: "Pull Requests",
      href: `/${repo}/pulls/1`,
    },
  ];

  const renderContent = () => {
    if (currentView === "issues") {
      return <RepoSidebarIssues />;
    }
    if (currentView === "pulls") {
      return <RepoSidebarPulls />;
    }
    if (currentView === "commits") {
      return <RepoSidebarCommits repo={repo} commits={commits} />;
    } else {
      return (
        <RepoSidebarFiles
          repo={repo}
          folders={folders}
          entries={entries}
          currentPath={currentPath}
        />
      );
    }
  };

  return (
    <div className="hidden md:flex md:flex-col h-svh">
      <Sidebar className="overflow-hidden flex-row">
        <Sidebar
          className="border-r"
          style={{ width: `calc(${SIDEBAR_ICON_WIDTH} + 1px)` }}
        >
          <SidebarContent>
            <SidebarGroup className="p-0!">
              <SidebarGroupContent>
                <SidebarMenu className="gap-0">
                  {navItems.map((item) => (
                    <SidebarMenuItem
                      className="w-9 h-9 border-b p-0!"
                      key={item.id}
                    >
                      <SidebarMenuButton
                        asChild
                        isActive={currentView === item.id}
                        tooltip={item.label}
                        className={cn(
                          "w-full h-full flex items-center justify-center p-0! rounded-none cursor-default",
                          currentView === item.id &&
                            "bg-primary! text-background!",
                        )}
                      >
                        <Link href={item.href}>
                          <item.icon />
                          <span className="sr-only">{item.label}</span>
                        </Link>
                      </SidebarMenuButton>
                    </SidebarMenuItem>
                  ))}
                </SidebarMenu>
              </SidebarGroupContent>
            </SidebarGroup>
          </SidebarContent>
        </Sidebar>

        <Sidebar
          className="border-r bg-background"
          style={{ width: SIDEBAR_CONTENT_WIDTH }}
        >
          <RepoSidebarHeader repo={repo} />
          <SidebarContent>{renderContent()}</SidebarContent>
        </Sidebar>
      </Sidebar>
    </div>
  );
}
