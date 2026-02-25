import type { BuildResponse } from "@/lib/dto";
import { cn, timeAgo } from "@/util";

const STATUS_STYLES: Record<string, string> = {
  pending: "bg-yellow-400",
  assigned: "bg-blue-400",
  running: "bg-blue-400",
  success: "bg-green-500",
  failure: "bg-red-500",
};

function aggregateStatus(tasks: BuildResponse["tasks"]): string {
  if (tasks.length === 0) return "pending";
  if (tasks.some((t) => t.status === "failure")) return "failure";
  if (tasks.some((t) => t.status === "running")) return "running";
  if (tasks.some((t) => t.status === "assigned")) return "assigned";
  if (tasks.every((t) => t.status === "success")) return "success";
  return "pending";
}

export function BuildRow({
  build,
}: {
  owner: string;
  repo: string;
  build: BuildResponse;
}) {
  const status = aggregateStatus(build.tasks);
  const dotClass = STATUS_STYLES[status] ?? "bg-muted-foreground";
  const shortSha = build.commit_sha.slice(0, 7);

  return (
    <div className="flex flex-row w-full border-b hover:bg-accent/50 select-none cursor-default py-2 px-3 gap-3 items-start min-h-14">
      <div className="flex items-center pt-1.5">
        <span className={cn("size-2 rounded-full shrink-0", dotClass)} />
      </div>
      <div className="flex flex-col min-w-0 flex-1">
        <div className="flex flex-row items-center gap-2">
          <span className="text-xs text-muted-foreground capitalize">
            {status}
          </span>
          <span className="text-xs text-muted-foreground">•</span>
          <span className="text-xs text-muted-foreground font-mono">
            {shortSha}
          </span>
          <span className="text-xs text-muted-foreground">•</span>
          <span className="text-xs text-muted-foreground">
            {timeAgo(new Date(build.created_at))}
          </span>
        </div>
        <div className="flex flex-row items-center gap-2 mt-0.5">
          <span className="text-sm capitalize">{build.trigger}</span>
          <span className="text-xs text-muted-foreground">
            {build.tasks.length} {build.tasks.length === 1 ? "task" : "tasks"}
          </span>
        </div>
      </div>
    </div>
  );
}
