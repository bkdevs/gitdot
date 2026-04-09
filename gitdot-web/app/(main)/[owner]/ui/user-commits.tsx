"use client";

import type { RepositoryCommitResource } from "gitdot-api";
import { useState } from "react";
import { cn } from "@/util";
import { inRange } from "@/util/date";
import { cellColor, computeThresholds } from "../[repo]/(index)/commits/util";

export function UserCommits({
  commits,
}: {
  commits: RepositoryCommitResource[];
}) {
  const [startDate, setStartDate] = useState<string | null>(null);
  const [endDate, setEndDate] = useState<string | null>(null);

  const commitMap = new Map<string, RepositoryCommitResource[]>();
  for (const c of commits) {
    const day = c.date.slice(0, 10);
    if (!commitMap.has(day)) commitMap.set(day, []);
    commitMap.get(day)?.push(c);
  }

  return (
    <>
      <CommitCalendar
        commits={commitMap}
        startDate={startDate}
        endDate={endDate}
        setStartDate={setStartDate}
        setEndDate={setEndDate}
      />
      <CommitLog commits={commitMap} startDate={startDate} endDate={endDate} />
    </>
  );
}

function CommitCalendar({
  commits,
  startDate,
  endDate,
  setStartDate,
  setEndDate,
}: {
  commits: Map<string, RepositoryCommitResource[]>;
  startDate: string | null;
  endDate: string | null;
  setStartDate: (date: string | null) => void;
  setEndDate: (date: string | null) => void;
}) {
  const today = new Date().toISOString().slice(0, 10);
  const months = trailingMonths();
  const selectedMonth = startDate ? startDate.slice(0, 7) : null;

  const counts = new Map<string, number>();
  for (const [day, cs] of commits) counts.set(day, cs.length);
  const thresholds = computeThresholds([...counts.values()]);

  return (
    <div>
      <p className="text-xs text-muted-foreground font-mono mb-2">
        <span className="text-foreground/40 select-none"># </span>Activity
      </p>
      <div className="grid grid-cols-6 gap-x-4 gap-y-5">
        {months.map(({ year, month }) => {
          const monthStr = `${year}-${String(month + 1).padStart(2, "0")}`;
          const label = `${new Date(year, month).toLocaleString("en-US", { month: "short" })} '${String(year).slice(2)}`;
          const cells = monthCells(year, month);
          const isSelected = selectedMonth === monthStr;
          const isDimmed = selectedMonth !== null && !isSelected;
          const firstDay = `${monthStr}-01`;
          const lastDay = `${monthStr}-${String(new Date(year, month + 1, 0).getDate()).padStart(2, "0")}`;

          return (
            <button
              key={monthStr}
              type="button"
              className={cn(
                "flex flex-col gap-1 transition-opacity duration-200 cursor-pointer appearance-none bg-transparent border-none p-0 text-left",
                isDimmed && "opacity-25",
              )}
              onClick={() => {
                if (isSelected) {
                  setStartDate(null);
                  setEndDate(null);
                } else {
                  setStartDate(firstDay);
                  setEndDate(lastDay);
                }
              }}
            >
              <span
                className={cn(
                  "text-[10px] font-mono select-none mb-0.5",
                  isSelected ? "text-foreground" : "text-muted-foreground",
                )}
              >
                {label}
              </span>
              <div className="grid grid-cols-7 w-full gap-px">
                {cells.map((dateStr, i) => {
                  if (dateStr === null) {
                    return <div key={`e${i}`} className="aspect-square" />;
                  }
                  const count = counts.get(dateStr) ?? 0;
                  const isFuture = dateStr > today;
                  return (
                    <div
                      key={dateStr}
                      className={cn(
                        "aspect-square rounded-[1px]",
                        isFuture ? "opacity-0" : cellColor(count, thresholds),
                      )}
                      title={`${dateStr}: ${count} commits`}
                    />
                  );
                })}
              </div>
            </button>
          );
        })}
      </div>
    </div>
  );
}

function trailingMonths(): { year: number; month: number }[] {
  const now = new Date();
  const result = [];
  for (let i = 11; i >= 0; i--) {
    const d = new Date(now.getFullYear(), now.getMonth() - i, 1);
    result.push({ year: d.getFullYear(), month: d.getMonth() });
  }
  return result;
}

function monthCells(year: number, month: number): (string | null)[] {
  const firstDow = new Date(year, month, 1).getDay();
  const daysInMonth = new Date(year, month + 1, 0).getDate();
  const cells: (string | null)[] = Array(firstDow).fill(null);
  for (let d = 1; d <= daysInMonth; d++) {
    cells.push(
      `${year}-${String(month + 1).padStart(2, "0")}-${String(d).padStart(2, "0")}`,
    );
  }
  while (cells.length % 7 !== 0) cells.push(null);
  return cells;
}

function CommitLog({
  commits,
  startDate,
  endDate,
}: {
  commits: Map<string, RepositoryCommitResource[]>;
  startDate: string | null;
  endDate: string | null;
}) {
  const now = new Date();
  const today = now.toISOString().slice(0, 10);
  const trailingStart = new Date(now.getFullYear(), now.getMonth() - 11, 1)
    .toISOString()
    .slice(0, 10);

  const logDays = [...commits.entries()]
    .sort((a, b) => b[0].localeCompare(a[0]))
    .map(([date, cs]) => ({ date, commits: cs }));

  const visibleDays =
    startDate && endDate
      ? logDays.filter(({ date }) => inRange(date, startDate, endDate))
      : logDays.filter(({ date }) => date >= trailingStart && date <= today);

  return (
    <div>
      <div className="flex items-baseline justify-between mb-2">
        <p className="text-xs text-muted-foreground font-mono">
          <span className="text-foreground/40 select-none"># </span>Log
        </p>
        <span className="text-xs font-mono text-muted-foreground/60">
          {(startDate ?? now.toISOString()).slice(0, 7)} (
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
    </div>
  );
}
