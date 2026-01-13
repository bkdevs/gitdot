"use client";

import { useMemo, useState } from "react";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/ui/tooltip";
import { groupCommitsByDate, getCommitIntensity, cn, formatTime } from "@/util";
import type { RepositoryCommit } from "@/lib/dto";

interface CommitsCalendarProps {
  commits: RepositoryCommit[];
}

const intensityClasses = {
  0: "bg-muted/30",
  1: "bg-primary/20",
  2: "bg-primary/40",
  3: "bg-primary/60",
  4: "bg-primary/80",
};

const dayLabels = ["S", "M", "T", "W", "T", "F", "S"];

export function CommitsCalendar({ commits }: CommitsCalendarProps) {
  const [selectedRange, setSelectedRange] = useState<{
    start: string | null;
    end: string | null;
  }>({ start: null, end: null });
  const [isDragging, setIsDragging] = useState(false);

  const calendarData = useMemo(() => {
    // Calculate date range (1 year ago to today)
    const today = new Date();
    today.setHours(0, 0, 0, 0);

    // Store the actual data range (for display)
    const actualStartDate = new Date(today);
    actualStartDate.setFullYear(today.getFullYear() - 1);
    actualStartDate.setDate(actualStartDate.getDate() + 1);

    const endDate = new Date(today);
    const startDate = new Date(actualStartDate);

    // Adjust start date to the beginning of the week (Sunday)
    const startDayOfWeek = startDate.getDay();
    startDate.setDate(startDate.getDate() - startDayOfWeek);

    // Adjust end date to the end of the week (Saturday)
    const endDayOfWeek = endDate.getDay();
    const daysUntilSaturday = 6 - endDayOfWeek;
    endDate.setDate(endDate.getDate() + daysUntilSaturday);

    // Format date range for display
    const dateRangeDisplay = `${actualStartDate.toLocaleDateString("en-US", {
      month: "long",
      day: "numeric",
      year: "numeric",
    })} - ${today.toLocaleDateString("en-US", {
      month: "long",
      day: "numeric",
      year: "numeric",
    })}`;

    // Group commits by date
    const commitsByDate = new Map(
      groupCommitsByDate(commits).map(([date, commits]) => [date, commits])
    );

    // Calculate max commits per day
    const maxCommitsPerDay = Math.max(
      1,
      ...Array.from(commitsByDate.values()).map((arr) => arr.length)
    );

    // Generate all days from start to end
    const days: Array<{
      date: Date;
      dateKey: string;
      commits: RepositoryCommit[];
      intensity: number;
      isFuture: boolean;
    }> = [];

    const currentDate = new Date(startDate);
    const todayKey = today.toISOString().split("T")[0];

    while (currentDate <= endDate) {
      const dateKey = currentDate.toISOString().split("T")[0];
      const dayCommits = commitsByDate.get(dateKey) || [];
      const intensity = getCommitIntensity(dayCommits.length, maxCommitsPerDay);
      const isFuture = currentDate > today;

      days.push({
        date: new Date(currentDate),
        dateKey,
        commits: dayCommits,
        intensity,
        isFuture,
      });

      currentDate.setDate(currentDate.getDate() + 1);
    }

    // Group days into weeks
    const weeks: typeof days[][] = [];
    for (let i = 0; i < days.length; i += 7) {
      weeks.push(days.slice(i, i + 7));
    }

    // Find month labels (show month when at least 2 days of new month appear)
    const monthLabels: Array<{ weekIndex: number; label: string }> = [];
    let lastMonth = -1;

    weeks.forEach((week, weekIndex) => {
      // Count days in this week that belong to each month
      const monthCounts = new Map<number, number>();
      week.forEach((day) => {
        if (!day.isFuture) {
          const month = day.date.getMonth();
          monthCounts.set(month, (monthCounts.get(month) || 0) + 1);
        }
      });

      // Find the predominant month in this week
      let predominantMonth = -1;
      let maxCount = 0;
      monthCounts.forEach((count, month) => {
        if (count > maxCount) {
          maxCount = count;
          predominantMonth = month;
        }
      });

      // Only show label if this is a new month with at least 2 days
      if (
        predominantMonth !== -1 &&
        predominantMonth !== lastMonth &&
        maxCount >= 2
      ) {
        const firstDayOfMonth = week.find(
          (d) => d.date.getMonth() === predominantMonth
        );
        if (firstDayOfMonth) {
          monthLabels.push({
            weekIndex,
            label: firstDayOfMonth.date.toLocaleDateString("en-US", {
              month: "short",
            }),
          });
          lastMonth = predominantMonth;
        }
      }
    });

    return { weeks, monthLabels, maxCommitsPerDay, dateRangeDisplay };
  }, [commits]);

  // Filter commits based on selected range
  const filteredCommits = useMemo(() => {
    if (!selectedRange.start) return [];

    const start = selectedRange.start;
    const end = selectedRange.end || selectedRange.start;

    return commits.filter((commit) => {
      const commitDate = commit.date.split("T")[0];
      return commitDate >= start && commitDate <= end;
    }).sort((a, b) => new Date(b.date).getTime() - new Date(a.date).getTime());
  }, [commits, selectedRange]);

  const handleMouseDown = (dateKey: string) => {
    setIsDragging(true);
    setSelectedRange({ start: dateKey, end: dateKey });
  };

  const handleMouseEnter = (dateKey: string) => {
    if (isDragging && selectedRange.start) {
      setSelectedRange((prev) => ({ ...prev, end: dateKey }));
    }
  };

  const handleMouseUp = () => {
    setIsDragging(false);
  };

  const isSelected = (dateKey: string) => {
    if (!selectedRange.start) return false;
    const start = selectedRange.start;
    const end = selectedRange.end || selectedRange.start;
    const min = start < end ? start : end;
    const max = start < end ? end : start;
    return dateKey >= min && dateKey <= max;
  };

  return (
    <div className="w-full h-screen flex flex-col" onMouseUp={handleMouseUp}>
      {/* Header bar */}
      <div className="flex flex-row w-full h-9 items-center border-b">
        <div className="flex-1 ml-2 text-sm font-mono">
          {calendarData.dateRangeDisplay} <span className="text-muted-foreground">
            (1432 commits)
          </span>
        </div>
      </div>

      {/* GitHub-style contribution graph */}
      <div className="pt-4 px-4 border-b border-border">
        <div className="flex gap-3">
          {/* Main grid */}
          <div className="flex-1">
            {/* Grid of squares */}
            <div className="flex w-full border-l border-t border-border overflow-hidden select-none">
              {calendarData.weeks.map((week, weekIndex) => (
                <div key={weekIndex} className="flex flex-col flex-1 min-w-[14px]">
                  {week.map((day) => {
                    const isToday =
                      day.dateKey ===
                      new Date().toISOString().split("T")[0];

                    const selected = isSelected(day.dateKey);

                    return (
                      <Tooltip key={day.dateKey}>
                        <TooltipTrigger asChild>
                          <div
                            className={cn(
                              "w-full aspect-square transition-all cursor-pointer border-r border-b border-border",
                              !day.isFuture && "hover:brightness-90",
                              selected && "ring-2 ring-inset ring-foreground",
                              day.isFuture
                                ? "bg-muted/10 opacity-30"
                                : intensityClasses[
                                    day.intensity as keyof typeof intensityClasses
                                  ],
                            )}
                            onMouseDown={() => !day.isFuture && handleMouseDown(day.dateKey)}
                            onMouseEnter={() => !day.isFuture && handleMouseEnter(day.dateKey)}
                            onMouseUp={handleMouseUp}
                          />
                        </TooltipTrigger>
                        <TooltipContent>
                          <div className="text-xs font-semibold">{day.dateKey}</div>
                        </TooltipContent>
                      </Tooltip>
                    );
                  })}
                </div>
              ))}
            </div>

            {/* Month labels */}
            <div className="flex h-6 mt-1 w-full">
              {calendarData.weeks.map((week, weekIndex) => {
                const monthLabel = calendarData.monthLabels.find(
                  (m) => m.weekIndex === weekIndex
                );
                return (
                  <div key={weekIndex} className="flex-1 min-w-[14px]">
                    {monthLabel && (
                      <div className="text-xs text-muted-foreground">
                        {monthLabel.label}
                      </div>
                    )}
                  </div>
                );
              })}
            </div>
          </div>

          {/* Day labels on the right */}
          <div className="flex flex-col -mt-1">
            {dayLabels.map((label, index) => (
              <div
                key={`${label}-${index}`}
                className="w-8 flex-1 text-xs text-muted-foreground flex items-center justify-center"
              >
                {label}
              </div>
            ))}
          </div>
        </div>
      </div>

      {/* Commits list view */}
      {filteredCommits.length > 0 && (
        <div className="flex-1 overflow-auto p-4">
          <div className="mb-3 text-sm font-semibold">
            {filteredCommits.length} {filteredCommits.length === 1 ? "commit" : "commits"}
            {selectedRange.start === selectedRange.end ? (
              <span className="text-muted-foreground font-normal"> on {selectedRange.start}</span>
            ) : (
              <span className="text-muted-foreground font-normal">
                {" "}from {selectedRange.start} to {selectedRange.end}
              </span>
            )}
          </div>
          <div className="space-y-2">
            {filteredCommits.map((commit) => (
              <div
                key={commit.sha}
                className="flex flex-col gap-0.5 py-1 hover:bg-muted/30 transition-colors"
              >
                <div className="flex items-center gap-2 text-xs text-muted-foreground">
                  <span className="font-mono">
                    {commit.sha.substring(0, 7)}
                  </span>
                  <span>•</span>
                  <span>{commit.author}</span>
                  <span>•</span>
                  <span>
                    {formatTime(new Date(commit.date))}
                  </span>
                </div>
                <div className="text-sm">{commit.message}</div>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
