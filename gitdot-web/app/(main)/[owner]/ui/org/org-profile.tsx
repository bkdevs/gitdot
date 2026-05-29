import type { OrganizationResource } from "gitdot-api";
import { OrgImage } from "./org-image";

export function OrgProfile({ org }: { org: OrganizationResource }) {
  const displayName = org.display_name?.trim();
  const location = org.location?.trim();
  return (
    <div className="flex flex-col items-end">
      <div className="mb-0.5">
        <OrgImage px={36} orgId={org.id} updatedAt={org.image_updated_at} />
      </div>
      <p className="font-semibold text-sm mb-0.5">{org.name}</p>
      {displayName && (
        <p className="text-xs text-muted-foreground">{displayName}</p>
      )}
      {location && <p className="text-xs text-muted-foreground">{location}</p>}
      {!displayName && !location && (
        <p className="text-xs text-muted-foreground italic">no description</p>
      )}
    </div>
  );
}
