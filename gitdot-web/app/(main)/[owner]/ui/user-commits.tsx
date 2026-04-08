"use client";

import type { RepositoryCommitResource } from "gitdot-api";
import { ChevronDownIcon } from "lucide-react";
import { useState } from "react";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/ui/dropdown-menu";
import { ActivityGrid } from "./activity-grid";

function inRange(
  date: string,
  start: string | null,
  end: string | null,
): boolean {
  if (!start || !end) return false;
  const lo = start <= end ? start : end;
  const hi = start <= end ? end : start;
  return date >= lo && date <= hi;
}

export function UserCommits({
  commits,
}: {
  commits: RepositoryCommitResource[];
}) {
  const [startDate, setStartDate] = useState<string | null>(null);
  const [endDate, setEndDate] = useState<string | null>(null);
  const currentYear = new Date().getFullYear();
  const [selectedYear, setSelectedYear] = useState(currentYear);
  const years = [currentYear, currentYear - 1, currentYear - 2];

  const byDate = new Map<string, RepositoryCommitResource[]>();
  for (const c of commits) {
    const day = c.date.slice(0, 10);
    if (!byDate.has(day)) byDate.set(day, []);
    byDate.get(day)?.push(c);
  }

  const counts = new Map<string, number>();
  for (const [day, cs] of byDate) counts.set(day, cs.length);

  const logDays = [...byDate.entries()]
    .sort((a, b) => b[0].localeCompare(a[0]))
    .map(([date, cs]) => ({ date, commits: cs }));

  const visibleDays =
    startDate && endDate
      ? logDays.filter(({ date }) => inRange(date, startDate, endDate))
      : logDays;

  return (
    <>
      <div>
        <div className="flex items-baseline justify-between mb-2">
          <p className="text-xs text-muted-foreground font-mono">
            <span className="text-foreground/40 select-none"># </span>Activity
          </p>
          <DropdownMenu>
            <DropdownMenuTrigger className="flex items-center gap-0.5 text-xs font-mono text-muted-foreground hover:text-foreground transition-colors">
              {selectedYear}
              <ChevronDownIcon className="size-3" />
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end">
              {years.map((y) => (
                <DropdownMenuItem
                  key={y}
                  className={y === selectedYear ? "text-foreground" : ""}
                  onSelect={() => {
                    setSelectedYear(y);
                    setStartDate(null);
                    setEndDate(null);
                  }}
                >
                  {y}
                </DropdownMenuItem>
              ))}
            </DropdownMenuContent>
          </DropdownMenu>
        </div>
        <ActivityGrid
          counts={counts}
          startDate={startDate}
          endDate={endDate}
          setStartDate={setStartDate}
          setEndDate={setEndDate}
        />
      </div>

      <div>
        <div className="flex items-baseline justify-between mb-2">
          <p className="text-xs text-muted-foreground font-mono">
            <span className="text-foreground/40 select-none"># </span>Log
          </p>
          <span className="text-xs font-mono text-muted-foreground/60">
            {(startDate ?? new Date().toISOString()).slice(0, 7)} (
            {visibleDays.reduce((n, d) => n + d.commits.length, 0)} commits)
          </span>
        </div>
        <div className="flex flex-col gap-4">
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
                    const added = c.diffs.reduce(
                      (s, d) => s + d.lines_added,
                      0,
                    );
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
      </div>
    </>
  );
}
