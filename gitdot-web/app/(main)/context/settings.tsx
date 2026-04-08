"use client";

import { SettingsDialog } from "@/(main)/ui/settings-dialog";

export function SettingsProvider({ children }: { children: React.ReactNode }) {
  return (
    <>
      {children}
      <SettingsDialog />
    </>
  );
}
