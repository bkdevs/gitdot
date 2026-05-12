"use client";

import type { OrganizationResource } from "gitdot-api";
import { useState } from "react";
import { OrgSettingsDialog } from "./org-settings-dialog";
import type { OrgSettingsTab } from "./org-settings-sidebar";

export function OrgActions({ org }: { org: OrganizationResource }) {
  const [settingsOpen, setSettingsOpen] = useState(false);
  const [settingsTab, setSettingsTab] = useState<OrgSettingsTab>("profile");

  function openSettings(tab: OrgSettingsTab) {
    setSettingsTab(tab);
    setSettingsOpen(true);
  }

  const actions: { label: string; onClick: () => void }[] = [
    {
      label: "new repo",
      onClick: () =>
        window.dispatchEvent(
          new CustomEvent("openNewRepo", { detail: { owner: org.name } }),
        ),
    },
    { label: "members", onClick: () => openSettings("members") },
    { label: "settings", onClick: () => openSettings("profile") },
  ];

  return (
    <div className="flex flex-col items-end">
      <p className="font-semibold text-sm mb-0.5">actions</p>
      {actions.map((action) => (
        <button
          key={action.label}
          type="button"
          onClick={action.onClick}
          className="text-xs underline decoration-transparent hover:decoration-current transition-colors duration-200 cursor-pointer"
        >
          {action.label}
        </button>
      ))}
      <OrgSettingsDialog
        org={org}
        open={settingsOpen}
        onOpenChange={setSettingsOpen}
        tab={settingsTab}
        onTabChange={setSettingsTab}
      />
    </div>
  );
}
