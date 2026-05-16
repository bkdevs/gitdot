"use client";

import type {
  RepositoryCommitFilterResource,
  RepositoryCommitResource,
  RepositoryPathsResource,
} from "gitdot-api";
import { ALL_COMMITS_FILTER, isFilterModified } from "../util";
import { CommitsFilterDetail } from "./commits-filter-detail";
import { CommitsFilterList } from "./commits-filter-list";

export function CommitsFilterPanel({
  owner,
  repo,
  commits,
  paths,
  filters,
  activeFilter,
  setActiveFilter,
}: {
  owner: string;
  repo: string;
  commits: RepositoryCommitResource[];
  paths: RepositoryPathsResource | null;
  filters: RepositoryCommitFilterResource[];
  activeFilter: RepositoryCommitFilterResource;
  setActiveFilter: (filter: RepositoryCommitFilterResource) => void;
}) {
  const original =
    filters.find((f) => f.id === activeFilter.id) ?? ALL_COMMITS_FILTER;
  const isModified = isFilterModified(activeFilter, original);

  return (
    <div className="flex flex-col w-64 shrink-0 border-l border-border">
      <CommitsFilterList
        filters={filters}
        activeFilter={activeFilter}
        setActiveFilter={setActiveFilter}
        isModified={isModified}
      />
      <CommitsFilterDetail
        owner={owner}
        repo={repo}
        commits={commits}
        paths={paths}
        filter={activeFilter}
        setActiveFilter={setActiveFilter}
        isModified={isModified}
      />
    </div>
  );
}
