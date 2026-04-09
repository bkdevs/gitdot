"use client";

import type { RepositoryCommitResource } from "gitdot-api";

export function UserCommitsHeader({
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
  return (
    <div className="flex items-baseline mb-2 justify-between">
      <span className="text-xs text-muted-foreground font-mono">
        <span className="text-foreground/40 select-none"># </span>Log
      </span>
      <span className="text-xs text-muted-foreground/60 font-mono">
        {selectedMonth
          ? `${formatYearMonth(selectedMonth)} (${[...commits.entries()].filter(([d]) => d.startsWith(selectedMonth)).reduce((s, [, cs]) => s + cs.length, 0)} commits)`
          : `${formatYearMonth(startDate.slice(0, 7))} - ${formatYearMonth(endDate.slice(0, 7))} (${[...commits.values()].reduce((s, cs) => s + cs.length, 0)} commits)`}
      </span>
    </div>
  )

}

function formatYearMonth(yearMonth: string): string {
  const [year, month] = yearMonth.split("-").map(Number);
  return new Date(year, month - 1).toLocaleString("en-US", {
    month: "short",
    year: "numeric",
  });
}
