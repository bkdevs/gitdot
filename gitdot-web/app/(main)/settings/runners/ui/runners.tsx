import type { RunnerResource } from "gitdot-api";
import { Plus } from "lucide-react";
import Link from "@/ui/link";
import { timeAgoFull } from "@/util";

export function Runners({
  runners,
  basePath,
}: {
  runners: RunnerResource[];
  basePath: string;
}) {
  return (
    <div className="flex flex-col w-full">
      <RunnersHeader basePath={basePath} />
      {runners.map((runner) => (
        <Link
          className="flex flex-row items-center px-2 py-2 border-b hover:bg-accent/50 select-none"
          key={runner.id}
          href={`${basePath}/${runner.name}`}
        >
          <div className="flex flex-col">
            <div className="flex flex-row text-sm">{runner.name}</div>
            <div className="flex flex-row text-xs text-muted-foreground pt-0.5">
              <RunnerStatus runner={runner} />
            </div>
          </div>
        </Link>
      ))}
      {runners.length === 0 && (
        <p className="px-2 py-3 text-sm text-muted-foreground">
          No runners registered.
        </p>
      )}
    </div>
  );
}

function RunnerStatus({ runner }: { runner: RunnerResource }) {
  if (!runner.last_active) {
    return <span className="text-amber-600">Pending installation</span>;
  }
  const lastActiveDate = new Date(runner.last_active);

  // heuristic to account for the touch in /task/poll
  const isActive = Date.now() - lastActiveDate.getTime() <= 90 * 1000;
  if (isActive) {
    return <span className="text-green-600">Active</span>;
  }

  return <span>Active {timeAgoFull(lastActiveDate)}</span>;
}

function RunnersHeader({ basePath }: { basePath: string }) {
  return (
    <div className="flex items-center justify-between border-b pl-2 h-9">
      <h3 className="text-xs font-semibold text-muted-foreground uppercase tracking-wide">
        Runners
      </h3>
      <Link
        href={`${basePath}/new`}
        className="flex flex-row h-full items-center px-2 border-border border-l bg-primary text-xs text-primary-foreground hover:bg-primary/80 outline-0! ring-0!"
        prefetch={true}
      >
        <Plus className="size-3 mr-1.5" />
        New runner
      </Link>
    </div>
  );
}
