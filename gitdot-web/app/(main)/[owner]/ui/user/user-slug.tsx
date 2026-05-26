"use client";

import Link from "@/ui/link";
import { cn } from "@/util";

export function UserSlug({
  user,
  className,
}: {
  user: { id?: string; name?: string; git_name?: string };
  className?: string;
}) {
  if (user.id && user.name) {
    return (
      <Link
        href={`/${user.name}`}
        className={cn(
          "truncate min-w-0 underline hover:text-foreground transition-colors",
          className,
        )}
        onClick={(e) => e.stopPropagation()}
      >
        {user.name}
      </Link>
    );
  }

  return (
    <span className={cn("truncate min-w-0", className)}>
      {user.git_name ?? "Unknown"}
    </span>
  );
}
