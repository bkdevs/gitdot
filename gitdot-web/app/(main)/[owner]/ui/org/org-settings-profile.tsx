"use client";

import type { OrganizationResource } from "gitdot-api";
import { formatDate, timeAgo } from "@/util/date";
import { OrgImage } from "./org-image";

export function OrgSettingsProfile({ org }: { org: OrganizationResource }) {
  const links = org.links ?? [];
  const readme = org.readme ?? "";

  return (
    <div className="max-w-lg mx-auto p-4">
      <div className="space-y-6">
        <OrgProfilePrimary org={org} />
        <OrgProfileLinks links={links} />
        <OrgProfileReadme readme={readme} />
      </div>
      <div className="flex justify-end mt-2">
        <button
          type="button"
          disabled
          className="text-sm underline-offset-4 text-muted-foreground cursor-not-allowed"
        >
          Save profile
        </button>
      </div>
    </div>
  );
}

function OrgProfilePrimary({ org }: { org: OrganizationResource }) {
  return (
    <div className="grid grid-cols-[auto_1fr] gap-x-4 gap-y-0 items-end">
      <div className="size-8 mb-1.5">
        <OrgImage orgId={org.id} />
      </div>
      <span className="text-sm font-semibold mb-1.5">{org.name}</span>
      <span className="text-sm text-muted-foreground">joined</span>
      <span className="text-sm text-muted-foreground">
        {formatDate(new Date(org.created_at))} (
        {timeAgo(new Date(org.created_at))})
      </span>
    </div>
  );
}

function OrgProfileLinks({ links }: { links: string[] }) {
  return (
    <div className="space-y-2">
      <p className="text-xs text-muted-foreground font-mono">
        <span className="text-foreground/40 select-none"># </span>
        links
      </p>
      <div className="space-y-1">
        {links.length === 0 ? (
          <p className="text-sm text-muted-foreground/40">no links</p>
        ) : (
          links.map((link, i) => (
            <input
              key={i}
              value={link}
              readOnly
              className="text-sm bg-transparent border-b border-border outline-none w-full -mb-px"
            />
          ))
        )}
      </div>
    </div>
  );
}

function OrgProfileReadme({ readme }: { readme: string }) {
  return (
    <div className="space-y-2">
      <p className="text-xs text-muted-foreground font-mono">
        <span className="text-foreground/40 select-none"># </span>
        README.md
      </p>
      <textarea
        value={readme}
        readOnly
        placeholder="what this org is about..."
        className="text-sm bg-transparent border-l border-border pl-2 outline-none w-full min-h-24 placeholder:text-muted-foreground/40 resize-none field-sizing-content"
      />
    </div>
  );
}
