"use client";

import { usePathname } from "next/navigation";
import Link from "@/ui/link";
import { OverlayScroll } from "@/ui/scroll";
import { Sidebar, SidebarContent } from "@/ui/sidebar";

export const NAV_ITEMS = [
  { path: "", label: "/home" },
  { path: "files", label: "/files" },
  { path: "commits", label: "/commits" },
  { path: "settings", label: "/settings", protected: true },
  { path: "questions", label: "/questions", beta: true },
  { path: "reviews", label: "/reviews", beta: true },
  { path: "builds", label: "/builds", beta: true },
];

const IS_BETA = process.env.NEXT_PUBLIC_GITDOT_BETA === "true";

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

  const items = NAV_ITEMS.filter(
    (i) => (!i.protected || showSettings) && (!i.beta || IS_BETA),
  );
  const isActive = (itemPath: string) => {
    const full = `/${itemPath}`;
    return path === full || path.startsWith(`${full}/`);
  };

  return (
    <Sidebar>
      <SidebarContent className="overflow-auto">
        <div className="flex flex-col w-full">
          <div className="h-15 px-2 border-b flex flex-col justify-center">
            <Link
              href={`/${owner}/${repo}`}
              className="text-sm font-medium hover:underline font-mono leading-tight"
              prefetch={true}
            >
              <span className="font-normal text-muted-foreground">
                {owner}/
              </span>
              {repo}
            </Link>
            <p className="text-xs text-muted-foreground mt-0.5 leading-tight">
              no description
            </p>
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
