import type { RepositoryCommitResource } from "gitdot-api";
import { addDays, cn, dateOnly, subtractDays } from "@/util";

const NUM_WEEKS = 53;
const NUM_DAYS = 7;
const CELL_HEIGHT = 20;
const GAP_HEIGHT = 2;

type Day = {
  date: string;
  commitCount: number;
};

type Week = Day[];

type Month = {
  label: string;
  startingWeek: number;
  numWeeks: number;
};

// [low, med, high] buckets
type Thresholds = [number, number, number];

/**
 * renders a calendar view of commits, few notes:
 * - uses css-rendering only
 * - fixed to showing the last year of commits
 * - cell height is fixed but width is determined by the size of the outer container
 */
export function CommitsGrid({
  commits,
}: {
  commits: RepositoryCommitResource[];
}) {
  const { weeks, months } = buildGrid(commits);
  const thresholds = computeThresholds(weeks);
  const dayOfWeek = new Date().getDay();

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

        <div
          className="grid w-full py-1 px-1"
          style={{
            gap: GAP_HEIGHT,
            gridTemplateColumns: `repeat(${NUM_WEEKS}, 1fr)`,
            gridTemplateRows: `repeat(${NUM_DAYS}, ${CELL_HEIGHT}px)`,
          }}
        >
          {weeks.flatMap((week, col) =>
            week.map((day, row) => (
              <div
                key={`cell-${day.date}`}
                className={cn(cellColor(day.commitCount, thresholds))}
                style={{ gridRow: row + 1, gridColumn: col + 1 }}
                title={`${day.date}: ${day.commitCount} commits`}
              />
            )),
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

function buildGrid(commits: RepositoryCommitResource[]): {
  weeks: Week[];
  months: Month[];
} {
  const countMap = new Map<string, number>();
  for (const commit of commits) {
    const date = commit.date.slice(0, 10); // iso date YYYY-MM-DD
    countMap.set(date, (countMap.get(date) ?? 0) + 1);
  }

  const today = dateOnly(new Date());
  const thisWeekStart = subtractDays(today, today.getDay());

  const weeks: Week[] = [];
  const months: Month[] = [];
  let prevMonth = -1;

  for (let col = 0; col < NUM_WEEKS; col++) {
    const weekStart: Date = subtractDays(thisWeekStart, col * 7);
    const week: Day[] = [];

    for (let row = 0; row < NUM_DAYS; row++) {
      const d = addDays(weekStart, row);
      if (d > today) break;

      const dateStr = d.toISOString().slice(0, 10);
      week.push({
        date: dateStr,
        commitCount: countMap.get(dateStr) ?? 0,
      });
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

  return { weeks, months };
}

function computeThresholds(weeks: Week[]): Thresholds {
  const counts = weeks
    .flatMap((w) => w.map((d) => d.commitCount))
    .sort((a, b) => a - b);
  if (counts.every((c) => c === 0)) return [1, 2, 3];

  const q = (p: number) => counts[Math.floor(p * (counts.length - 1))];
  return [q(0.25), q(0.5), q(0.75)];
}

function cellColor(count: number, thresholds: Thresholds): string {
  const [low, med, high] = thresholds;
  if (count === 0) return "bg-commit-grid-empty";
  if (count <= low) return "bg-commit-grid-low";
  if (count <= med) return "bg-commit-grid-med";
  if (count <= high) return "bg-commit-grid-high";
  return "bg-commit-grid-max";
}
