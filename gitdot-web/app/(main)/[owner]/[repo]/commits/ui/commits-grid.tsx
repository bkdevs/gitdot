import type { RepositoryCommitResource } from "gitdot-api";
import { addDays, cn, dateOnly } from "@/util";

const NUM_WEEKS = 53;
const NUM_DAYS = 7;
const CELL_HEIGHT = 18;
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

  return (
    <div className="flex flex-col gap-1 w-full h-45 p-3 border-b border-border">
      <div className="flex flex-row items-stretch flex-1 gap-2">
        {/* render both the cells and month labels as one css grid */}
        <div
          className="grid w-full"
          style={{
            gap: GAP_HEIGHT,
            gridTemplateColumns: `repeat(${NUM_WEEKS}, 1fr)`,
            gridTemplateRows: `repeat(${NUM_DAYS}, ${CELL_HEIGHT}px) auto`,
          }}
        >
          {weeks.flatMap((week, col) =>
            week.map((day, row) => (
              <div
                key={`cell-${day.date}`}
                className={cn("rounded-xs", colorForCount(day.commitCount))}
                style={{ gridRow: row + 1, gridColumn: col + 1 }}
                title={`${day.date}: ${day.commitCount} commits`}
              />
            )),
          )}
          {months.map((m) => (
            <span
              key={`${m.label}-${m.startingWeek}`}
              className="text-[10px] pl-[0.5] text-muted-foreground"
              style={{
                gridRow: NUM_DAYS + 1,
                gridColumn: `${m.startingWeek + 1} / span ${m.numWeeks}`,
              }}
            >
              {m.label}
            </span>
          ))}
        </div>

        {/* render the day labels as an aligned flex column */}
        <div className="flex flex-col pl-1" style={{ gap: GAP_HEIGHT }}>
          {["S", "M", "T", "W", "T", "F", "S"].map((d, i) => (
            <span
              // biome-ignore lint/suspicious/noArrayIndexKey: day-of-week labels have intentional duplicates
              key={`${d}-${i}`}
              className="text-[10px] text-muted-foreground flex items-center"
              style={{ height: CELL_HEIGHT }}
            >
              {d}
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
    const date = commit.date.slice(0, 10);
    countMap.set(date, (countMap.get(date) ?? 0) + 1);
  }

  const today = dateOnly(new Date());
  const todayDow = today.getDay();

  const startOfThisWeek = addDays(today, -todayDow);
  const earliest = addDays(startOfThisWeek, -52 * 7);

  const weeks: Week[] = [];
  const months: Month[] = [];
  let prevMonth = -1;

  for (let col = 0; col < NUM_WEEKS; col++) {
    const week: Day[] = [];
    for (let row = 0; row < NUM_DAYS; row++) {
      const d = addDays(earliest, col * 7 + row);
      if (d > today) {
        // do not include any divs for the future days in the current week
        break;
      }

      const dateStr = d.toISOString().slice(0, 10);
      week.push({
        date: dateStr,
        commitCount: countMap.get(dateStr) ?? 0,
      });
    }
    weeks.push(week);

    const date = addDays(earliest, col * 7);
    if (date.getMonth() !== prevMonth) {
      months.push({
        label: date.toLocaleString("en-US", { month: "short" }),
        startingWeek: col,
        numWeeks: 0,
      });
      prevMonth = date.getMonth();
    }
  }

  // calculate how many weeks each month spanned
  for (let i = 0; i < months.length; i++) {
    const next = months[i + 1];
    months[i].numWeeks = next
      ? next.startingWeek - months[i].startingWeek
      : NUM_WEEKS - months[i].startingWeek;
  }

  return { weeks, months };
}

function colorForCount(count: number): string {
  if (count === 0) return "bg-neutral-100 dark:bg-neutral-800";
  if (count <= 2) return "bg-green-200 dark:bg-green-900";
  if (count <= 5) return "bg-green-400 dark:bg-green-700";
  if (count <= 9) return "bg-green-500 dark:bg-green-600";
  return "bg-green-700 dark:bg-green-400";
}
