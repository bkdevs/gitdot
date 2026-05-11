import type { OrganizationResource } from "gitdot-api";
import { listOrganizationMembers, listOrganizationRepositories } from "@/dal";
import { getUserMetadata } from "@/lib/auth";
import { OrgActions } from "./org-actions";
import { OrgLinks } from "./org-links";
import { OrgMembers } from "./org-members";
import { OrgProfile } from "./org-profile";
import { OrgReadme } from "./org-readme";
import { OrgRepositories } from "./org-repositories";

export default async function OrgPage({ org }: { org: OrganizationResource }) {
  const [members, repos, metadata] = await Promise.all([
    listOrganizationMembers(org.name),
    listOrganizationRepositories(org.name),
    getUserMetadata(),
  ]);
  const isMember = metadata.orgs.some((entry) => entry.split(":")[0] === org.name);

  return (
    <div className="grid grid-cols-[8rem_minmax(0,3fr)_minmax(0,2fr)] h-full">
      <div className="overflow-y-auto scrollbar-none">
        <div className="flex flex-col items-end px-4 my-2.5 pt-0.5 gap-6 border-r">
          <OrgProfile org={org} />
          <OrgLinks org={org} />
          <OrgActions org={org} />
        </div>
      </div>

      <div className="pl-4 pr-3 py-2 flex flex-col gap-8 overflow-y-auto scrollbar-none">
        <OrgReadme readme={org.readme} />
        <OrgRepositories repos={repos} isMember={isMember} />
      </div>

      <div className="pt-2 border-l flex flex-col min-h-0">
        <OrgMembers members={members} />
      </div>
    </div>
  );
}
