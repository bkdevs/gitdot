"use client";

import type {
  OrganizationMemberResource,
  OrganizationResource,
} from "gitdot-api";
import { formatDate } from "@/util/date";
import { UserImage } from "../user/user-image";

export function OrgSettingsMembers({
  org: _org,
  members,
}: {
  org: OrganizationResource;
  members: OrganizationMemberResource[] | null;
}) {
  const sorted = members
    ? [...members].sort(
        (a, b) =>
          new Date(a.created_at).getTime() - new Date(b.created_at).getTime(),
      )
    : null;
  return (
    <div className="divide-y divide-border">
      {sorted?.map((member) => (
        <div key={member.id} className="flex items-start gap-3 px-4 py-3">
          <UserImage userId={member.user_id} px={32} className="mt-0.5" />
          <div className="flex flex-col flex-1 min-w-0">
            <span className="font-sans text-sm font-medium mb-0.5">
              {member.user_name}
            </span>
            <p
              className={
                member.role_description
                  ? "font-sans text-xs text-foreground"
                  : "font-sans text-xs text-muted-foreground"
              }
            >
              {member.role_description || "no description found"}
            </p>
            <span className="text-[10px] font-mono text-muted-foreground mt-0.5">
              Joined {formatDate(new Date(member.created_at))}
            </span>
          </div>
        </div>
      ))}
      <div className="px-4 py-3">
        <button
          type="button"
          onClick={() => {}}
          className="text-sm underline underline-offset-2 cursor-pointer transition-colors text-muted-foreground hover:text-foreground"
        >
          Add member
        </button>
      </div>
    </div>
  );
}
