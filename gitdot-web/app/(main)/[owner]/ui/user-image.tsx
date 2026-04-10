"use client";

import Image from "next/image";
import { useState } from "react";

export function UserImage({
  user,
  px = 32,
}: {
  user: { name: string };
  px?: number;
}) {
  const [imgError, setImgError] = useState(false);

  if (imgError) {
    return (
      <div
        className="rounded-full bg-foreground flex items-center justify-center shrink-0"
        style={{ width: px, height: px }}
      >
        <span
          className={`font-mono font-light text-background leading-none ${px <= 20 ? "text-xs" : "text-sm"}`}
        >
          {user.name[0].toUpperCase()}
        </span>
      </div>
    );
  }

  return (
    <Image
      src={`https://images.gitdot.io/users/${user.name}.webp`}
      alt={user.name}
      width={px}
      height={px}
      unoptimized
      className="rounded-full"
      style={{ width: px, height: px }}
      onError={() => setImgError(true)}
    />
  );
}
