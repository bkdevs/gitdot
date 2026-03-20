"use client";

import { Suspense, use, useState } from "react";
import { useRepoContext } from "../context";
import { CommitsGrid } from "./ui/commits-grid";
import { CommitsHeader } from "./ui/commits-header";
import { CommitsList } from "./ui/commits-list";
import { CommitsShortcuts } from "./shortcuts";
import { isSelected } from "./util";

export function CommitsClient() {
  const commits = use(useRepoContext().commits);
  const [startDate, setStartDate] = useState<string | null>(null);
  const [endDate, setEndDate] = useState<string | null>(null);

  if (!commits) return null;

  const filteredCommits =
    startDate && endDate
      ? commits.filter((c) =>
          isSelected(c.date.slice(0, 10), startDate, endDate),
        )
      : commits;

  return (
    <div className="flex flex-col h-full">
      <CommitsShortcuts setStartDate={setStartDate} setEndDate={setEndDate} />
      <CommitsHeader
        startDate={startDate}
        endDate={endDate}
        commitCount={filteredCommits.length}
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
