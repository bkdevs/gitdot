import type { RunnerResource } from "gitdot-api";
import { Plus } from "lucide-react";
import Link from "@/ui/link";
import { formatDate } from "@/util";

export function Runners({ runners }: { runners: RunnerResource[] }) {
  return (
    <div className="flex flex-col w-full">
      <RunnersHeader />
      {runners.map((runner) => (
        <Link
          className="flex flex-row items-center px-2 py-2 border-b hover:bg-accent/50 select-none"
          key={runner.id}
          href={`/settings/runners/${runner.name}`}
        >
          <div className="flex flex-col">
            <div className="flex flex-row text-sm">{runner.name}</div>
            <div className="flex flex-row text-xs text-muted-foreground pt-0.5">
              {runner.last_verified ? "Active" : "Pending verification"} •{" "}
              {formatDate(new Date(runner.created_at))}
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

function RunnersHeader() {
  return (
    <div className="flex items-center justify-between border-b pl-2 h-9">
      <h3 className="text-xs font-semibold text-muted-foreground uppercase tracking-wide">
        Runners
      </h3>
      <Link
        href="/settings/runners/new"
        className="flex flex-row h-full items-center px-2 border-border border-l bg-primary text-xs text-primary-foreground hover:bg-primary/80 outline-0! ring-0!"
        prefetch={true}
      >
        <Plus className="size-3 mr-1.5" />
        New runner
      </Link>
    </div>
  );
}
