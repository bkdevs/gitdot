"use client";

import { useEffect, useState } from "react";
import { useShortcuts } from "@/(main)/context/shortcuts";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";
import { SettingsProfile } from "./settings-profile";
import { SettingsShortcuts } from "./settings-shortcuts";
import { SettingsTheme } from "./settings-theme";
import { type SettingsTab, SettingsSidebar } from "./settings-sidebar";

export function SettingsDialog() {
  const [open, setOpen] = useState(false);
  const [tab, setTab] = useState<SettingsTab>("profile");

  useShortcuts([
    {
      name: "Settings",
      description: "Open settings",
      keys: [","],
      execute: () => setOpen(true),
    },
  ]);

  useEffect(() => {
    const handle = () => setOpen(true);
    window.addEventListener("openSettings", handle);
    return () => window.removeEventListener("openSettings", handle);
  }, []);

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
            {tab === "profile" && <SettingsProfile />}
            {tab === "theme" && <SettingsTheme />}
            {tab === "shortcuts" && <SettingsShortcuts />}
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}
