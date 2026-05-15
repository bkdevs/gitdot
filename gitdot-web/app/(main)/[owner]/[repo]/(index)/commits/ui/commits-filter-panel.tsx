"use client";

import type {
  RepositoryCommitResource,
  RepositoryPathsResource,
} from "gitdot-api";
import type { CommitFilter } from "../util";
import { CommitsFilterDetail } from "./commits-filter-detail";
import { CommitsFilterList } from "./commits-filter-list";

export function CommitsFilterPanel({
  commits,
  paths,
  filters,
  activeFilter,
  setActiveFilter,
}: {
  commits: RepositoryCommitResource[];
  paths: RepositoryPathsResource | null;
  filters: CommitFilter[];
  activeFilter: CommitFilter;
  setActiveFilter: (filter: CommitFilter) => void;
}) {
  const active =
    filters.find((f) => f.name === activeFilter.name) ?? filters[0];

  return (
    <div className="flex flex-col w-64 shrink-0 border-l border-border">
      <CommitsFilterList
        filters={filters}
        activeFilter={activeFilter}
        setActiveFilter={setActiveFilter}
      />
      <CommitsFilterDetail
        key={active.name}
        commits={commits}
        paths={paths}
        filter={active}
      />
    </div>
  );
}
