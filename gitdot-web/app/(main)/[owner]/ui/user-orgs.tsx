import type { OrganizationResponse } from "@/lib/dto";
import Link from "@/ui/link";
import { formatDate } from "@/util";

export function UserOrgs({ orgs }: { orgs: OrganizationResponse[] }) {
  return (
    <div className="flex flex-col w-full">
      <OrgHeader />
      {orgs.map((org) => (
        <Link
          className="flex flex-row items-center px-2 py-2 border-b hover:bg-accent/50 select-none"
          key={org.id}
          href={`/${org.name}`}
        >
          <div className="flex flex-col">
            <div className="flex flex-row text-sm">{org.name}</div>
            <div className="flex flex-row text-xs text-muted-foreground pt-0.5">
              {formatDate(new Date(org.created_at))}
            </div>
          </div>
        </Link>
      ))}
    </div>
  );
}

function OrgHeader() {
  return (
    <div className="flex items-center justify-between border-b pl-2 h-9">
      <h3 className="text-xs font-semibold text-muted-foreground uppercase tracking-wide">
        Organizations
      </h3>
    </div>
  );
}
