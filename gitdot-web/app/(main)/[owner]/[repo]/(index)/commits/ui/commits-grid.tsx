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
import { formatDate, inRange } from "@/util/date";
import {
  buildGrid,
  cellColor,
  computeThresholds,
  NUM_DAYS,
  recentWindowEnd,
  recentWindowStart,
} from "../util";

const CELL_HEIGHT = 15;
const GAP_HEIGHT = 2;

/**
 * renders a calendar view of commits, few notes:
 * - uses css-rendering only
 * - the visible range is controlled by windowStart/windowEnd (set via the dropdown)
 * - a sub-range can be highlighted via selectedStart/selectedEnd (drag / month-click)
 * - cell height is fixed but width is determined by the size of the outer container
 */
export function CommitsGrid({
  commits,
  repository,
  windowStart,
  windowEnd,
  setWindowStart,
  setWindowEnd,
  selectedStart,
  selectedEnd,
  setSelectedStart,
  setSelectedEnd,
}: {
  commits: RepositoryCommitResource[];
  repository: RepositoryResource | null;
  windowStart: string;
  windowEnd: string;
  setWindowStart: (date: string) => void;
  setWindowEnd: (date: string) => void;
  selectedStart: string | null;
  selectedEnd: string | null;
  setSelectedStart: (date: string | null) => void;
  setSelectedEnd: (date: string | null) => void;
}) {
  const [hoverActive, setHoverActive] = useState(false);
  const { onCellMouseDown, onCellMouseEnter } = useDragSelect(
    selectedStart,
    selectedEnd,
    setSelectedStart,
    setSelectedEnd,
    setHoverActive,
  );

  const { weeks, months, numWeeks } = buildGrid(
    commits,
    windowStart,
    windowEnd,
  );
  const thresholds = computeThresholds(
    weeks.flatMap((w) => w.map((d) => d.commitCount)),
  );
  const dayOfWeek = new Date().getDay();
  const dimmed = hoverActive || !!(selectedStart && selectedEnd);

  const commitsInRange = commits.filter((c) =>
    inRange(
      c.date.slice(0, 10),
      selectedStart ?? windowStart,
      selectedEnd ?? windowEnd,
    ),
  );

  return (
    <div className="flex flex-col w-full h-42 border-b border-border">
      {/* header */}
      <div className="flex items-center px-1 h-6 border-b border-border shrink-0">
        <DateDropdown
          commits={commits}
          repository={repository}
          windowStart={windowStart}
          windowEnd={windowEnd}
          selectedStart={selectedStart}
          selectedEnd={selectedEnd}
          commitsInRange={commitsInRange}
          setWindowStart={setWindowStart}
          setWindowEnd={setWindowEnd}
          setSelectedStart={setSelectedStart}
          setSelectedEnd={setSelectedEnd}
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
            gridTemplateColumns: `repeat(${numWeeks}, 1fr)`,
            gridTemplateRows: `repeat(${NUM_DAYS}, ${CELL_HEIGHT}px)`,
          }}
          onMouseEnter={() => setHoverActive(true)}
          onMouseLeave={() => setHoverActive(false)}
        >
          {weeks.flatMap((week, col) =>
            week.map((day) => {
              const row = new Date(`${day.date}T00:00:00`).getDay();
              return (
                <button
                  key={`cell-${day.date}`}
                  type="button"
                  className="group appearance-none border-none bg-transparent -m-px p-px"
                  style={{ gridRow: row + 1, gridColumn: numWeeks - col }}
                  title={`${day.date}: ${day.commitCount} commits`}
                  onMouseDown={(e) => onCellMouseDown(day.date, e)}
                  onMouseEnter={() => onCellMouseEnter(day.date)}
                >
                  <div
                    className={cn(
                      "w-full h-full transition-opacity duration-300 group-hover:duration-0",
                      cellColor(day.commitCount, thresholds),
                      inRange(day.date, selectedStart, selectedEnd)
                        ? "opacity-100! ring-1 ring-inset ring-foreground"
                        : cn(
                            dimmed && "opacity-40",
                            "group-hover:opacity-100!",
                          ),
                    )}
                  />
                </button>
              );
            }),
          )}
        </div>
      </div>

      {/* month labels below */}
      <div className="flex flex-row border-t border-border">
        <div
          className="grid w-full pl-1 pb-1"
          style={{ gridTemplateColumns: `repeat(${numWeeks}, 1fr)` }}
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
                gridColumn: `${numWeeks - m.startingWeek - m.numWeeks + 1} / span ${m.numWeeks}`,
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

                if (selectedStart === first && selectedEnd === last) {
                  setSelectedStart(null);
                  setSelectedEnd(null);
                } else {
                  setSelectedStart(first);
                  setSelectedEnd(last);
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
  selectedStart: string | null,
  selectedEnd: string | null,
  setSelectedStart: (d: string | null) => void,
  setSelectedEnd: (d: string | null) => void,
  setHoverActive: (active: boolean) => void,
) {
  const isDraggingRef = useRef(false);
  const pendingStartRef = useRef<string | null>(null);

  useEffect(() => {
    const onMouseUp = () => {
      if (pendingStartRef.current !== null) {
        setSelectedStart(null);
        setSelectedEnd(null);
        setHoverActive(false);
      }
      isDraggingRef.current = false;
      pendingStartRef.current = null;
    };
    window.addEventListener("mouseup", onMouseUp);
    return () => window.removeEventListener("mouseup", onMouseUp);
  }, [setSelectedStart, setSelectedEnd, setHoverActive]);

  const onCellMouseDown = (date: string, e: React.MouseEvent) => {
    e.preventDefault();
    isDraggingRef.current = true;
    const isRange = selectedStart !== selectedEnd;
    const isSameDate = selectedStart === date;
    if (selectedStart && selectedEnd && (isRange || isSameDate)) {
      pendingStartRef.current = date;
    } else {
      setSelectedStart(date);
      setSelectedEnd(date);
    }
  };

  const onCellMouseEnter = (date: string) => {
    if (!isDraggingRef.current) return;
    if (pendingStartRef.current !== null) {
      setSelectedStart(pendingStartRef.current);
      pendingStartRef.current = null;
    }
    setSelectedEnd(date);
  };

  return { onCellMouseDown, onCellMouseEnter };
}

