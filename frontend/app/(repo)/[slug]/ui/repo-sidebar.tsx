"use client";

import {
  CircleDot,
  Code2,
  DownloadIcon,
  GitPullRequest,
  History,
  SearchIcon,
} from "lucide-react";
import { usePathname, useRouter } from "next/navigation";
import { useCallback, useEffect, useState } from "react";
import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  useSidebar,
} from "@/ui/sidebar";
import { RepoFileTree } from "./repo-file-tree";
import { RepoIssues } from "./repo-issues";
import { RepoPulls } from "./repo-pulls";
import { RepoSwitcher } from "./repo-switcher";
import { cn } from "@/util/cn";

type ViewType = "code" | "history" | "issues" | "pulls";

export function RepoSidebar({
  ...props
}: React.ComponentProps<typeof Sidebar>) {
  const pathname = usePathname();
  const router = useRouter();
  const { setOpen } = useSidebar();

  // Determine initial view from pathname
  const getViewFromPathname = useCallback((path: string): ViewType => {
    if (path.includes("/issues")) return "issues";
    if (path.includes("/pulls")) return "pulls";
    if (path.includes("/history")) return "history";
    return "code";
  }, []);

  const [selectedView, setSelectedView] = useState<ViewType>(() =>
    getViewFromPathname(pathname),
  );

  // Sync view with pathname changes
  useEffect(() => {
    setSelectedView(getViewFromPathname(pathname));
  }, [pathname, getViewFromPathname]);

  const handleViewChange = (view: ViewType) => {
    setSelectedView(view);

    // Navigate to the appropriate route
    // Extract just the repo slug (e.g., "/gitdot" from "/gitdot/issues/1")
    const baseSlug = pathname.split("/").slice(0, 2).join("/");
    if (view === "issues") {
      router.push(`${baseSlug}/issues/1`);
    } else if (view === "pulls") {
      router.push(`${baseSlug}/pulls/1`);
    } else if (view === "history") {
      router.push(`${baseSlug}/history`);
    } else {
      router.push(baseSlug);
    }
    setOpen(true);
  };

  const navItems = [
    { id: "code" as const, icon: Code2, label: "Code" },
    { id: "history" as const, icon: History, label: "History" },
    { id: "issues" as const, icon: CircleDot, label: "Issues" },
    { id: "pulls" as const, icon: GitPullRequest, label: "Pull Requests" },
  ];

  const renderContent = () => {
    if (selectedView === "issues") {
      return <RepoIssues />;
    }
    if (selectedView === "pulls") {
      return <RepoPulls />;
    }
    if (selectedView === "history") {
      return <div className="p-4 text-muted-foreground">History view</div>;
    }
    return <RepoFileTree />;
  };

  return (
    <Sidebar
      collapsible="icon"
      className="overflow-hidden *:data-[sidebar=sidebar]:flex-row"
      {...props}
    >
      {/* Primary icon sidebar */}
      <Sidebar
        collapsible="none"
        className="w-[calc(var(--sidebar-width-icon)+1px)]! border-r"
      >
        <SidebarContent>
          <SidebarGroup className="p-0!">
            <SidebarGroupContent>
              <SidebarMenu className="gap-0">
                {navItems.map((item) => (
                  <SidebarMenuItem className="w-9 h-9 border-b p-0!" key={item.id}>
                    <SidebarMenuButton
                      tooltip={{
                        children: item.label,
                        hidden: false,
                      }}
                      onClick={() => handleViewChange(item.id)}
                      isActive={selectedView === item.id}
                      className={cn(
                        "w-full h-full flex items-center justify-center p-0! rounded-none",
                        selectedView === item.id && "bg-primary/80! text-background!"
                      )}
                    >
                      <item.icon />
                    </SidebarMenuButton>
                  </SidebarMenuItem>
                ))}
              </SidebarMenu>
            </SidebarGroupContent>
          </SidebarGroup>
        </SidebarContent>
      </Sidebar>

      {/* Secondary content sidebar */}
      <Sidebar collapsible="none" className="hidden flex-1 md:flex">
        <SidebarHeader className="border-b h-9 flex flex-row items-center justify-between">
          <RepoSwitcher />
        </SidebarHeader>
        <SidebarContent>{renderContent()}</SidebarContent>
      </Sidebar>
    </Sidebar>
  );
}
