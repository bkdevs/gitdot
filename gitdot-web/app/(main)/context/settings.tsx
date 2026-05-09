"use client";

import { SettingsDialog } from "@/(main)/ui/settings/settings-dialog";

export function SettingsProvider({ children }: { children: React.ReactNode }) {
  return (
    <>
      {children}
      <SettingsDialog />
    </>
  );
}
