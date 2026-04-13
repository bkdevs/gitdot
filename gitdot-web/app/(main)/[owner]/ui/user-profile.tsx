import type { UserResource } from "gitdot-api";
import { UserImage } from "./user-image";

export function UserProfile({ user }: { user: UserResource }) {
  return (
    <div className="flex flex-col items-end">
      <div className="mb-0.5">
        <UserImage userId={user.id} />
      </div>
      <p className="font-semibold text-sm mb-0.5">{user.name}</p>

      {user.company && (
        <p className="text-xs text-muted-foreground">{user.company}</p>
      )}
      {user.location && (
        <p className="text-xs text-muted-foreground">{user.location}</p>
      )}
    </div>
  );
}
