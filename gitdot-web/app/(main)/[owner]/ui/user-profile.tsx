import type { UserResource } from "gitdot-api";
import { User } from "lucide-react";
import Image from "next/image";

export function UserProfile({ user }: { user: UserResource }) {
  console.log(user);
  return (
    <div className="flex flex-col items-end">
      {user.image ? (
        <Image
          src={`data:image/webp;base64,${user.image}`}
          alt={user.name}
          width={32}
          height={32}
          unoptimized
          className="rounded-full"
        />
      ) : (
        <User className="size-8 mb-0.5" />
      )}
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
