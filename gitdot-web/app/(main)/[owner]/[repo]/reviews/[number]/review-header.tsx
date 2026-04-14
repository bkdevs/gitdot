"use client";

import { useParams } from "next/navigation";
import { AvatarBeam } from "@/ui/avatar-beam";
import { useUserContext } from "@/(main)/context/user";
import Link from "@/ui/link";

export function ReviewHeader() {
  const { user } = useUserContext();
  const { owner, repo, number } = useParams<{ owner: string; repo: string; number: string }>();

  return (
    <div className="shrink-0 h-16 border-b border-border flex items-stretch">
      <div className="flex-1 flex flex-col justify-center gap-1 px-7">
        <h1 className="text-sm font-semibold leading-snug truncate">
          Diff Actions Refactor
        </h1>
        <div className="flex items-center gap-2">
          <div className="flex items-center gap-1">
            <AvatarBeam name={user?.name ?? "baepaul"} size={14} />
            <span className="text-xs text-foreground/70">{user?.name ?? "baepaul"}</span>
          </div>
          <span className="text-xs text-muted-foreground">·</span>
          <div className="flex items-center text-xs font-mono text-muted-foreground">
            <Link href={`/${owner}`} className="hover:text-foreground hover:underline transition-all duration-200 cursor-pointer">{owner}</Link>
            <span>/</span>
            <Link href={`/${owner}/${repo}`} className="hover:text-foreground hover:underline transition-all duration-200 cursor-pointer">{repo}</Link>
            <span>/</span>
            <Link href={`/${owner}/${repo}/reviews`} className="hover:text-foreground hover:underline transition-all duration-200 cursor-pointer">reviews</Link>
            <span>/</span>
            <span>{number}</span>
          </div>
        </div>
      </div>
      <div className="border-l border-border flex flex-col">
        <button
          type="button"
          className="flex-1 w-full px-3 text-xs font-mono bg-primary text-primary-foreground hover:bg-primary/90 border-b border-border cursor-pointer transition-colors"
        >
          Approve
        </button>
        <button
          type="button"
          className="flex-1 w-full px-3 text-xs font-mono text-muted-foreground hover:text-foreground hover:bg-muted cursor-pointer transition-colors"
        >
          Reject
        </button>
      </div>
    </div>
  );
}
