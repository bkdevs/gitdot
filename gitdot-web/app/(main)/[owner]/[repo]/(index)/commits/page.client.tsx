"use client";

import type { RepositoryCommitFilterResource } from "gitdot-api";
import { Suspense, use, useState } from "react";
import {
  type ResourcePromisesType,
  type ResourceRequestsType,
  useResolvePromises,
} from "@/(main)/[owner]/[repo]/resources";
import { Loading } from "@/ui/loading";
import { inRange } from "@/util/date";
import type { Resources } from "./page";
import { CommitsFilterPanel } from "./ui/commits-filter-panel";
import { CommitsGrid } from "./ui/commits-grid";
import { CommitsList } from "./ui/commits-list";
import { ALL_COMMITS_FILTER, filterCommits } from "./util";

type ResourceRequests = ResourceRequestsType<Resources>;
type ResourcePromises = ResourcePromisesType<Resources>;

export function PageClient({
  owner,
  repo,
  requests,
  promises,
}: {
  owner: string;
  repo: string;
  requests: ResourceRequests;
  promises: ResourcePromises;
}) {
  const resolvedPromises = useResolvePromises(owner, repo, requests, promises);
  return (
    <Suspense fallback={<Loading />}>
      <PageContent promises={resolvedPromises} />
    </Suspense>
  );
}

function PageContent({ promises }: { promises: ResourcePromises }) {
  const commits = use(promises.commits);
  const paths = use(promises.paths);
  const commitFilters = use(promises.commitFilters);

  const [startDate, setStartDate] = useState<string | null>(null);
  const [endDate, setEndDate] = useState<string | null>(null);
  const filters = [ALL_COMMITS_FILTER, ...(commitFilters ?? [])];
  const [activeFilter, setActiveFilter] =
    useState<RepositoryCommitFilterResource>(ALL_COMMITS_FILTER);

  if (!commits) return null;

  const filteredCommits = filterCommits(activeFilter, commits);
  const commitsInRange =
    startDate && endDate
      ? filteredCommits.filter((commit) =>
          inRange(commit.date.slice(0, 10), startDate, endDate),
        )
      : filteredCommits;

  return (
    <div className="flex flex-row h-full">
      <div className="flex flex-col flex-1 min-w-0 min-h-0">
        <CommitsGrid
          commits={filteredCommits}
          startDate={startDate}
          endDate={endDate}
          setStartDate={setStartDate}
          setEndDate={setEndDate}
        />
        <CommitsList commits={commitsInRange} />
      </div>
      <CommitsFilterPanel
        commits={commits}
        paths={paths}
        filters={filters}
        activeFilter={activeFilter}
        setActiveFilter={setActiveFilter}
      />
    </div>
  );
}
