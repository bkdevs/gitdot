import { Sidebar, SidebarContent } from "@/ui/sidebar";
import { Suspense } from "react";
import { RepoSidebarCommits } from "../../ui/sidebar/repo-sidebar-commits";

const SIDEBAR_WIDTH = "15rem";

export default function Layout({ children }: { children: React.ReactNode }) {
  return (
    <div className="flex h-full w-full">
      <div className="hidden md:flex flex-col h-full border-r shrink-0">
        <Sidebar className="bg-background h-full!" style={{ width: SIDEBAR_WIDTH }}>
          <SidebarContent className="overflow-auto">
            <Suspense fallback={<div>Loading</div>}>
              <RepoSidebarCommits />
            </Suspense>
          </SidebarContent>
        </Sidebar>
      </div>
      <div className="flex-1 min-w-0 overflow-auto scrollbar-thin">
        {children}
      </div>
    </div>
  );
}
