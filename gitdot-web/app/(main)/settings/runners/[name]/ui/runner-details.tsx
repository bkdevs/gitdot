import type { RunnerResource } from "gitdot-api";
import { formatDateTime } from "@/util";
import { RunnerSettings } from "./runner-settings";

export function RunnerDetails({ runner }: { runner: RunnerResource }) {
  const createdAt = new Date(runner.created_at);
  const lastActive = runner.last_active ? new Date(runner.last_active) : null;
  const isActive = lastActive && Date.now() - lastActive.getTime() <= 90 * 1000;

  return (
    <div className="flex w-72 shrink-0 flex-col border-l border-border">
      <div className="p-2">
        <dl className="space-y-3 text-sm text-muted-foreground">
          <div className="space-y-0.5">
            <dt className="font-medium text-foreground">Owner</dt>
            <dd>{runner.owner_name}</dd>
          </div>
          <div className="space-y-0.5">
            <dt className="font-medium text-foreground">Created</dt>
            <dd>{formatDateTime(createdAt)}</dd>
          </div>
          <div className="space-y-0.5">
            <dt className="font-medium text-foreground">Last active</dt>
            <dd className={isActive ? "text-green-600" : undefined}>
              {isActive && lastActive
                ? "Now"
                : lastActive
                ? formatDateTime(lastActive)
                : "Never"}
            </dd>
          </div>
        </dl>
      </div>
      <div className="mt-auto flex w-full items-center justify-end border-t border-border px-2">
        <RunnerSettings runner={runner} />
      </div>
    </div>
  );
}
