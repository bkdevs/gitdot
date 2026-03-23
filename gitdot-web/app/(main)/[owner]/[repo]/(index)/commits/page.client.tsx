"use client";

import type { CommitFilterResource } from "gitdot-api";
import { Suspense, use, useState } from "react";
import {
  type ResourcePromisesType,
  type ResourceRequestsType,
  resolveResources,
} from "@/provider/client";
import type { Resources } from "./page";
import { CommitsGrid } from "./ui/commits-grid";
import { CommitsHeader } from "./ui/commits-header";
import { CommitsList } from "./ui/commits-list";
import { CommitsShortcuts } from "./ui/commits-shortcuts";
import { filterCommits, inRange } from "./util";

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
  const resolvedPromises = resolveResources(owner, repo, requests, promises);
  return (
    <Suspense>
      <PageContent promises={resolvedPromises} />
    </Suspense>
  );
}

const ALL_COMMITS_FILTER: CommitFilterResource = {
  name: "All commits",
  created_at: "1970-01-01T00:00:00Z",
  updated_at: "1970-01-01T00:00:00Z",
};

function PageContent({ promises }: { promises: ResourcePromises }) {
  const commits = use(promises.commits);
  const settings = use(promises.settings);

  const [startDate, setStartDate] = useState<string | null>(null);
  const [endDate, setEndDate] = useState<string | null>(null);
  const filters = [ALL_COMMITS_FILTER, ...(settings?.commit_filters ?? [])];
  const [activeFilter, setActiveFilter] = useState(filters[0]);

  if (!commits) return null;

  const filteredCommits = filterCommits(activeFilter, commits);
  const commitsInRange =
    startDate && endDate
      ? filteredCommits.filter((commit) =>
          inRange(commit.date.slice(0, 10), startDate, endDate),
        )
      : filteredCommits;

  return (
    <div className="flex flex-col h-full">
      <CommitsShortcuts setStartDate={setStartDate} setEndDate={setEndDate} />
      <CommitsHeader
        startDate={startDate}
        endDate={endDate}
        commitCount={commitsInRange.length}
        filters={filters}
        activeFilter={activeFilter}
        setActiveFilter={setActiveFilter}
        setStartDate={setStartDate}
        setEndDate={setEndDate}
      />
      <CommitsGrid
        commits={filteredCommits}
        startDate={startDate}
        endDate={endDate}
        setStartDate={setStartDate}
        setEndDate={setEndDate}
      />
      <CommitsList commits={commitsInRange} />
    </div>
  );
}
