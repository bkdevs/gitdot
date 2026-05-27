"use client";

import type {
  RepositoryCommitFilterResource,
  RepositoryResource,
} from "gitdot-api";
import {
  type ResourcePromisesType,
  type ResourceResultType,
  useResolvePromises,
} from "gitdot-dal/client";
import { Suspense, use, useState } from "react";
import { useTimezone } from "@/(main)/context/timezone";
import { Loading } from "@/ui/loading";
import { dateInRange, formatDateIso } from "@/util/date";
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

type ResourcePromises = ResourcePromisesType<Resources>;

export function PageClient({
  owner,
  repo,
  resources,
  repository,
}: {
  owner: string;
  repo: string;
  resources: ResourceResultType<Resources>;
  repository: RepositoryResource | null;
}) {
  const resolvedPromises = useResolvePromises(owner, repo, resources);
  return (
    <Suspense fallback={<Loading />}>
      <PageContent
        owner={owner}
        repo={repo}
        promises={resolvedPromises}
        repository={repository}
      />
    </Suspense>
  );
}

function PageContent({
  owner,
  repo,
  promises,
  repository,
}: {
  owner: string;
  repo: string;
  promises: ResourcePromises;
  repository: RepositoryResource | null;
}) {
  const tz = useTimezone();
  const commits = use(promises.commits);
  const paths = use(promises.paths);
  const commitFilters = use(promises.commitFilters);

  const [windowStart, setWindowStart] = useState(() =>
    recentWindowStart(commits, tz),
  );
  const [windowEnd, setWindowEnd] = useState(() =>
    recentWindowEnd(commits, tz),
  );
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
    dateInRange(
      formatDateIso(new Date(commit.date), tz),
      filterStart,
      filterEnd,
    ),
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
        owner={owner}
        repo={repo}
        commits={commits}
        paths={paths}
        filters={filters}
        activeFilter={activeFilter}
        setActiveFilter={setActiveFilter}
      />
    </div>
  );
}
