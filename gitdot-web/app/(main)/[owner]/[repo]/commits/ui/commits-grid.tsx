"use client";

import type { RepositoryCommitResource } from "gitdot-api";
import { useEffect, useRef, useState } from "react";
import { cn } from "@/util";
import {
  buildGrid,
  computeThresholds,
  isSelected,
  NUM_DAYS,
  NUM_WEEKS,
  type Thresholds,
} from "../util";

const CELL_HEIGHT = 20;
const GAP_HEIGHT = 2;

/**
 * renders a calendar view of commits, few notes:
 * - uses css-rendering only
 * - fixed to showing the last year of commits
 * - cell height is fixed but width is determined by the size of the outer container
 */
export function CommitsGrid({
  commits,
  startDate,
  endDate,
  setStartDate,
  setEndDate,
}: {
  commits: RepositoryCommitResource[];
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
  const thresholds = computeThresholds(weeks);
  const dayOfWeek = new Date().getDay();
  const dimmed = hoverActive || !!(startDate && endDate);

  return (
    <div className="flex flex-col w-full h-45 border-b border-border">
      {/* day labels and the grid are in the same row */}
      <div className="flex flex-row items-start flex-1 h-full">
        <div
          className="flex flex-col py-1 w-5 h-full border-r border-border"
          style={{ gap: GAP_HEIGHT }}
        >
          {["S", "M", "T", "W", "T", "F", "S"].map((d, i) => (
            <span
              // biome-ignore lint/suspicious/noArrayIndexKey: day-of-week labels have intentional duplicates
              key={`${d}-${i}`}
              className={cn(
                "text-[10px] flex items-center justify-center w-full",
                i === dayOfWeek ? "text-foreground" : "text-muted-foreground",
              )}
              style={{ height: CELL_HEIGHT }}
            >
              {d}
            </span>
          ))}
        </div>

        {/* biome-ignore lint/a11y/noStaticElementInteractions: mouse enter/leave are purely cosmetic, not interactive */}
        <div
          className="grid w-full py-1 px-1"
          style={{
            gap: GAP_HEIGHT,
            gridTemplateColumns: `repeat(${NUM_WEEKS}, 1fr)`,
            gridTemplateRows: `repeat(${NUM_DAYS}, ${CELL_HEIGHT}px)`,
          }}
          onMouseEnter={() => setHoverActive(true)}
          onMouseLeave={() => setHoverActive(false)}
        >
          {weeks.flatMap((week, col) =>
            week.map((day, row) => {
              const selected = isSelected(day.date, startDate, endDate);
              return (
                <button
                  key={`cell-${day.date}`}
                  type="button"
                  className="group appearance-none border-none bg-transparent -m-px p-px"
                  style={{ gridRow: row + 1, gridColumn: col + 1 }}
                  title={`${day.date}: ${day.commitCount} commits`}
                  onMouseDown={(e) => onCellMouseDown(day.date, e)}
                  onMouseEnter={() => onCellMouseEnter(day.date)}
                >
                  <div
                    className={cn(
                      "w-full h-full transition-opacity duration-300 group-hover:duration-0",
                      cellColor(day.commitCount, thresholds),
                      selected
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

      {/* month labels are in a row below, with a spacer to continue the day label border */}
      <div className="flex flex-row border-t border-border">
        <div className="w-5 shrink-0 border-r border-border" />

        <div
          className="grid w-full pl-1 pb-1"
          style={{ gridTemplateColumns: `repeat(${NUM_WEEKS}, 1fr)` }}
        >
          {months.map((m, i) => (
            <span
              key={`${m.label}-${m.startingWeek}`}
              className={cn(
                "text-[10px]",
                i === 0 ? "text-foreground" : "text-muted-foreground",
              )}
              style={{
                gridRow: 1,
                gridColumn: `${m.startingWeek + 1} / span ${m.numWeeks}`,
              }}
            >
              {m.label}
            </span>
          ))}
        </div>
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
    if (startDate && endDate) {
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

function cellColor(count: number, thresholds: Thresholds): string {
  const [low, med, high] = thresholds;
  if (count === 0) return "bg-commit-grid-empty";
  if (count <= low) return "bg-commit-grid-low";
  if (count <= med) return "bg-commit-grid-med";
  if (count <= high) return "bg-commit-grid-high";
  return "bg-commit-grid-max";
}
