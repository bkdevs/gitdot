"use client";

import {
  Activity,
  Bell,
  Circle,
  Code,
  Command,
  Ellipsis,
  Gauge,
  Inbox,
  Mail,
  MessageCircleQuestion,
  MessageSquare,
  MoreHorizontal,
  MoreVertical,
  Plus,
  Rss,
  Search,
  Settings,
  Square,
  User,
} from "lucide-react";
import Image from "next/image";
import { usePathname } from "next/navigation";
import { useUser } from "@/(main)/providers/user-provider";
import Link from "@/ui/link";
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
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
  const isDefault = !["/inbox", "/questions", "/charts", "/settings"].includes(
    pathname,
  );
  const { user } = useUser();

  return (
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
                href={user ? `/${user.name}` : "/"}
                isActive={true}
                iconClassName="!size-2 fill-current"
              />
              <NavItem
                icon={Search}
                label="Search"
                href="/search"
                isActive={pathname === "/search"}
              />
              <NavItem
                icon={Bell}
                label="Notifications"
                href="/notifications"
                isActive={pathname === "/notifications"}
              />
              <NavItem
                icon={Plus}
                label="Create"
                href="/create"
                isActive={pathname === "/create"}
              />
              <NavItem
                icon={MoreHorizontal}
                label="More"
                href="/more"
                isActive={pathname === "/more"}
              />
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>
    </Sidebar>
  );
}

function NavItem({
  icon: Icon,
  label,
  href,
  isActive,
  iconClassName,
}: {
  icon: React.ComponentType<{ className?: string }>;
  label: string;
  href?: string;
  isActive: boolean;
  iconClassName?: string;
}) {
  const button = (
    <SidebarMenuButton
      tooltip={label}
      className="group w-full h-full flex items-center justify-center p-0! rounded-none hover:bg-transparent! hover:text-current!"
    >
      <Icon
        className={cn(
          iconClassName ?? "h-4 w-4",
          "mr-1 group-hover:stroke-[2.25]",
        )}
      />
      <span className="sr-only">{label}</span>
    </SidebarMenuButton>
  );

  return (
    <SidebarMenuItem
      className={`w-10 h-9 border-b p-0! border-l-4 bg-sidebar ${isActive ? "border-l-primary" : "border-l-transparent"}`}
    >
      {href ? (
        <Link prefetch={true} href={href}>
          {button}
        </Link>
      ) : (
        button
      )}
    </SidebarMenuItem>
  );
}
