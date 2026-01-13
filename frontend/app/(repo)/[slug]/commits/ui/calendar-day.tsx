"use client";

import { Tooltip, TooltipContent, TooltipTrigger } from "@/ui/tooltip";
import { cn } from "@/util";
import type { RepositoryCommit } from "@/lib/dto";

interface CalendarDayProps {
  day: number;
  commits: RepositoryCommit[];
  intensity: number;
  monthKey: string;
  isToday: boolean;
}

const intensityClasses = {
  0: "",
  1: "bg-primary/20",
  2: "bg-primary/40",
  3: "bg-primary/60",
  4: "bg-primary/80",
};

export function CalendarDay({
  day,
  commits,
  intensity,
  monthKey,
  isToday,
}: CalendarDayProps) {
  const dateKey = `${monthKey}-${String(day).padStart(2, "0")}`;
  const hasCommits = commits.length > 0;

  return (
    <Tooltip>
      <TooltipTrigger asChild>
        <div
          className={cn(
            "w-full aspect-square flex items-center justify-center text-sm transition-all",
            "rounded-full cursor-default",
            // Today's date gets red circle background
            isToday && "bg-red-500 text-white font-semibold hover:bg-red-600",
            // Color gradient for days with commits (not today)
            !isToday && intensityClasses[intensity as keyof typeof intensityClasses],
            !isToday && hasCommits && "hover:opacity-80",
            !isToday && !hasCommits && "hover:bg-muted/30",
          )}
        >
          {day}
        </div>
      </TooltipTrigger>
      {hasCommits && (
        <TooltipContent>
          <div className="text-xs">
            <div className="font-semibold">{dateKey}</div>
            <div className="text-muted-foreground">
              {commits.length} {commits.length === 1 ? "commit" : "commits"}
            </div>
            <div className="mt-1 max-w-xs space-y-0.5">
              {commits.slice(0, 3).map((c) => (
                <div key={c.sha} className="truncate">
                  • {c.message}
                </div>
              ))}
              {commits.length > 3 && (
                <div className="text-muted-foreground">
                  +{commits.length - 3} more
                </div>
              )}
            </div>
          </div>
        </TooltipContent>
      )}
    </Tooltip>
  );
}
