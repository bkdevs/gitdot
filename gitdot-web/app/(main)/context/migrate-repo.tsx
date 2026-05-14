"use client";

import { MigrateRepoDialog } from "@/(main)/ui/migrate-repo-dialog";

export function MigrateRepoProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <>
      {children}
      <MigrateRepoDialog />
    </>
  );
}
