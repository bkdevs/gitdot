"use client";

import { NewOrgDialog } from "@/(main)/ui/new-org-dialog";

export function NewOrgProvider({ children }: { children: React.ReactNode }) {
  return (
    <>
      {children}
      <NewOrgDialog />
    </>
  );
}
