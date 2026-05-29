"use client";

import { Building2 } from "lucide-react";
import Image from "next/image";

export function OrgImage({
  orgId,
  updatedAt,
  px = 32,
}: {
  orgId?: string;
  updatedAt?: string | null;
  px?: number;
}) {
  if (!orgId) {
    return (
      <Building2
        className="shrink-0 text-muted-foreground"
        style={{ width: px, height: px }}
      />
    );
  }

  const version = updatedAt ? `?v=${new Date(updatedAt).getTime()}` : "";

  return (
    <Image
      // TODO: make images url configurable
      src={`https://images.gitdot.io/orgs/${orgId}.webp${version}`}
      alt="organization avatar"
      width={px}
      height={px}
      className="rounded-full shrink-0"
      style={{ width: px, height: px }}
      unoptimized
    />
  );
}
