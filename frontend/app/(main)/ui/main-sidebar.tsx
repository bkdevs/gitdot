"use client";

import { Bell, Circle, MoreHorizontal, Plus, Search } from "lucide-react";
import { usePathname, useRouter } from "next/navigation";
import { useState } from "react";
import CreateRepoDialog from "@/(main)/[owner]/ui/create-repo-dialog";
import { useAuthBlocker } from "@/(main)/providers/auth-blocker-provider";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/ui/dropdown-menu";
import Link from "@/ui/link";
import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@/ui/sidebar";
import { cn } from "@/util";

const SIDEBAR_ICON_WIDTH = "2.5rem";

export function MainSidebar() {
  const pathname = usePathname();
  const router = useRouter();
  const isDefault = !["/search", "/notifications", "/settings"].includes(
    pathname,
  );
  const [createRepoOpen, setCreateRepoOpen] = useState(false);
  const { requireAuth } = useAuthBlocker();

  return (
    <>
      <Sidebar
        className="bg-sidebar h-full! border-r"
        style={{ width: SIDEBAR_ICON_WIDTH }}
      >
        <SidebarContent>
          <SidebarGroup className="p-0!">
            <SidebarGroupContent>
              <SidebarMenu className="gap-0">
                <NavItem
                  icon={Circle}
                  label="Home"
                  href="/home"
                  isActive={isDefault}
                  iconClassName="!size-2 fill-current"
                  requiresAuth={false}
                />
                <NavItem
                  icon={Search}
                  label="Search"
                  href="/search"
                  isActive={pathname === "/search"}
                  requiresAuth={false}
                />
                <NavItem
                  icon={Bell}
                  label="Notifications"
                  href="/notifications"
                  isActive={pathname === "/notifications"}
                  requiresAuth={true}
                />
                <DropdownNavItem icon={Plus} label="Create">
                  <DropdownMenuItem
                    onClick={() => {
                      if (requireAuth()) return null;
                      setCreateRepoOpen(true);
                    }}
                    className="rounded-none px-2 py-1.5 text-sm cursor-pointer"
                  >
                    New repo
                  </DropdownMenuItem>
                </DropdownNavItem>
                <DropdownNavItem icon={MoreHorizontal} label="More">
                  <DropdownMenuItem
                    onClick={() => {
                      if (requireAuth()) return null;
                      router.push("/settings");
                    }}
                    className="rounded-none px-2 py-1.5 text-sm cursor-pointer"
                  >
                    Settings
                  </DropdownMenuItem>
                </DropdownNavItem>
              </SidebarMenu>
            </SidebarGroupContent>
          </SidebarGroup>
        </SidebarContent>
      </Sidebar>
      <CreateRepoDialog open={createRepoOpen} setOpen={setCreateRepoOpen} />
    </>
  );
}

function NavItem({
  icon: Icon,
  label,
  href,
  isActive,
  iconClassName,
  requiresAuth,
}: {
  icon: React.ComponentType<{ className?: string }>;
  label: string;
  href: string;
  isActive: boolean;
  iconClassName?: string;
  requiresAuth: boolean;
}) {
  const { requireAuth } = useAuthBlocker();

  return (
    <SidebarMenuItem
      className={`w-10 h-9 border-b p-0! border-l-4 bg-sidebar ${isActive ? "border-l-primary" : "border-l-transparent"}`}
    >
      <Link
        prefetch={true}
        href={href}
        onClick={(e) => {
          if (requiresAuth) {
            e.preventDefault();
            requireAuth();
          }
        }}
      >
        <SidebarMenuButton className="group w-full h-full flex items-center justify-center p-0! rounded-none hover:bg-sidebar-accent! hover:text-current!">
          <Icon
            className={cn(
              iconClassName ?? "h-4 w-4",
              "mr-1 group-hover:stroke-[2.25]",
            )}
          />
          <span className="sr-only">{label}</span>
        </SidebarMenuButton>
      </Link>
    </SidebarMenuItem>
  );
}

function DropdownNavItem({
  icon: Icon,
  label,
  children,
}: {
  icon: React.ComponentType<{ className?: string }>;
  label: string;
  children: React.ReactNode;
}) {
  return (
    <SidebarMenuItem
      className={`w-10 h-9 border-b p-0! border-l-4 bg-sidebar border-l-transparent`}
    >
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <SidebarMenuButton className="group w-full h-full flex items-center justify-center p-0! rounded-none hover:bg-sidebar-accent! data-[state=open]:bg-sidebar-accent! hover:text-current! ring-0! outline-0!">
            <Icon className={"h-4 w-4 mr-1 group-hover:stroke-[2.25]"} />
            <span className="sr-only">{label}</span>
          </SidebarMenuButton>
        </DropdownMenuTrigger>
        <DropdownMenuContent side="right" className="rounded-none min-w-32 p-0">
          {children}
        </DropdownMenuContent>
      </DropdownMenu>
    </SidebarMenuItem>
  );
}
