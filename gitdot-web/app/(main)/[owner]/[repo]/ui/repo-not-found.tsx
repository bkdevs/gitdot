"use client";

import { useUserContext } from "@/(main)/context/user";
import Link from "@/ui/link";

export function RepoNotFound({ owner, repo }: { owner: string; repo: string }) {
  const { user } = useUserContext();
  const home = user ? `/${user.name}` : "/";
  return (
    <div className="flex flex-col items-center justify-center h-full w-full gap-1 p-4">
      <p className="text-sm font-mono text-foreground">
        {owner}/{repo} not found
      </p>
      <Link
        href={home}
        className="text-xs text-muted-foreground hover:text-foreground transition-colors duration-200 underline lowercase"
      >
        return home
      </Link>
    </div>
  );
}
