"use client";

import { CircleDot, Code2, GitPullRequest, History } from "lucide-react";
import Link from "next/link";
import { usePathname } from "next/navigation";
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
import { RepoFileTree } from "./repo-file-tree";
import { RepoIssues } from "./repo-issues";
import { RepoPulls } from "./repo-pulls";
import { RepoSwitcher } from "./repo-switcher";

const SIDEBAR_ICON_WIDTH = "2.25rem";
const SIDEBAR_CONTENT_WIDTH = "12.5rem";

type ViewType = "code" | "history" | "issues" | "pulls";

function getViewFromPathname(path: string): ViewType {
  if (path.includes("/issues")) return "issues";
  if (path.includes("/pulls")) return "pulls";
  if (path.includes("/history")) return "history";
  return "code";
}

export function RepoSidebar({
  ...props
}: React.ComponentProps<typeof Sidebar>) {
  const pathname = usePathname();
  const currentView = getViewFromPathname(pathname);
  const baseSlug = pathname.split("/").slice(0, 2).join("/");

  const navItems = [
    { id: "code" as const, icon: Code2, label: "Code", href: baseSlug },
    {
      id: "history" as const,
      icon: History,
      label: "History",
      href: `${baseSlug}/history`,
    },
    {
      id: "issues" as const,
      icon: CircleDot,
      label: "Issues",
      href: `${baseSlug}/issues/1`,
    },
    {
      id: "pulls" as const,
      icon: GitPullRequest,
      label: "Pull Requests",
      href: `${baseSlug}/pulls/1`,
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
    return <RepoFileTree />;
  };

  return (
    <Sidebar className="overflow-hidden flex-row" {...props}>
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
                          "bg-primary/80! text-background!",
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
  );
}
