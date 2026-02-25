import type { BuildResponse } from "@/lib/dto";
import Link from "@/ui/link";
import { timeAgo } from "@/util";

export function BuildRow({
  owner,
  repo,
  build,
}: {
  owner: string;
  repo: string;
  build: BuildResponse;
}) {
  const shortSha = build.commit_sha.slice(0, 7);

  return (
    <Link
      href={`/${owner}/${repo}/builds/${build.id}`}
      className="flex flex-row w-full border-b hover:bg-accent/50 select-none py-2 px-3 gap-3 items-start min-h-14"
    >
      <div className="flex flex-col min-w-0 flex-1">
        <div className="flex flex-row items-center gap-2">
          <span className="text-xs text-muted-foreground font-mono">
            {shortSha}
          </span>
          <span className="text-xs text-muted-foreground">•</span>
          <span className="text-xs text-muted-foreground">
            {timeAgo(new Date(build.created_at))}
          </span>
        </div>
        <div className="flex flex-row items-center gap-2 mt-0.5">
          <span className="text-sm">{build.trigger}</span>
        </div>
      </div>
    </Link>
  );
}
