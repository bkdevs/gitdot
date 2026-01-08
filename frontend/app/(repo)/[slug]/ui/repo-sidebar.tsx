"use client";

import { CircleDot, Code2, GitPullRequest, History } from "lucide-react";
import Link from "next/link";
import { usePathname } from "next/navigation";
import type { RepositoryTree } from "@/lib/dto";
import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@/ui/sidebar";
import { cn } from "@/util";
import { parseRepositoryTree } from "../[...filePath]/util";
import { RepoContextFiles } from "./repo-context-files";
import { RepoIssues } from "./repo-issues";
import { RepoPulls } from "./repo-pulls";
import { RepoSwitcher } from "./repo-switcher";

const SIDEBAR_ICON_WIDTH = "2.25rem";
const SIDEBAR_CONTENT_WIDTH = "13rem";

type ViewType = "code" | "history" | "issues" | "pulls";

function getViewFromPathname(path: string): ViewType {
  if (path.includes("/issues")) return "issues";
  if (path.includes("/pulls")) return "pulls";
  if (path.includes("/history")) return "history";
  return "code";
}

export function RepoSidebar({
  repo,
  tree,
}: {
  repo: string;
  tree: RepositoryTree;
}) {
  const pathname = usePathname();
  const currentView = getViewFromPathname(pathname);
  const currentPath = pathname.replace(`/${repo}`, "").replace(/^\//, "");

  const navItems = [
    { id: "code" as const, icon: Code2, label: "Code", href: `/${repo}` },
    {
      id: "history" as const,
      icon: History,
      label: "History",
      href: `/${repo}/history`,
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
      return <RepoIssues />;
    }
    if (currentView === "pulls") {
      return <RepoPulls />;
    }
    if (currentView === "history") {
      return <div className="p-4 text-muted-foreground">History view</div>;
    }

    const { folders } = parseRepositoryTree(tree);
    return (
      <RepoContextFiles
        repo={repo}
        folders={folders}
        currentPath={currentPath}
      />
    );
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

        <Sidebar className="border-r" style={{ width: SIDEBAR_CONTENT_WIDTH }}>
          <SidebarHeader className="border-b h-9 flex flex-row items-center justify-between">
            <RepoSwitcher />
          </SidebarHeader>
          <SidebarContent>{renderContent()}</SidebarContent>
        </Sidebar>
      </Sidebar>
    </div>
  );
}
