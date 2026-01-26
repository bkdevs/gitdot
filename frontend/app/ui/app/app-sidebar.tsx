"use client";

import {
  GitPullRequest,
  Mail,
  MessageCircleQuestion,
  Plus,
} from "lucide-react";
import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@/ui/sidebar";

const SIDEBAR_ICON_WIDTH = "2.25rem";

const navItems = [
  { id: "inbox", icon: Mail, label: "Inbox" },
  { id: "questions", icon: MessageCircleQuestion, label: "Questions" },
  { id: "pulls", icon: GitPullRequest, label: "Pull Requests" },
  { id: "new", icon: Plus, label: "New" },
];

export function AppSidebar() {
  return (
    <Sidebar
      className="bg-sidebar h-full! border-r"
      style={{ width: SIDEBAR_ICON_WIDTH }}
    >
      <SidebarContent>
        <SidebarGroup className="p-0!">
          <SidebarGroupContent>
            <SidebarMenu className="gap-0">
              {navItems.map((item) => (
                <SidebarMenuItem
                  className="w-9 h-9 border-b p-0!"
                  key={item.id}
                >
                  <SidebarMenuButton
                    tooltip={item.label}
                    className="w-full h-full flex items-center justify-center p-0! rounded-none cursor-default"
                  >
                    <item.icon className="h-4 w-4" />
                    <span className="sr-only">{item.label}</span>
                  </SidebarMenuButton>
                </SidebarMenuItem>
              ))}
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>
    </Sidebar>
  );
}
