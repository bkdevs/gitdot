"use client";

import { ImportRepoDialog } from "@/(main)/ui/import-repo-dialog";

export function ImportRepoProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <>
      {children}
      <ImportRepoDialog />
    </>
  );
}
