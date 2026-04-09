"use client";

import type { RepositoryCommitResource } from "gitdot-api";
import { inRange } from "@/util/date";

export function UserCommitStatistics({
  commits,
  startDate,
  endDate,
  selectedMonth,
}: {
  commits: Map<string, RepositoryCommitResource[]>;
  startDate: string;
  endDate: string;
  selectedMonth: string | null;
}) {
  if (selectedMonth === null) return null;

  const visibleCommits = [...commits.entries()]
    .filter(
      ([date]) =>
        inRange(date, startDate, endDate) && date.startsWith(selectedMonth),
    )
    .flatMap(([, cs]) => cs);

  const repoCounts = new Map<string, number>();
  for (const c of visibleCommits) {
    repoCounts.set(c.repo_name, (repoCounts.get(c.repo_name) ?? 0) + 1);
  }
  const repoList = [...repoCounts.entries()].sort((a, b) => b[1] - a[1]);
  const totalCommits = visibleCommits.length;

  const [year, month] = selectedMonth.split("-").map(Number);
  const monthLabel = new Date(year, month - 1).toLocaleString("en-US", {
    month: "long",
  });

  const sentence =
    `${totalCommits} commits to ` +
    `${repoList.map(([r, c]) => `${r} (${c})`).join(", ")}`;

  return (
    <div className="mt-6">
      <p className="text-xs text-muted-foreground font-mono mb-1">
        {monthLabel}
      </p>
      <p className="text-xs font-mono">{sentence}</p>
    </div>
  );
}
