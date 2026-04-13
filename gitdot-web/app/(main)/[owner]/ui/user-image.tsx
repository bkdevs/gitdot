"use client";

import { AvatarBeam } from "@/ui/avatar-beam";
import Image from "next/image";

export function UserImage({
  userId,
  px = 32,
}: {
  userId?: string;
  px?: number;
}) {
  if (!userId) {
    return <AvatarBeam name="anonymous" size={px} />;
  }

  return (
    <Image
      src={`https://images.gitdot.io/users/${userId}.webp`}
      alt="user avatar"
      width={px}
      height={px}
      className="rounded-full"
      style={{ width: px, height: px }}
    />
  );
}
