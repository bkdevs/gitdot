"use client";

import {
  Activity,
  ChartBar,
  ChartLine,
  ChartNoAxesCombined,
  Circle,
  GitPullRequest,
  Mail,
  MessageCircleQuestion,
  Plus,
  TrendingUp,
} from "lucide-react";
import { usePathname } from "next/navigation";
import { useUser } from "@/providers/user-provider";
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

export function AppSidebar() {
  const pathname = usePathname();
  const isDefault = !["/inbox", "/questions", "/charts"].includes(pathname);
  const user = useUser();

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
                isActive={isDefault}
                iconClassName="!size-2 fill-current"
              />
              <NavItem
                icon={Mail}
                label="Inbox"
                href="/inbox"
                isActive={pathname === "/inbox"}
              />
              <NavItem
                icon={MessageCircleQuestion}
                label="Questions"
                href="/questions"
                isActive={pathname === "/questions"}
              />
              <NavItem
                icon={Activity}
                label="Charts"
                href="/charts"
                isActive={pathname === "/charts"}
              />
              <NavItem icon={Plus} label="New" isActive={false} />
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
      className="w-full h-full flex items-center justify-center p-0! rounded-none hover:bg-transparent! hover:text-current!"
    >
      <Icon className={cn(iconClassName ?? "h-4 w-4", "mr-1")} />
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
