"use client";

import type { UserResource } from "gitdot-api";
import { useEffect, useState } from "react";
import { useShortcuts } from "@/(main)/provider/shortcuts";
import { useUserContext } from "@/(main)/provider/user";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";
import { SettingsAccount } from "./settings-account";
import { SettingsAppearance } from "./settings-appearance";
import { SettingsEmails } from "./settings-emails";
import { SettingsInstallations } from "./settings-installations";
import { SettingsMigrations } from "./settings-migrations";
import { SettingsProfile } from "./settings-profile";
import { SettingsSidebar, type SettingsTab } from "./settings-sidebar";

export function SettingsDialog() {
  const { user } = useUserContext();
  if (!user) return null;

  return <SettingsDialogInner user={user} />;
}

function SettingsDialogInner({ user }: { user: UserResource }) {
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
        className="max-w-[60vw]! h-[80vh]! p-0! gap-0! overflow-hidden flex flex-col"
        animations={true}
        showOverlay={true}
        aria-describedby={undefined}
      >
        <DialogTitle className="sr-only">Settings</DialogTitle>

        <div className="flex flex-1 min-h-0 overflow-hidden font-mono text-sm">
          <SettingsSidebar tab={tab} onTabChange={setTab} />

          <div className="flex-1 overflow-y-auto scrollbar-thin">
            {tab === "profile" && <SettingsProfile user={user} />}
            {tab === "emails" && <SettingsEmails />}
            {tab === "account" && <SettingsAccount setSettingsOpen={setOpen} />}
            {tab === "appearance" && <SettingsAppearance />}
            {tab === "installations" && <SettingsInstallations />}
            {tab === "migrations" && <SettingsMigrations />}
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}
