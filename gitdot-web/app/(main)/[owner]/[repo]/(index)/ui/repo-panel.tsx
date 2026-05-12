import { RepoActions } from "./repo-actions";
import { RepoActivity } from "./repo-activity";
import { RepoStats } from "./repo-stats";

export function RepoPanel() {
  return (
    <div className="flex-1 min-w-0 h-full border-l flex flex-col">
      <RepoStats />
      <RepoActions />
      <RepoActivity />
    </div>
  );
}
