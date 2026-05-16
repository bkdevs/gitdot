"use client";

import type { RepositoryCommitResource, RepositoryResource } from "gitdot-api";
import { ChevronDownIcon } from "lucide-react";
import { useEffect, useRef, useState } from "react";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/ui/dropdown-menu";
import { cn, pluralize } from "@/util";
import { formatDate, inRange, subtractDays } from "@/util/date";
import {
  buildGrid,
  cellColor,
  computeThresholds,
  NUM_DAYS,
  NUM_WEEKS,
} from "../util";

const CELL_HEIGHT = 15;
const GAP_HEIGHT = 2;

/**
 * renders a calendar view of commits, few notes:
 * - uses css-rendering only
 * - fixed to showing the last year of commits
 * - cell height is fixed but width is determined by the size of the outer container
 */
export function CommitsGrid({
  commits,
  repository,
  startDate,
  endDate,
  setStartDate,
  setEndDate,
}: {
  commits: RepositoryCommitResource[];
  repository: RepositoryResource | null;
  startDate: string | null;
  endDate: string | null;
  setStartDate: (date: string | null) => void;
  setEndDate: (date: string | null) => void;
}) {
  const [hoverActive, setHoverActive] = useState(false);
  const { onCellMouseDown, onCellMouseEnter } = useDragSelect(
    startDate,
    endDate,
    setStartDate,
    setEndDate,
    setHoverActive,
  );

  const { weeks, months } = buildGrid(commits);
  const thresholds = computeThresholds(
    weeks.flatMap((w) => w.map((d) => d.commitCount)),
  );
  const dayOfWeek = new Date().getDay();
  const dimmed = hoverActive || !!(startDate && endDate);

  const today = new Date();
  const graphEnd = today.toISOString().slice(0, 10);
  const graphStart = new Date(today);
  graphStart.setDate(today.getDate() - today.getDay() - (NUM_WEEKS - 1) * 7);
  const graphStartDate = graphStart.toISOString().slice(0, 10);

  const displayStart = startDate ?? graphStartDate;
  const displayEnd = endDate ?? graphEnd;
  const commitsInRange =
    startDate && endDate
      ? commits.filter((c) => inRange(c.date.slice(0, 10), startDate, endDate))
      : commits;

  return (
    <div className="flex flex-col w-full h-42 border-b border-border">
      {/* header */}
      <div className="flex items-center px-1 h-6 border-b border-border shrink-0">
        <DateDropdown
          commits={commits}
          createdAt={repository?.created_at ?? null}
          displayStart={displayStart}
          displayEnd={displayEnd}
          commitsInRange={commitsInRange}
          setStartDate={setStartDate}
          setEndDate={setEndDate}
        />
      </div>

      {/* day labels and the grid are in the same row */}
      <div className="flex flex-row items-start flex-1 h-full">
        <div
          className="flex flex-col pt-1.5 w-5 h-full border-l border-border order-last"
          style={{ gap: GAP_HEIGHT }}
        >
          {["S", "M", "T", "W", "T", "F", "S"].map((d, i) => (
            <span
              key={`${d}-${i}`}
              className={cn(
                "text-[10px] flex items-center justify-center w-full select-none",
                i === dayOfWeek ? "text-foreground" : "text-muted-foreground",
              )}
              style={{ height: CELL_HEIGHT }}
            >
              {d}
            </span>
          ))}
        </div>

        <div
          className="grid w-full pt-1.5 pb-1 px-1.5"
          style={{
            gap: GAP_HEIGHT,
            gridTemplateColumns: `repeat(${NUM_WEEKS}, 1fr)`,
            gridTemplateRows: `repeat(${NUM_DAYS}, ${CELL_HEIGHT}px)`,
          }}
          onMouseEnter={() => setHoverActive(true)}
          onMouseLeave={() => setHoverActive(false)}
        >
          {weeks.flatMap((week, col) =>
            week.map((day, row) => (
              <button
                key={`cell-${day.date}`}
                type="button"
                className="group appearance-none border-none bg-transparent -m-px p-px"
                style={{ gridRow: row + 1, gridColumn: NUM_WEEKS - col }}
                title={`${day.date}: ${day.commitCount} commits`}
                onMouseDown={(e) => onCellMouseDown(day.date, e)}
                onMouseEnter={() => onCellMouseEnter(day.date)}
              >
                <div
                  className={cn(
                    "w-full h-full transition-opacity duration-300 group-hover:duration-0",
                    cellColor(day.commitCount, thresholds),
                    inRange(day.date, startDate, endDate)
                      ? "opacity-100! ring-1 ring-inset ring-foreground"
                      : cn(dimmed && "opacity-40", "group-hover:opacity-100!"),
                  )}
                />
              </button>
            )),
          )}
        </div>
      </div>

      {/* month labels below */}
      <div className="flex flex-row border-t border-border">
        <div
          className="grid w-full pl-1 pb-1"
          style={{ gridTemplateColumns: `repeat(${NUM_WEEKS}, 1fr)` }}
        >
          {months.map((m, i) => (
            <button
              key={`${m.label}-${m.startingWeek}`}
              type="button"
              className={cn(
                "text-[10px] text-left transition-colors hover:text-foreground cursor-pointer appearance-none bg-transparent border-none p-0 select-none",
                i === 0 ? "text-foreground" : "text-muted-foreground",
              )}
              style={{
                gridRow: 1,
                gridColumn: `${NUM_WEEKS - m.startingWeek - m.numWeeks + 1} / span ${m.numWeeks}`,
              }}
              onClick={() => {
                const monthWeeks = weeks.slice(
                  m.startingWeek,
                  m.startingWeek + m.numWeeks,
                );
                const days = monthWeeks.flat();
                if (days.length === 0) return;

                const sorted = days.map((d) => d.date).sort();
                const first = sorted[0];
                const last = sorted[sorted.length - 1];

                if (startDate === first && endDate === last) {
                  setStartDate(null);
                  setEndDate(null);
                } else {
                  setStartDate(first);
                  setEndDate(last);
                }
              }}
            >
              {m.label}
            </button>
          ))}
        </div>
        <div className="w-5 h-4 shrink-0 border-l border-border order-last" />
      </div>
    </div>
  );
}