function DateDropdown({
  commits,
  repository,
  windowStart,
  windowEnd,
  selectedStart,
  selectedEnd,
  commitsInRange,
  setWindowStart,
  setWindowEnd,
  setSelectedStart,
  setSelectedEnd,
}: {
  commits: RepositoryCommitResource[];
  repository: RepositoryResource | null;
  windowStart: string;
  windowEnd: string;
  selectedStart: string | null;
  selectedEnd: string | null;
  commitsInRange: RepositoryCommitResource[];
  setWindowStart: (date: string) => void;
  setWindowEnd: (date: string) => void;
  setSelectedStart: (date: string | null) => void;
  setSelectedEnd: (date: string | null) => void;
}) {
  const currentYear = new Date().getFullYear();
  const createdYear = repository?.created_at
    ? new Date(repository.created_at).getFullYear()
    : currentYear;
  const latestYear = commits[0]?.date
    ? new Date(commits[0].date).getFullYear()
    : currentYear;

  const presets: { label: string; start: string; end: string }[] = [
    {
      label: "Recent",
      start: recentWindowStart(commits),
      end: recentWindowEnd(commits),
    },
  ];
  for (let y = createdYear; y <= latestYear; y++) {
    presets.push({
      label: String(y),
      start: `${y}-01-01`,
      end: `${y}-12-31`,
    });
  }

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <button
          type="button"
          className="flex items-center gap-0.5 text-xs font-mono text-muted-foreground hover:text-foreground transition-colors"
        >
          {pluralize(commitsInRange.length, "commit")}:{" "}
          {formatDate(new Date(`${selectedStart ?? windowStart}T00:00:00`))} –{" "}
          {formatDate(new Date(`${selectedEnd ?? windowEnd}T00:00:00`))}
          <ChevronDownIcon className="size-3 shrink-0" />
        </button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="start" className="min-w-0">
        {presets.map((opt) => (
          <DropdownMenuItem
            key={opt.label}
            onClick={() => {
              setWindowStart(opt.start);
              setWindowEnd(opt.end);
              setSelectedStart(null);
              setSelectedEnd(null);
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
