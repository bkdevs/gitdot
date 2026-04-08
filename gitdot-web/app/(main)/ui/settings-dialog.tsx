"use client";

import { useEffect, useState } from "react";
import { useShortcuts } from "@/(main)/context/shortcuts";
import { useUserContext } from "@/(main)/context/user";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";
import { SettingsProfile } from "./settings-profile";
import { SettingsShortcuts } from "./settings-shortcuts";
import { SettingsSidebar, type SettingsTab } from "./settings-sidebar";
import { SettingsTheme } from "./settings-theme";

export function SettingsDialog() {
  const { user } = useUserContext();
  const [open, setOpen] = useState(false);
  const [tab, setTab] = useState<SettingsTab>("profile");

  useShortcuts([
    {
      name: "Settings",
      description: "Open settings",
      keys: [","],
      execute: () => {
        if (user) setOpen(true);
      },
    },
  ]);

  useEffect(() => {
    const handle = () => {
      if (user) setOpen(true);
    };
    window.addEventListener("openSettings", handle);
    return () => window.removeEventListener("openSettings", handle);
  }, [user]);

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogContent
        className="max-w-[80vw]! h-[85vh]! p-0! gap-0! overflow-hidden flex flex-col"
        animations={true}
        showOverlay={true}
        aria-describedby={undefined}
      >
        <DialogTitle className="sr-only">Settings</DialogTitle>

        <div className="flex flex-1 min-h-0 overflow-hidden font-mono text-sm">
          <SettingsSidebar tab={tab} onTabChange={setTab} />

          <div className="flex-1 overflow-y-auto scrollbar-thin p-4">
            {tab === "profile" && <SettingsProfile user={user} open={open} />}
            {tab === "theme" && <SettingsTheme />}
            {tab === "shortcuts" && <SettingsShortcuts />}
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}
