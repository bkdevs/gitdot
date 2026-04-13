"use client";

import { Ghost } from "lucide-react";
import Image from "next/image";

export function UserImage({
  userId,
  px = 32,
}: {
  userId?: string;
  px?: number;
}) {
  if (!userId) {
    return (
      <div
        className="rounded-full bg-black shrink-0 flex items-center justify-center"
        style={{ width: px, height: px }}
      >
        <Ghost
          style={{ width: px * 0.6, height: px * 0.6 }}
          className="text-white"
        />
      </div>
    );
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
