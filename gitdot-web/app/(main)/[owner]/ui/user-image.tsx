"use client";

import Image from "next/image";
import { useState } from "react";

export function UserImage({
  user,
  px = 32,
}: {
  user: { id: string };
  px?: number;
}) {
  const [imgError, setImgError] = useState(false);

  if (imgError) {
    return (
      <div
        className="rounded-full bg-foreground shrink-0"
        style={{ width: px, height: px }}
      />
    );
  }

  return (
    <Image
      src={`https://images.gitdot.io/users/${user.id}.webp`}
      alt="user avatar"
      width={px}
      height={px}
      unoptimized
      className="rounded-full"
      style={{ width: px, height: px }}
      onError={() => setImgError(true)}
    />
  );
}
