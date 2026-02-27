import type { RunnerResource } from "gitdot-api";
import { timeAgoFull } from "@/util";

export function RepositorySettingsRunners({
  runners,
}: {
  runners: RunnerResource[];
}) {
  return (
    <div className="flex flex-col w-full">
      <div className="flex items-center border-b px-2 h-9">
        <h3 className="text-xs font-semibold text-muted-foreground uppercase tracking-wide">
          Runners
        </h3>
      </div>
      {runners.map((runner) => (
        <div key={runner.id} className="flex flex-col px-2 py-2 border-b">
          <div className="text-sm">
            {runner.owner_name}/{runner.name}
          </div>
          <div className="text-xs text-muted-foreground pt-0.5">
            <RunnerStatus runner={runner} />
          </div>
        </div>
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

  const isActive = Date.now() - lastActiveDate.getTime() <= 90 * 1000;
  if (isActive) {
    return <span className="text-green-600">Active</span>;
  }

  return <span>Active {timeAgoFull(lastActiveDate)}</span>;
}
