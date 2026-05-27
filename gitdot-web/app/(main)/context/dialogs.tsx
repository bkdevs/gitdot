"use client";

import { MigrateRepoDialog } from "@/(main)/ui/migrate-repo-dialog";
import { NewOrgDialog } from "@/(main)/ui/new-org-dialog";
import { NewRepoDialog } from "@/(main)/ui/new-repo-dialog";
import { RepoSwitcherDialog } from "@/(main)/ui/repo-switcher-dialog";
import { SettingsDialog } from "@/(main)/ui/settings/settings-dialog";

export function DialogsProvider({ children }: { children: React.ReactNode }) {
  return (
    <>
      {children}
      <RepoSwitcherDialog />
      <NewOrgDialog />
      <NewRepoDialog />
      <MigrateRepoDialog />
      <SettingsDialog />
    </>
  );
}
