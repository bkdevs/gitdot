"use client";

import type { OrganizationResource } from "gitdot-api";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";
import { OrgSettingsMembers } from "./org-settings-members";
import { OrgSettingsProfile } from "./org-settings-profile";
import {
  OrgSettingsSidebar,
  type OrgSettingsTab,
} from "./org-settings-sidebar";

export function OrgSettingsDialog({
  org,
  open,
  onOpenChange,
  tab,
  onTabChange,
}: {
  org: OrganizationResource;
  open: boolean;
  onOpenChange: (open: boolean) => void;
  tab: OrgSettingsTab;
  onTabChange: (tab: OrgSettingsTab) => void;
}) {
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent
        className="max-w-[60vw]! h-[80vh]! p-0! gap-0! overflow-hidden flex flex-col"
        animations={true}
        showOverlay={true}
        aria-describedby={undefined}
      >
        <DialogTitle className="sr-only">Org settings</DialogTitle>

        <div className="flex flex-1 min-h-0 overflow-hidden font-mono text-sm">
          <OrgSettingsSidebar tab={tab} onTabChange={onTabChange} />

          <div className="flex-1 overflow-y-auto scrollbar-thin">
            {tab === "profile" && <OrgSettingsProfile org={org} />}
            {tab === "members" && <OrgSettingsMembers org={org} />}
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}
