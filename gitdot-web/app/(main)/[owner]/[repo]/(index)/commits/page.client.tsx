"use client";

import type {
  RepositoryCommitFilterResource,
  RepositoryResource,
} from "gitdot-api";
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
import {
  ALL_COMMITS_FILTER,
  filterCommits,
  recentWindowEnd,
  recentWindowStart,
} from "./util";

type ResourceRequests = ResourceRequestsType<Resources>;
type ResourcePromises = ResourcePromisesType<Resources>;

export function PageClient({
  owner,
  repo,
  requests,
  promises,
  repository,
}: {
  owner: string;
  repo: string;
  requests: ResourceRequests;
  promises: ResourcePromises;
  repository: RepositoryResource | null;
}) {
  const resolvedPromises = useResolvePromises(owner, repo, requests, promises);
  return (
    <Suspense fallback={<Loading />}>
      <PageContent promises={resolvedPromises} repository={repository} />
    </Suspense>
  );
}

function PageContent({
  promises,
  repository,
}: {
  promises: ResourcePromises;
  repository: RepositoryResource | null;
}) {
  const commits = use(promises.commits);
  const paths = use(promises.paths);
  const commitFilters = use(promises.commitFilters);

  const [windowStart, setWindowStart] = useState(() =>
    recentWindowStart(commits),
  );
  const [windowEnd, setWindowEnd] = useState(() => recentWindowEnd(commits));
  const [selectedStart, setSelectedStart] = useState<string | null>(null);
  const [selectedEnd, setSelectedEnd] = useState<string | null>(null);
  const filters = [ALL_COMMITS_FILTER, ...(commitFilters ?? [])];
  const [activeFilter, setActiveFilter] =
    useState<RepositoryCommitFilterResource>(ALL_COMMITS_FILTER);

  if (!commits) return null;

  const filteredCommits = filterCommits(activeFilter, commits);
  const filterStart = selectedStart ?? windowStart;
  const filterEnd = selectedEnd ?? windowEnd;
  const commitsInRange = filteredCommits.filter((commit) =>
    inRange(commit.date.slice(0, 10), filterStart, filterEnd),
  );

  return (
    <div className="flex flex-row h-full">
      <div className="flex flex-col flex-1 min-w-0 min-h-0">
        <CommitsGrid
          commits={filteredCommits}
          repository={repository}
          windowStart={windowStart}
          windowEnd={windowEnd}
          setWindowStart={setWindowStart}
          setWindowEnd={setWindowEnd}
          selectedStart={selectedStart}
          selectedEnd={selectedEnd}
          setSelectedStart={setSelectedStart}
          setSelectedEnd={setSelectedEnd}
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
