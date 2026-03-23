"use client";

import { usePathname } from "next/navigation";
import Link from "@/ui/link";
import { OverlayScroll } from "@/ui/scroll";
import { Sidebar, SidebarContent } from "@/ui/sidebar";

const navItems = [
  { path: "", label: "/home" },
  { path: "files", label: "/files" },
  { path: "commits", label: "/commits" },
  { path: "questions", label: "/questions" },
  { path: "reviews", label: "/reviews" },
  { path: "builds", label: "/builds" },
];

export const NAV_SECTIONS = new Set(
  [...navItems.map((i) => i.path), "settings"].filter(Boolean),
);

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
      <LayoutSidebar owner={owner} repo={repo} showSettings={showSettings} />
      <OverlayScroll> {children} </OverlayScroll>
    </>
  );
}

function LayoutSidebar({
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

  const items = showSettings
    ? [...navItems, { path: "settings", label: "/settings" }]
    : navItems;
  const isActive = (itemPath: string) => {
    const full = `/${itemPath}`;
    return path === full || path.startsWith(`${full}/`);
  };

  return (
    <Sidebar>
      <SidebarContent className="overflow-auto">
        <div className="flex flex-col w-full">
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
