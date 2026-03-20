"use client";

import type { CommitFilterResource } from "gitdot-api";
import { Suspense, use, useState } from "react";
import { useRepoContext } from "../context";
import { CommitsShortcuts } from "./shortcuts";
import { CommitsGrid } from "./ui/commits-grid";
import { CommitsHeader } from "./ui/commits-header";
import { CommitsList } from "./ui/commits-list";
import { filterCommits } from "./util";

const ALL_COMMITS_FILTER: CommitFilterResource = {
  name: "All commits",
  created_at: "1970-01-01T00:00:00Z",
  updated_at: "1970-01-01T00:00:00Z",
};

export function CommitsClient() {
  const commits = use(useRepoContext().commits);
  const settings = use(useRepoContext().settings);
  const [startDate, setStartDate] = useState<string | null>(null);
  const [endDate, setEndDate] = useState<string | null>(null);
  const filters = [ALL_COMMITS_FILTER, ...(settings?.commit_filters ?? [])];
  const [activeFilter, setActiveFilter] = useState(filters[0]);

  if (!commits) return null;

  const filteredCommits = filterCommits(
    activeFilter,
    commits,
    startDate,
    endDate,
  );

  return (
    <div className="flex flex-col h-full">
      <CommitsShortcuts setStartDate={setStartDate} setEndDate={setEndDate} />
      <CommitsHeader
        startDate={startDate}
        endDate={endDate}
        commitCount={filteredCommits.length}
        filters={filters}
        activeFilter={activeFilter}
        setActiveFilter={setActiveFilter}
      />
      <CommitsGrid
        commits={commits}
        startDate={startDate}
        endDate={endDate}
        setStartDate={setStartDate}
        setEndDate={setEndDate}
      />
      <CommitsList commits={filteredCommits} />
    </div>
  );
}

export default function Page() {
  return (
    <Suspense>
      <CommitsClient />
    </Suspense>
  );
}
