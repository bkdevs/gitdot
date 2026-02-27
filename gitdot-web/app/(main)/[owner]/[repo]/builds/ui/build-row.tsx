import type { BuildResource, RepositoryCommitResource } from "gitdot-api";
import { Check, Loader2, X } from "lucide-react";
import Link from "@/ui/link";
import { formatDuration, pluralize, timeAgo } from "@/util";

const STATUS_ICONS = {
  running: <Loader2 className="size-3.5 animate-spin text-muted-foreground" />,
  success: <Check className="size-3.5 text-green-600" />,
  failure: <X className="size-3.5 text-red-600" />,
};

export function BuildRow({
  owner,
  repo,
  build,
  commit,
}: {
  owner: string;
  repo: string;
  build: BuildResource;
  commit: RepositoryCommitResource;
}) {
  const message = commit.message.split("\n")[0];

  const elapsedMs =
    build.status === "running"
      ? Date.now() - new Date(build.created_at).getTime()
      : new Date(build.updated_at).getTime() -
        new Date(build.created_at).getTime();

  const footerLabel = (() => {
    if (build.status === "success") {
      return `${pluralize(build.total_tasks, "task")} finished in ${formatDuration(elapsedMs)}`;
    }
    if (build.status === "running") {
      return `${build.completed_tasks}/${build.total_tasks} tasks finished, ${formatDuration(elapsedMs)} elapsed`;
    }
    return `${pluralize(build.total_tasks - build.completed_tasks, "task")} failed`;
  })();

  return (
    <Link
      href={`/${owner}/${repo}/builds/${build.number}`}
      className="flex flex-row w-full border-b hover:bg-accent/50 select-none px-2 pt-2 pb-1.5 h-18"
    >
      <div className="flex flex-col">
        <div className="text-xs flex items-center gap-1">
          <span className="text-muted-foreground shrink-0">
            {build.commit_sha.slice(0, 7)}
          </span>
          <span className="text-muted-foreground shrink-0">·</span>
          <span className="text-muted-foreground shrink-0">
            {timeAgo(new Date(build.created_at))}
          </span>
        </div>
        <span className="text-sm truncate">{message}</span>
        <div className="flex items-center gap-1 mt-1.5">
          {STATUS_ICONS[build.status]}
          <span className="text-xs text-muted-foreground">{footerLabel}</span>
        </div>
      </div>
    </Link>
  );
}
