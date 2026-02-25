import type { TaskResponse } from "@/lib/dto";
import { cn } from "@/util";

const STATUS_STYLES: Record<string, string> = {
  pending: "bg-yellow-400",
  blocked: "bg-muted-foreground",
  assigned: "bg-blue-400",
  running: "bg-blue-400",
  success: "bg-green-500",
  failure: "bg-red-500",
};

export function TaskRow({ task }: { task: TaskResponse }) {
  const dotClass = STATUS_STYLES[task.status] ?? "bg-muted-foreground";

  return (
    <div className="flex flex-row w-full border-b py-2 px-3 gap-3 items-start min-h-14">
      <div className="flex items-center pt-1.5">
        <span className={cn("size-2 rounded-full shrink-0", dotClass)} />
      </div>
      <div className="flex flex-col min-w-0 flex-1">
        <div className="flex flex-row items-center gap-2">
          <span className="text-sm">{task.name}</span>
          <span className="text-xs text-muted-foreground capitalize">
            {task.status}
          </span>
        </div>
        <span className="text-xs text-muted-foreground font-mono mt-0.5">
          {task.command}
        </span>
      </div>
    </div>
  );
}
