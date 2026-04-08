import type { UserResource } from "gitdot-api";
import Image from "next/image";

export function UserProfile({ user }: { user: UserResource }) {
  return (
    <div className="flex flex-col items-end">
      <Image
      src="/paul-penguin.jpeg"
      alt={user.name}
      width={32}
      height={32}
      className="rounded-full"
      />
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
