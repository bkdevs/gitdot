"use client";

import type {
  RepositoryCommitFilterResource,
  RepositoryCommitResource,
  RepositoryPathsResource,
} from "gitdot-api";
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
  filters: RepositoryCommitFilterResource[];
  activeFilter: RepositoryCommitFilterResource;
  setActiveFilter: (filter: RepositoryCommitFilterResource) => void;
}) {
  const active = filters.find((f) => f.id === activeFilter.id) ?? filters[0];

  return (
    <div className="flex flex-col w-64 shrink-0 border-l border-border">
      <CommitsFilterList
        filters={filters}
        activeFilter={activeFilter}
        setActiveFilter={setActiveFilter}
      />
      <CommitsFilterDetail
        key={active.id}
        commits={commits}
        paths={paths}
        filter={active}
      />
    </div>
  );
}
