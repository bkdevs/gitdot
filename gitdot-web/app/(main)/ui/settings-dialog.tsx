"use client";

import { usePathname, useRouter } from "next/navigation";
import { useEffect, useState } from "react";
import { useShortcuts } from "@/(main)/context/shortcuts";
import { useUserContext } from "@/(main)/context/user";
import { updateUserAction } from "@/actions";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";
import { SettingsProfile } from "./settings-profile";
import { SettingsShortcuts } from "./settings-shortcuts";
import { SettingsSidebar, type SettingsTab } from "./settings-sidebar";
import { SettingsTheme } from "./settings-theme";

export function SettingsDialog() {
  const { user, refreshUser } = useUserContext();
  const router = useRouter();
  const pathname = usePathname();
  const [open, setOpen] = useState(false);
  const [tab, setTab] = useState<SettingsTab>("profile");
  const [location, setLocation] = useState(user?.location ?? "");
  const [website, setWebsite] = useState(user?.website ?? "");
  const [readme, setReadme] = useState(user?.readme ?? "");

  useEffect(() => {
    setLocation(user?.location ?? "");
    setWebsite(user?.website ?? "");
    setReadme(user?.readme ?? "");
  }, [user]);

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

  async function handleOpenChange(next: boolean) {
    if (!next && user) {
      const formData = new FormData();
      formData.set("location", location);
      formData.set("website", website);
      formData.set("readme", readme);
      await updateUserAction(null, formData);
      refreshUser();
      if (pathname === `/${user.name}`) router.refresh();
    }
    setOpen(next);
  }

  return (
    <Dialog open={open} onOpenChange={handleOpenChange}>
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
            {tab === "profile" && (
              <SettingsProfile
                user={user}
                location={location}
                onLocationChange={setLocation}
                website={website}
                onWebsiteChange={setWebsite}
                readme={readme}
                onReadmeChange={setReadme}
              />
            )}
            {tab === "theme" && <SettingsTheme />}
            {tab === "shortcuts" && <SettingsShortcuts />}
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}
