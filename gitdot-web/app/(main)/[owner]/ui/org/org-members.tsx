import type { OrganizationMemberResource } from "gitdot-api";
import { UserImage } from "../user/user-image";

const MOCK_ROLE_DESCRIPTIONS = [
  "Leads infrastructure work and keeps the build pipeline healthy across all repositories.",
  "Focuses on frontend architecture and the design system. Reviews most UI-heavy pull requests.",
  "Owns the data platform and analytics tooling. Author of the migration framework.",
  "Maintains the public API and SDK packages. Writes most of the integration docs.",
];

function mockRoleDescription(index: number): string {
  return MOCK_ROLE_DESCRIPTIONS[index % MOCK_ROLE_DESCRIPTIONS.length];
}

export function OrgMembers({
  members,
}: {
  members: OrganizationMemberResource[] | null;
}) {
  if (!members?.length) return null;

  return (
    <div>
      <p className="text-xs text-muted-foreground font-mono mb-2">
        <span className="text-foreground/40 select-none"># </span>
        Members
      </p>
      <div className="flex flex-col gap-4">
        {members.map((member, i) => (
          <div key={member.id} className="flex items-start gap-3">
            <UserImage userId={member.user_id} px={32} className="mt-0.5" />
            <div className="flex flex-col min-w-0">
              <span className="text-sm font-medium">{member.user_name}</span>
              <p className="text-sm text-muted-foreground">
                {mockRoleDescription(i)}
              </p>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
