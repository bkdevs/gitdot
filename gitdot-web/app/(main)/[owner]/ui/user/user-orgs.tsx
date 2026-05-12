import type { OrganizationMemberResource } from "gitdot-api";
import Link from "@/ui/link";
import { formatDate } from "@/util/date";
import { OrgImage } from "../org/org-image";

export function UserOrgs({
  memberships,
}: {
  memberships: OrganizationMemberResource[] | null;
}) {
  if (!memberships?.length) return null;

  return (
    <div>
      <p className="text-xs text-muted-foreground font-mono mb-2">
        <span className="text-foreground/40 select-none"># </span>
        Organizations
      </p>
      <div className="flex flex-col gap-4">
        {memberships.map((m) => (
          <div key={m.id} className="grid grid-cols-[auto_1fr_auto] gap-x-3">
            <Link
              href={`/${m.org_name}`}
              className="peer row-span-2 self-start"
            >
              <OrgImage orgId={m.organization_id} px={32} />
            </Link>
            <Link
              href={`/${m.org_name}`}
              className="text-sm font-medium underline decoration-transparent hover:decoration-current peer-hover:decoration-current transition-colors duration-200 self-start"
            >
              {m.org_name}
            </Link>
            <span className="text-xs font-mono text-muted-foreground self-start">
              member since {formatDate(new Date(m.created_at))}
            </span>
            <p
              className={`col-start-2 col-end-4 ${
                m.role_description
                  ? "text-xs text-foreground"
                  : "text-xs text-muted-foreground"
              }`}
            >
              {m.role_description || "no description found"}
            </p>
          </div>
        ))}
      </div>
    </div>
  );
}
