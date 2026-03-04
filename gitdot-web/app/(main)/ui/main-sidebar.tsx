"use client";

import CreateRepoDialog from "@/(main)/[owner]/ui/create-repo-dialog";
import { useAuthBlocker } from "@/(main)/providers/auth-blocker-provider";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/ui/dropdown-menu";
import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@/ui/sidebar";
import { Files, History, Plus, Search } from "lucide-react";
import { usePathname } from "next/navigation";
import { useState } from "react";

export function MainSidebar() {
  const pathname = usePathname();
  const isDefault = !["/search", "/notifications", "/settings"].includes(
    pathname,
  );
  const [createRepoOpen, setCreateRepoOpen] = useState(false);
  const { requireAuth } = useAuthBlocker();

  return (
    <>
      <Sidebar
        className="bg-background! h-full! border-r w-9!"
      >
        <SidebarContent>
          <SidebarGroup className="p-0!">
            <SidebarGroupContent>
              <SidebarMenu className="gap-0">
                <NavItem
                icon={Search}
                label="Search"
                onClick={() => { }}
                />
                <NavItem
                icon={Files}
                label="File"
                onClick={() => window.dispatchEvent(new CustomEvent("openFileSearch")) }
                />
                <NavItem
                icon={History}
                label="History"
                onClick={() => { }}
                />
                <DropdownNavItem icon={Plus} label="Create">
                  <DropdownMenuItem
                    onClick={() => {
                      if (requireAuth()) return null;
                      setCreateRepoOpen(true);
                    }}
                  >
                    New repo
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
  iconClassName,
  onClick,
}: {
  icon: React.ComponentType<{ className?: string }>;
  label: string;
  iconClassName?: string;
  onClick: () => void;
}) {
  return (
    <SidebarMenuItem
      className={"size-9 border-b p-0!"}
    >
      <SidebarMenuButton
        onClick={onClick}
        className="w-full h-full flex items-center justify-center p-0! rounded-none hover:bg-sidebar-accent! hover:text-current!"
      >
        <Icon className={"size-4"} />
        <span className="sr-only">{label}</span>
      </SidebarMenuButton>
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
    <SidebarMenuItem className="size-9 border-b p-0!" >
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <SidebarMenuButton className="w-full h-full flex items-center justify-center p-0! rounded-none hover:bg-sidebar-accent! data-[state=open]:bg-sidebar-accent! hover:text-current! ring-0! outline-0!">
            <Icon className={"h-4 w-4"} />
            <span className="sr-only">{label}</span>
          </SidebarMenuButton>
        </DropdownMenuTrigger>
        <DropdownMenuContent side="right">{children}</DropdownMenuContent>
      </DropdownMenu>
    </SidebarMenuItem>
  );
}
