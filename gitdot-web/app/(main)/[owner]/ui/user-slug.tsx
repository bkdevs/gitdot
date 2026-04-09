"use client";

import Link from "@/ui/link";
import { cn } from "@/util";

export function UserSlug({
  user,
  className,
}: {
  user: { name: string };
  className?: string;
}) {
  return (
    <Link
      href={`/${user.name}`}
      className={cn(
        "truncate min-w-0 underline hover:text-foreground transition-colors",
        className,
      )}
    >
      {user.name}
    </Link>
  );
}
