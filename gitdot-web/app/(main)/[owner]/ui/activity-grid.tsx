"use client";

import { addDays, cn, dateOnly, subtractDays } from "@/util";

const CELL_HEIGHT = 10;
const GAP = 1;
const NUM_WEEKS = 53;
const NUM_DAYS = 7;

type Day = { date: string; count: number };
type Week = Day[];
type Month = { label: string; startingWeek: number; numWeeks: number };

function buildGrid(counts: Map<string, number>): {
  weeks: Week[];
  months: Month[];
} {
  const today = dateOnly(new Date());
  const thisWeekStart = subtractDays(today, today.getDay());

  const weeks: Week[] = [];
  const months: Month[] = [];
  let prevMonth = -1;

  for (let col = 0; col < NUM_WEEKS; col++) {
    const weekStart = subtractDays(thisWeekStart, col * 7);
    const week: Day[] = [];

    for (let row = 0; row < NUM_DAYS; row++) {
      const d = addDays(weekStart, row);
      if (d > today) break;
      const dateStr = d.toISOString().slice(0, 10);
      week.push({ date: dateStr, count: counts.get(dateStr) ?? 0 });
    }
    weeks.push(week);

    if (weekStart.getMonth() !== prevMonth) {
      months.push({
        label: weekStart.toLocaleString("en-US", { month: "short" }),
        startingWeek: col,
        numWeeks: 0,
      });
      prevMonth = weekStart.getMonth();
    }
  }

  for (let i = 0; i < months.length; i++) {
    const next = months[i + 1];
    months[i].numWeeks = next
      ? next.startingWeek - months[i].startingWeek
      : NUM_WEEKS - months[i].startingWeek;
  }

  // Reverse so oldest week is leftmost, most recent is rightmost
  weeks.reverse();
  for (const month of months) {
    month.startingWeek = NUM_WEEKS - month.startingWeek - month.numWeeks;
  }
  months.sort((a, b) => a.startingWeek - b.startingWeek);

  return { weeks, months };
}

function cellColor(count: number): string {
  if (count === 0) return "bg-commit-grid-empty";
  if (count <= 1) return "bg-commit-grid-low";
  if (count <= 3) return "bg-commit-grid-med";
  if (count <= 5) return "bg-commit-grid-high";
  return "bg-commit-grid-max";
}

export function ActivityGrid({
  counts,
  startDate,
  endDate: _endDate,
  setStartDate,
  setEndDate,
}: {
  counts: Map<string, number>;
  startDate: string | null;
  endDate: string | null;
  setStartDate: (d: string | null) => void;
  setEndDate: (d: string | null) => void;
}) {
  const { weeks, months } = buildGrid(counts);
  const selectedMonth = startDate ? startDate.slice(0, 7) : null;

  return (
    <div className="flex w-full">
      {months.map((m) => {
        const monthWeeks = weeks.slice(
          m.startingWeek,
          m.startingWeek + m.numWeeks,
        );
        const days = monthWeeks.flat();
        if (days.length === 0) return null;

        const sorted = days.map((d) => d.date).sort();
        const first = sorted[0];
        const last = sorted[sorted.length - 1];
        const isSelected = selectedMonth === first.slice(0, 7);
        const isDimmed = selectedMonth !== null && !isSelected;

        return (
          <button
            key={m.label}
            type="button"
            className={cn(
              "flex flex-col gap-1 transition-opacity duration-200 min-w-0 cursor-pointer appearance-none bg-transparent border-none p-0 text-left",
              isDimmed && "opacity-25",
            )}
            style={{ flex: m.numWeeks }}
            onClick={() => {
              if (isSelected) {
                setStartDate(null);
                setEndDate(null);
              } else {
                setStartDate(first);
                setEndDate(last);
              }
            }}
          >
            <div
              className="grid w-full"
              style={{
                gap: GAP,
                gridTemplateColumns: `repeat(${m.numWeeks}, 1fr)`,
                gridTemplateRows: `repeat(${NUM_DAYS}, ${CELL_HEIGHT}px)`,
              }}
            >
              {monthWeeks.flatMap((week, col) =>
                week.map((day, row) => (
                  <div
                    key={day.date}
                    className={cn(
                      "w-full h-full rounded-[1px]",
                      cellColor(day.count),
                    )}
                    style={{ gridRow: row + 1, gridColumn: col + 1 }}
                    title={`${day.date}: ${day.count} commits`}
                  />
                )),
              )}
            </div>

            <span
              className={cn(
                "text-[10px] text-left truncate transition-colors hover:text-foreground select-none w-full",
                isSelected ? "text-foreground" : "text-muted-foreground",
              )}
            >
              {m.label}
            </span>
          </button>
        );
      })}
    </div>
  );
}
