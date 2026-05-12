"use client";

import type {
  OrganizationMemberResource,
  OrganizationResource,
} from "gitdot-api";
import { useState } from "react";
import { EditMemberDialog } from "./edit-member-dialog";
import { NewMemberDialog } from "./new-member-dialog";
import { OrgSettingsDialog } from "./org-settings-dialog";
import type { OrgSettingsTab } from "./org-settings-sidebar";

export function OrgActions({
  org,
  members,
  role,
  membership,
}: {
  org: OrganizationResource;
  members: OrganizationMemberResource[] | null;
  role: "guest" | "member" | "admin";
  membership: OrganizationMemberResource | null;
}) {
  const [settingsOpen, setSettingsOpen] = useState(false);
  const [settingsTab, setSettingsTab] = useState<OrgSettingsTab>("profile");
  const [newMemberOpen, setNewMemberOpen] = useState(false);
  const [editingMember, setEditingMember] =
    useState<OrganizationMemberResource | null>(null);

  if (role === "guest") return null;

  function openSettings(tab: OrgSettingsTab) {
    setSettingsTab(tab);
    setSettingsOpen(true);
  }

  const isAdmin = role === "admin";
  const actions: { label: string; onClick: () => void; adminOnly?: boolean }[] =
    [
      {
        label: "new repo",
        onClick: () =>
          window.dispatchEvent(
            new CustomEvent("openNewRepo", { detail: { owner: org.name } }),
          ),
      },
      {
        label: "new member",
        onClick: () => setNewMemberOpen(true),
        adminOnly: true,
      },
      {
        label: "edit profile",
        onClick: () => setEditingMember(membership),
      },
      {
        label: "settings",
        onClick: () => openSettings("profile"),
        adminOnly: true,
      },
    ].filter((a) => !a.adminOnly || isAdmin);

  return (
    <div className="flex flex-col items-start">
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
        members={members}
        open={settingsOpen}
        onOpenChange={setSettingsOpen}
        tab={settingsTab}
        onTabChange={setSettingsTab}
        onAddMember={() => setNewMemberOpen(true)}
        onEditMember={(m) => setEditingMember(m)}
      />
      <NewMemberDialog
        orgName={org.name}
        open={newMemberOpen}
        setOpen={setNewMemberOpen}
      />
      {editingMember && (
        <EditMemberDialog
          orgName={org.name}
          member={editingMember}
          open={!!editingMember}
          setOpen={(o) => !o && setEditingMember(null)}
        />
      )}
    </div>
  );
}
