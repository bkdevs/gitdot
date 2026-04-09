"use client";

import type { RepositoryCommitResource } from "gitdot-api";
import { cn } from "@/util";
import { cellColor, computeThresholds } from "../[repo]/(index)/commits/util";

export function UserCommitsCalendar({
  commits,
  startDate,
  endDate,
  selectedMonth,
  setSelectedMonth,
}: {
  commits: Map<string, RepositoryCommitResource[]>;
  startDate: string;
  endDate: string;
  selectedMonth: string | null;
  setSelectedMonth: (month: string | null) => void;
}) {
  const today = new Date().toISOString().slice(0, 10);
  const months = monthsBetween(startDate, endDate);

  const counts = new Map<string, number>();
  for (const [day, cs] of commits) counts.set(day, cs.length);
  const thresholds = computeThresholds([...counts.values()]);

  return (
    <div className="grid grid-cols-6 gap-x-4 gap-y-2">
      {months.map(({ year, month }) => {
        const monthStr = `${year}-${String(month + 1).padStart(2, "0")}`;
        const label = `${new Date(year, month).toLocaleString("en-US", { month: "short" })} '${String(year).slice(2)}`;
        const cells = monthCells(year, month);
        const isSelected = selectedMonth === monthStr;
        const isDimmed = selectedMonth !== null && !isSelected;

        return (
          <button
            key={monthStr}
            type="button"
            className={cn(
              "flex flex-col gap-1 transition-opacity duration-200 cursor-pointer appearance-none bg-transparent border-none p-0 text-left",
              isDimmed && "opacity-40",
            )}
            onClick={() => setSelectedMonth(isSelected ? null : monthStr)}
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
  );
}

function monthsBetween(
  start: string,
  end: string,
): { year: number; month: number }[] {
  const result: { year: number; month: number }[] = [];
  const [sy, sm] = start.slice(0, 7).split("-").map(Number);
  const [ey, em] = end.slice(0, 7).split("-").map(Number);
  let d = new Date(sy, sm - 1, 1);
  const endD = new Date(ey, em - 1, 1);
  while (d <= endD) {
    result.push({ year: d.getFullYear(), month: d.getMonth() });
    d = new Date(d.getFullYear(), d.getMonth() + 1, 1);
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
