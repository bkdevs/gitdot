"use client";

import type { OrganizationMemberResource } from "gitdot-api";
import { useTimezone } from "@/(main)/context/timezone";
import { formatDate } from "@/util/date";
import { UserImage } from "../user/user-image";

export function OrgSettingsMembers({
  members,
  onAddMember,
  onEditMember,
}: {
  members: OrganizationMemberResource[] | null;
  onAddMember: () => void;
  onEditMember: (member: OrganizationMemberResource) => void;
}) {
  const tz = useTimezone();
  const sorted = members
    ? [...members].sort(
        (a, b) =>
          new Date(a.created_at).getTime() - new Date(b.created_at).getTime(),
      )
    : null;
  return (
    <div className="divide-y divide-border">
      {sorted?.map((member) => (
        <div
          key={member.id}
          className="flex items-start justify-between gap-3 px-4 py-3"
        >
          <div className="flex items-start gap-3 min-w-0">
            <UserImage
              userId={member.user_id}
              updatedAt={member.image_updated_at}
              px={32}
              className="mt-0.5"
            />
            <div className="flex flex-col flex-1 min-w-0">
              <span className="font-sans text-sm font-medium dark:font-normal mb-0.5">
                {member.user_name}
              </span>
              <p
                className={
                  member.role_description
                    ? "font-sans text-xs text-foreground"
                    : "font-sans text-xs text-muted-foreground italic"
                }
              >
                {member.role_description || "no description"}
              </p>
              <span className="text-[10px] font-mono text-muted-foreground mt-0.5">
                Joined {formatDate(new Date(member.created_at), tz)}
              </span>
            </div>
          </div>
          <button
            type="button"
            onClick={() => onEditMember(member)}
            className="text-xs cursor-pointer transition-colors duration-200 text-muted-foreground hover:text-foreground self-start mt-1"
          >
            edit
          </button>
        </div>
      ))}
      <div className="px-4 py-3">
        <button
          type="button"
          onClick={onAddMember}
          className="text-sm underline underline-offset-2 cursor-pointer transition-colors text-muted-foreground hover:text-foreground"
        >
          Add member
        </button>
      </div>
    </div>
  );
}
