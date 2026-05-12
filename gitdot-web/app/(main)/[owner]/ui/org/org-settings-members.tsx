"use client";

import type { OrganizationResource } from "gitdot-api";

export function OrgSettingsMembers({
  org: _org,
}: {
  org: OrganizationResource;
}) {
  return (
    <div className="max-w-lg mx-auto p-4">
      <p className="text-sm text-muted-foreground">
        <span className="text-foreground/40 select-none"># </span>
        members settings (coming soon)
      </p>
    </div>
  );
}
