"use client";

import { usePathname } from "next/navigation";
import Link from "@/ui/link";
import { Sidebar, SidebarContent } from "@/ui/sidebar";

const navItems = [
  { path: "", label: "/profile" },
  { path: "runners", label: "/runners" },
];

export function SettingsSidebar({ owner }: { owner: string }) {
  const pathname = usePathname();
  const base = `/${owner}/settings`;
  const path = pathname.replace(base, "") || "/";

  const isActive = (itemPath: string) => {
    if (itemPath === "") {
      return path === "/" || path === "";
    }
    return path === `/${itemPath}` || path.startsWith(`/${itemPath}/`);
  };

  return (
    <Sidebar>
      <SidebarContent className="overflow-auto">
        <div className="flex flex-col w-full">
          {navItems.map((item) => {
            const active = isActive(item.path);
            return (
              <Link
                key={item.label}
                href={item.path ? `${base}/${item.path}` : base}
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
  );
}
