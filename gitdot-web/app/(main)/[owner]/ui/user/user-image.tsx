"use client";

import Image from "next/image";
import { AvatarBeam } from "@/ui/avatar-beam";
import { cn } from "@/util";

export function UserImage({
  userId,
  px = 32,
  className,
}: {
  userId?: string;
  px?: number;
  className?: string;
}) {
  if (!userId) {
    return <AvatarBeam name="anonymous" size={px} className={className} />;
  }

  return (
    <Image
      src={`https://images.gitdot.io/users/${userId}.webp`}
      alt="user avatar"
      width={px}
      height={px}
      className={cn("rounded-full shrink-0", className)}
      style={{ width: px, height: px }}
      unoptimized
    />
  );
}
