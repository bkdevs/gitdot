"use client";

import { usePathname } from "next/navigation";
import Link from "@/ui/link";
import { Sidebar, SidebarContent } from "@/ui/sidebar";

const SIDEBAR_WIDTH = "15rem";

const navItems = [
  { path: "profile", label: "/profile" },
  { path: "account", label: "/account" },
  { path: "appearance", label: "/appearance" },
  { path: "runners", label: "/runners" },
  { path: "migrations", label: "/migrations" },
];

export function SettingsSidebar() {
  const pathname = usePathname();
  const path = pathname.replace("/settings", "") || "/";

  const isActive = (itemPath: string) => {
    if (itemPath === "profile") {
      return (
        path === "/" ||
        path === "" ||
        path === `/${itemPath}` ||
        path.startsWith(`/${itemPath}/`)
      );
    }
    return path === `/${itemPath}` || path.startsWith(`/${itemPath}/`);
  };

  return (
    <div className="hidden md:flex flex-col h-full border-r shrink-0">
      <Sidebar
        className="bg-background h-full!"
        style={{ width: SIDEBAR_WIDTH }}
      >
        <SidebarContent className="overflow-auto">
          <div className="flex flex-col w-full">
            {navItems.map((item) => {
              const active = isActive(item.path);
              return (
                <Link
                  key={item.label}
                  href={item.path ? `/settings/${item.path}` : "/settings"}
                  className={`flex flex-row w-full h-9 items-center border-b select-none cursor-default text-sm hover:bg-accent/50 font-mono ${
                    active ? "bg-sidebar" : ""
                  }`}
                  prefetch={true}
                >
                  <span className="ml-2">{item.label}</span>
                </Link>
              );
            })}
          </div>
        </SidebarContent>
      </Sidebar>
    </div>
  );
}
