import type { UserResource } from "gitdot-api";
import Image from "next/image";

export function UserImage({ user }: { user: UserResource }) {
  return user.image ? (
    <Image
      src={`data:image/webp;base64,${user.image}`}
      alt={user.name}
      width={32}
      height={32}
      unoptimized
      className="rounded-full"
    />
  ) : (
    <div className="size-8 rounded-full bg-foreground flex items-center justify-center">
      <span className="font-mono font-light text-base text-background">
        {user.name[0].toUpperCase()}
      </span>
    </div>
  );
}
