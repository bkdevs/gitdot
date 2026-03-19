"use client";

import type { RepositoryCommitResource } from "gitdot-api";
import { Suspense, use } from "react";
import { useRepoContext } from "../context";
import { CommitsGrid } from "./ui/commits-grid";
import { CommitsHeader } from "./ui/commits-header";
import { CommitsList } from "./ui/commits-list";

// DEBUG: generate fake commits spread over the past year
function fakeDEBUGCommits(): RepositoryCommitResource[] {
  const commits: RepositoryCommitResource[] = [];
  const today = new Date();
  for (let i = 0; i < 365; i++) {
    const d = new Date(today.getTime() - i * 86400000);
    const count = Math.floor(Math.random() * 12);
    for (let j = 0; j < count; j++) {
      commits.push({
        sha: `fake-${i}-${j}`,
        parent_sha: `fake-${i}-${j}-parent`,
        message: "fake commit",
        date: d.toISOString(),
        author: { name: "dev", email: "dev@example.com" },
        diffs: [],
      });
    }
  }
  return commits;
}

export function CommitsClient() {
  const commits = use(useRepoContext().commits);
  if (!commits) return null;

  const debugCommits = fakeDEBUGCommits();

  return (
    <div className="flex flex-col h-full">
      <CommitsHeader />
      <CommitsGrid commits={debugCommits} />
      <CommitsList commits={commits} />
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