function useDragSelect(
  startDate: string | null,
  endDate: string | null,
  setStartDate: (d: string | null) => void,
  setEndDate: (d: string | null) => void,
  setHoverActive: (active: boolean) => void,
) {
  const isDraggingRef = useRef(false);
  const pendingStartRef = useRef<string | null>(null);

  useEffect(() => {
    const onMouseUp = () => {
      if (pendingStartRef.current !== null) {
        setStartDate(null);
        setEndDate(null);
        setHoverActive(false);
      }
      isDraggingRef.current = false;
      pendingStartRef.current = null;
    };
    window.addEventListener("mouseup", onMouseUp);
    return () => window.removeEventListener("mouseup", onMouseUp);
  }, [setStartDate, setEndDate, setHoverActive]);

  const onCellMouseDown = (date: string, e: React.MouseEvent) => {
    e.preventDefault();
    isDraggingRef.current = true;
    const isRange = startDate !== endDate;
    const isSameDate = startDate === date;
    if (startDate && endDate && (isRange || isSameDate)) {
      pendingStartRef.current = date;
    } else {
      setStartDate(date);
      setEndDate(date);
    }
  };

  const onCellMouseEnter = (date: string) => {
    if (!isDraggingRef.current) return;
    if (pendingStartRef.current !== null) {
      setStartDate(pendingStartRef.current);
      pendingStartRef.current = null;
    }
    setEndDate(date);
  };

  return { onCellMouseDown, onCellMouseEnter };
}

function DateDropdown({
  commits,
  createdAt,
  displayStart,
  displayEnd,
  commitsInRange,
  setStartDate,
  setEndDate,
}: {
  commits: RepositoryCommitResource[];
  createdAt: string | null;
  displayStart: string;
  displayEnd: string;
  commitsInRange: RepositoryCommitResource[];
  setStartDate: (date: string | null) => void;
  setEndDate: (date: string | null) => void;
}) {
  const today = new Date();
  const todayStr = today.toISOString().slice(0, 10);
  const currentYear = today.getFullYear();
  const createdYear = createdAt
    ? new Date(createdAt).getFullYear()
    : currentYear;
  const mostRecent = commits[0]?.date.slice(0, 10) ?? todayStr;

  const presets: { label: string; start: string; end: string }[] = [
    {
      label: "Recent",
      start: subtractDays(new Date(`${mostRecent}T00:00:00`), 365)
        .toISOString()
        .slice(0, 10),
      end: mostRecent,
    },
  ];
  for (let y = currentYear; y >= createdYear; y--) {
    presets.push(
      y === currentYear
        ? {
            label: String(y),
            start: subtractDays(today, 365).toISOString().slice(0, 10),
            end: todayStr,
          }
        : {
            label: String(y),
            start: `${y}-01-01`,
            end: `${y}-12-31`,
          },
    );
  }

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <button
          type="button"
          className="flex items-center gap-0.5 text-xs font-mono text-muted-foreground hover:text-foreground transition-colors"
        >
          {pluralize(commitsInRange.length, "commit")}:{" "}
          {formatDate(new Date(`${displayStart}T00:00:00`))} –{" "}
          {formatDate(new Date(`${displayEnd}T00:00:00`))}
          <ChevronDownIcon className="size-3 shrink-0" />
        </button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="start" className="min-w-0">
        {presets.map((opt) => (
          <DropdownMenuItem
            key={opt.label}
            onClick={() => {
              setStartDate(opt.start);
              setEndDate(opt.end);
            }}
            className="text-xs font-mono py-1 px-2"
          >
            {opt.label}
          </DropdownMenuItem>
        ))}
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
