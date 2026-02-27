import { timeAgoFull } from "@/util";
import { type RunnerResource } from "gitdot-api";

export function RunnerStatus({ runner }: { runner: RunnerResource }) {
  if (!runner.last_active) {
    return <span className="text-amber-600">Pending installation</span>;
  }
  const lastActiveDate = new Date(runner.last_active);

  // heuristic to account for the touch in /task/poll
  const isActive =
    Date.now() - lastActiveDate.getTime() <= 90 * 1000;
  if (isActive) {
    return <span className="text-green-600">Active</span>;
  }

  return <span>Active {timeAgoFull(lastActiveDate)}</span>;
}
