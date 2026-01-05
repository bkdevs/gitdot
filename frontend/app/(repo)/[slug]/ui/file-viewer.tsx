"use client";

import { useEffect, useState } from "react";
import { codeToHtml } from "shiki";

const sampleCode = `"use client";

import { Code2, CircleDot, GitPullRequest, DownloadIcon, SearchIcon } from "lucide-react";
import { usePathname, useRouter } from "next/navigation";
import { useState, useEffect } from "react";
import {
  Sidebar,
  SidebarContent,
  SidebarHeader,
  SidebarGroup,
  SidebarGroupContent,
  SidebarMenu,
  SidebarMenuItem,
  SidebarMenuButton,
  useSidebar,
} from "@/ui/sidebar";
import { RepoFileTree } from "./repo-file-tree";
import { RepoIssues } from "./repo-issues";
import { RepoPulls } from "./repo-pulls";
import { RepoSwitcher } from "./repo-switcher";

type ViewType = "code" | "issues" | "pulls";

export function RepoSidebar({
  ...props
}: React.ComponentProps<typeof Sidebar>) {
  const pathname = usePathname();
  const router = useRouter();
  const { setOpen } = useSidebar();

  // Determine initial view from pathname
  const getViewFromPathname = (path: string): ViewType => {
    if (path.includes("/issues")) return "issues";
    if (path.includes("/pulls")) return "pulls";
    return "code";
  };

  const [selectedView, setSelectedView] = useState<ViewType>(() =>
    getViewFromPathname(pathname),
  );

  // Sync view with pathname changes
  useEffect(() => {
    setSelectedView(getViewFromPathname(pathname));
  }, [pathname]);

  const handleViewChange = (view: ViewType) => {
    setSelectedView(view);

    // Navigate to the appropriate route
    const baseSlug = pathname.split("/").slice(0, 2).join("/");
    if (view === "issues") {
      router.push(\`\${baseSlug}/issues/1\`);
    } else if (view === "pulls") {
      router.push(\`\${baseSlug}/pulls/1\`);
    } else {
      router.push(baseSlug);
    }
    setOpen(true);
  };

  const navItems = [
    { id: "code" as const, icon: Code2, label: "Code" },
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
          <SidebarGroup>
            <SidebarGroupContent className="px-1.5 md:px-0">
              <SidebarMenu>
                {navItems.map((item) => (
                  <SidebarMenuItem key={item.id}>
                    <SidebarMenuButton
                      tooltip={{
                        children: item.label,
                        hidden: false,
                      }}
                      onClick={() => handleViewChange(item.id)}
                      isActive={selectedView === item.id}
                      className="px-2.5 md:px-2"
                    >
                      <item.icon />
                      <span>{item.label}</span>
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
          <div className="flex items-center mr-1">
            <button
              type="button"
              className="text-muted-foreground hover:text-foreground p-1 transition-colors"
            >
              <SearchIcon className="size-4" />
            </button>
            <button
              type="button"
              className="text-muted-foreground hover:text-foreground p-1 transition-colors"
            >
              <DownloadIcon className="size-4" />
            </button>
          </div>
        </SidebarHeader>
        <SidebarContent>{renderContent()}</SidebarContent>
      </Sidebar>
    </Sidebar>
  );
}`;

export function FileViewer() {
  const [highlightedCode, setHighlightedCode] = useState("");

  useEffect(() => {
    async function highlight() {
      const html = await codeToHtml(sampleCode, {
        lang: "tsx",
        theme: "github-light",
      });
      setHighlightedCode(html);
    }
    highlight();
  }, []);

  return (
    <div className="w-full h-full overflow-auto px-2">
      <div
        className="text-sm"
        // biome-ignore lint: required for shiki
        dangerouslySetInnerHTML={{ __html: highlightedCode }}
      />
    </div>
  );
}
