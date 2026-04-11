"use client";

import { CommandDialog } from "@/(main)/ui/command-dialog";

export function CommandProvider({ children }: { children: React.ReactNode }) {
  return (
    <>
      {children}
      <CommandDialog />
    </>
  );
}
