"use client";

import { Plus } from "lucide-react";
import { useState } from "react";
import { SidebarMenuButton, SidebarMenuItem } from "@/ui/sidebar";
import CreateRepoDialog from "./create-repo-dialog";

export function CreateRepoButton() {
  const [open, setOpen] = useState(false);

  return (
    <SidebarMenuItem className="w-10 h-9 border-b p-0! border-l-4 bg-sidebar border-l-transparent">
      <SidebarMenuButton
        tooltip={"New"}
        className="w-full h-full flex items-center justify-center p-0! rounded-none hover:bg-transparent! hover:text-current!"
        onClick={() => setOpen(true)}
      >
        <Plus className="h-4 w-4 mr-1" />
        <span className="sr-only">New</span>
      </SidebarMenuButton>
      <CreateRepoDialog open={open} setOpen={setOpen} />
    </SidebarMenuItem>
  );
}
