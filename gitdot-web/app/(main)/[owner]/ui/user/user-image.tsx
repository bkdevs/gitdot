"use client";

import Image from "next/image";
import { useState } from "react";
import { AvatarBeam } from "@/ui/avatar-beam";
import { cn } from "@/util";

export function UserImage({
  userId,
  username,
  updatedAt,
  px = 32,
  className,
}: {
  userId?: string;
  username?: string;
  updatedAt?: string | null;
  px?: number;
  className?: string;
}) {
  const [errored, setErrored] = useState(false);

  if (!userId || errored) {
    return (
      <AvatarBeam
        name={userId ?? username ?? "anonymous"}
        size={px}
        className={className}
      />
    );
  }

  const version = updatedAt ? `?v=${new Date(updatedAt).getTime()}` : "";

  return (
    <Image
      // TODO: make images url configurable
      src={`https://images.gitdot.io/users/${userId}.webp${version}`}
      alt="user avatar"
      width={px}
      height={px}
      className={cn("rounded-full shrink-0", className)}
      style={{ width: px, height: px }}
      unoptimized
      onError={() => setErrored(true)}
    />
  );
}
