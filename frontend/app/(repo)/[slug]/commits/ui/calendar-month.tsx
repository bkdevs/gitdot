"use client";

import { CalendarDay } from "./calendar-day";
import { getCommitIntensity } from "@/util";
import type { RepositoryCommit } from "@/lib/dto";

interface CalendarMonthProps {
  year: number;
  month: number;
  monthName: string;
  daysInMonth: number;
  startDayOfWeek: number;
  monthKey: string;
  commitsMap: Map<number, RepositoryCommit[]>;
  maxCommitsPerDay: number;
}

const dayLabels = ["S", "M", "T", "W", "T", "F", "S"];

export function CalendarMonth({
  year,
  month,
  monthName,
  daysInMonth,
  startDayOfWeek,
  monthKey,
  commitsMap,
  maxCommitsPerDay,
}: CalendarMonthProps) {
  // Calculate today's date for highlighting
  const today = new Date();
  const todayYear = today.getFullYear();
  const todayMonth = today.getMonth();
  const todayDay = today.getDate();

  return (
    <div className="flex flex-col gap-3">
      {/* Month header */}
      <h3 className="text-sm font-semibold">
        {monthName}
      </h3>

      {/* Day labels */}
      <div className="grid grid-cols-7 gap-1">
        {dayLabels.map((label) => (
          <div
            key={label}
            className="text-xs text-center text-muted-foreground font-medium"
          >
            {label}
          </div>
        ))}
      </div>

      {/* Calendar grid */}
      <div className="grid grid-cols-7 gap-1">
        {/* Padding cells */}
        {Array.from({ length: startDayOfWeek }).map((_, i) => (
          <div key={`empty-${i}`} />
        ))}

        {/* Day cells */}
        {Array.from({ length: daysInMonth }).map((_, i) => {
          const day = i + 1;
          const dayCommits = commitsMap.get(day) || [];
          const intensity = getCommitIntensity(
            dayCommits.length,
            maxCommitsPerDay,
          );

          // Check if this is today
          const isToday =
            year === todayYear && month === todayMonth && day === todayDay;

          return (
            <CalendarDay
              key={day}
              day={day}
              commits={dayCommits}
              intensity={intensity}
              monthKey={monthKey}
              isToday={isToday}
            />
          );
        })}
      </div>
    </div>
  );
}
