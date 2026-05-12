import type { OrganizationResource } from "gitdot-api";
import {
  getCurrentUser,
  listOrganizationMembers,
  listOrganizationRepositories,
} from "@/dal";
import { OrgActions } from "./org-actions";
import { OrgLinks } from "./org-links";
import { OrgMembers } from "./org-members";
import { OrgProfile } from "./org-profile";
import { OrgReadme } from "./org-readme";
import { OrgRepositories } from "./org-repositories";

export default async function OrgPage({ org }: { org: OrganizationResource }) {
  const [members, repos, current] = await Promise.all([
    listOrganizationMembers(org.name),
    listOrganizationRepositories(org.name),
    getCurrentUser(false),
  ]);
  const membership =
    current?.memberships.find((m) => m.org_name === org.name) ?? null;
  const role: "guest" | "member" | "admin" = membership
    ? membership.role === "admin"
      ? "admin"
      : "member"
    : "guest";
  const isMember = role !== "guest";

  return (
    <div className="grid grid-cols-[8rem_minmax(0,3fr)_minmax(0,2fr)] h-full">
      <div className="overflow-y-auto scrollbar-none">
        <div className="flex flex-col items-start pl-4 pr-2 my-2.5 pt-0.5 gap-6">
          <OrgProfile org={org} />
          <OrgLinks org={org} />
          <OrgActions
            org={org}
            members={members}
            role={role}
            membership={membership}
          />
        </div>
      </div>

      <div className="px-3 py-2 border-l flex flex-col gap-8 overflow-y-auto scrollbar-none">
        <OrgReadme readme={org.readme} />
        <OrgRepositories repos={repos} isMember={isMember} />
      </div>

      <div className="pt-2 border-l flex flex-col min-h-0">
        <OrgMembers members={members} />
      </div>
    </div>
  );
}
