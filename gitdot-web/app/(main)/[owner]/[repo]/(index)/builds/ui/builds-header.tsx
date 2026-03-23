import { GitBranch, GitPullRequest } from "lucide-react";
import { cn } from "@/util";
import type { BuildsFilter } from "./builds-client";
import { CreateBuildButton } from "./create-build-button";

export function BuildsHeader({
  owner,
  repo,
  filter,
  setFilter,
}: {
  owner: string;
  repo: string;
  filter: BuildsFilter;
  setFilter: (filter: BuildsFilter) => void;
}) {
  return (
    <div className="flex flex-row w-full h-9 items-center border-b">
      <FilterButton
        icon={GitBranch}
        label="Main"
        isActive={filter === "main"}
        onClick={() => setFilter("main")}
      />
      <FilterButton
        icon={GitPullRequest}
        label="Pull Request"
        isActive={filter === "pull-request"}
        onClick={() => setFilter("pull-request")}
      />
      <div className="ml-auto h-full flex flex-row">
        <CreateBuildButton owner={owner} repo={repo} />
      </div>
    </div>
  );
}

function FilterButton({
  icon: Icon,
  label,
  isActive,
  onClick,
}: {
  icon: React.ComponentType<{ className?: string }>;
  label: string;
  isActive: boolean;
  onClick: () => void;
}) {
  return (
    <button
      type="button"
      className={cn(
        "flex flex-row items-center h-full border-border border-r px-2 hover:bg-sidebar",
        isActive ? "bg-sidebar text-foreground" : "text-muted-foreground",
      )}
      onClick={onClick}
    >
      <Icon className="size-3 mr-1.5" />
      <span className="text-xs">{label}</span>
    </button>
  );
}
