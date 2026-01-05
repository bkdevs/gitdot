"use client";

import { DownloadIcon, SearchIcon } from "lucide-react";
import { usePathname } from "next/navigation";
import { Sidebar, SidebarContent, SidebarHeader } from "@/ui/sidebar";
import { RepoCommits } from "./repo-commits";
import { RepoIssues } from "./repo-issues";
import { RepoNavigation } from "./repo-navigation";
import { RepoPulls } from "./repo-pulls";
import { RepoSwitcher } from "./repo-switcher";

export function RepoSidebar({
  ...props
}: React.ComponentProps<typeof Sidebar>) {
  const pathname = usePathname();

  const renderContent = () => {
    if (pathname.includes("/issues")) {
      return <RepoIssues />;
    }
    if (pathname.includes("/pulls")) {
      return <RepoPulls />;
    }
    return <RepoCommits />;
  };

  return (
    <Sidebar {...props}>
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
      <RepoNavigation />
      <SidebarContent>{renderContent()}</SidebarContent>
    </Sidebar>
  );
}
