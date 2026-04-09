"use client";

import { inRange } from "@/util/date";
import type { RepositoryCommitResource } from "gitdot-api";

export function UserCommitsLog({
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
  const visibleDays = [...commits.entries()]
    .sort((a, b) => b[0].localeCompare(a[0]))
    .filter(
      ([date]) =>
      inRange(date, startDate, endDate) &&
      (selectedMonth === null || date.startsWith(selectedMonth)),
    )
    .map(([date, cs]) => ({ date, commits: cs }));

  return (
    <div className="flex flex-col gap-4 mt-6">
      {visibleDays.map(({ date, commits: dayCommits }) => (
        <div key={date}>
          <p className="text-xs text-muted-foreground font-mono mb-2">
            <span className="text-foreground/40 select-none">## </span>
            {date}
          </p>
          {dayCommits.length === 0 ? (
            <p className="text-xs text-muted-foreground/50 font-mono">—</p>
          ) : (
            <div className="flex flex-col gap-1.5">
              {dayCommits.map((c) => {
                const added = c.diffs.reduce((s, d) => s + d.lines_added, 0);
                const removed = c.diffs.reduce(
                  (s, d) => s + d.lines_removed,
                  0,
                );
                return (
                  <div key={c.sha} className="flex items-baseline gap-2">
                    <span className="text-xs font-mono text-muted-foreground shrink-0">
                      {c.sha.slice(0, 7)}
                    </span>
                    <span className="text-xs flex-1">{c.message}</span>
                    <span className="text-xs font-mono text-muted-foreground/50 shrink-0">
                      {c.diffs.length} files
                    </span>
                    <span className="text-xs font-mono text-green-600 dark:text-green-500 shrink-0">
                      +{added}
                    </span>
                    <span className="text-xs font-mono text-red-600 dark:text-red-500 shrink-0">
                      -{removed}
                    </span>
                  </div>
                );
              })}
            </div>
          )}
        </div>
      ))}
    </div>
  );
}
