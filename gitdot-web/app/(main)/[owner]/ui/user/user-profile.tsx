import type { UserResource } from "gitdot-api";
import { UserImage } from "./user-image";

export function UserProfile({ user }: { user: UserResource }) {
  const displayName = user.display_name?.trim();
  const location = user.location?.trim();
  return (
    <div className="flex flex-col items-end">
      <div className="mb-0.5">
        <UserImage px={36} userId={user.id} updatedAt={user.image_updated_at} />
      </div>
      <p className="font-semibold dark:font-normal text-sm mb-0.5">
        {user.name}
      </p>

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
