"use client";

import { CircleDashedIcon } from "lucide-react";
import { usePathname } from "next/navigation";
import Link from "@/ui/link";
import { OverlayScroll } from "@/ui/scroll";
import { Sidebar, SidebarContent } from "@/ui/sidebar";

export const NAV_ITEMS = [
  { path: "", label: "/home" },
  { path: "files", label: "/files" },
  { path: "commits", label: "/commits" },
  { path: "questions", label: "/questions" },
  { path: "reviews", label: "/reviews" },
  { path: "builds", label: "/builds" },
  { path: "settings", label: "/settings", protected: true },
];

export function LayoutClient({
  owner,
  repo,
  showSettings,
  children,
}: {
  owner: string;
  repo: string;
  showSettings?: boolean;
  children: React.ReactNode;
}) {
  return (
    <>
      <IndexSidebar owner={owner} repo={repo} showSettings={showSettings} />
      <OverlayScroll> {children} </OverlayScroll>
    </>
  );
}

function IndexSidebar({
  owner,
  repo,
  showSettings,
}: {
  owner: string;
  repo: string;
  showSettings?: boolean;
}) {
  const pathname = usePathname();
  const path = pathname.replace(`/${owner}/${repo}`, "") || "/";

  const items = NAV_ITEMS.filter((i) => !i.protected || showSettings);
  const isActive = (itemPath: string) => {
    const full = `/${itemPath}`;
    return path === full || path.startsWith(`${full}/`);
  };

  return (
    <Sidebar>
      <SidebarContent className="overflow-auto">
        <div className="flex flex-col w-full">
          <div className="flex flex-row w-full h-9 items-center border-b bg-background select-none text-sm font-mono tracking-tight">
            <CircleDashedIcon className="size-3.5 ml-2 shrink-0 text-foreground" />
            <span className="ml-auto mr-2">
              <Link
                href={`/${owner}`}
                className="text-muted-foreground underline decoration-transparent hover:decoration-current transition-colors duration-200"
              >
                {owner}
              </Link>
              /
              <Link
                href={`/${owner}/${repo}`}
                className="underline decoration-transparent hover:decoration-current transition-colors duration-200"
              >
                {repo}
              </Link>
            </span>
          </div>
          {items.map((item) => {
            const active = isActive(item.path);
            return (
              <Link
                key={item.label}
                href={`/${owner}/${repo}${item.path ? `/${item.path}` : ""}`}
                className={`flex flex-row w-full h-9 items-center border-b select-none cursor-default text-sm hover:bg-accent/50 font-mono ${
                  active ? "bg-sidebar" : ""
                }`}
                prefetch={true}
                data-sidebar-item
                data-sidebar-item-active={active ? "true" : undefined}
              >
                <span className="ml-2">{item.label}</span>
              </Link>
            );
          })}
        </div>
      </SidebarContent>
    </Sidebar>
  );
}
