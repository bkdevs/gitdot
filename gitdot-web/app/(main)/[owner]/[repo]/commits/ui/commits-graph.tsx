import type { RepositoryCommitResource } from "gitdot-api";
import { cn } from "@/util";

const NUM_WEEKS = 53;
const NUM_DAYS = 7;

type DayCell = {
  date: string;
  count: number;
  inRange: boolean;
};

type MonthLabel = {
  label: string;
  colIndex: number;
};

const MONTH_NAMES = [
  "Jan",
  "Feb",
  "Mar",
  "Apr",
  "May",
  "Jun",
  "Jul",
  "Aug",
  "Sep",
  "Oct",
  "Nov",
  "Dec",
];

const DAY_LABELS = ["S", "M", "T", "W", "T", "F", "S"];

function buildCommitGrid(commits: RepositoryCommitResource[]): {
  weeks: DayCell[][];
  months: MonthLabel[];
} {
  const countMap = new Map<string, number>();
  for (const commit of commits) {
    const date = commit.date.slice(0, 10);
    countMap.set(date, (countMap.get(date) ?? 0) + 1);
  }

  const today = new Date();
  today.setHours(0, 0, 0, 0);
  const todayDow = today.getDay();

  const startOfThisWeek = new Date(today);
  startOfThisWeek.setDate(today.getDate() - todayDow);

  const earliest = new Date(startOfThisWeek);
  earliest.setDate(startOfThisWeek.getDate() - 52 * 7);

  const weeks: DayCell[][] = [];
  const months: MonthLabel[] = [];
  let prevMonth = -1;

  for (let col = 0; col < NUM_WEEKS; col++) {
    const week: DayCell[] = [];
    for (let row = 0; row < NUM_DAYS; row++) {
      const d = new Date(earliest);
      d.setDate(earliest.getDate() + col * 7 + row);
      const inRange = d <= today;
      const dateStr = d.toISOString().slice(0, 10);
      week.push({
        date: dateStr,
        count: countMap.get(dateStr) ?? 0,
        inRange,
      });
    }
    weeks.push(week);

    const month = new Date(earliest);
    month.setDate(earliest.getDate() + col * 7);
    const m = month.getMonth();
    if (m !== prevMonth) {
      months.push({ label: MONTH_NAMES[m], colIndex: col });
      prevMonth = m;
    }
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

const ROW_SIZE = 18;
const ROW_STEP = ROW_SIZE + 2; // gap is 2px

function GraphGrid({ weeks }: { weeks: DayCell[][] }) {
  return (
    <div
      className="grid w-full gap-[2px]"
      style={{
        gridTemplateRows: `repeat(${NUM_DAYS}, ${ROW_SIZE}px)`,
        gridTemplateColumns: `repeat(${NUM_WEEKS}, 1fr)`,
        gridAutoFlow: "column",
      }}
    >
      {weeks.flat().map((cell, i) => (
        <div
          // biome-ignore lint/suspicious/noArrayIndexKey: stable index grid
          key={i}
          className={cn(
            "rounded-[2px]",
            cell.inRange ? colorForCount(cell.count) : "bg-transparent",
          )}
          title={
            cell.inRange ? `${cell.date}: ${cell.count} commits` : undefined
          }
        />
      ))}
    </div>
  );
}

function MonthLabels({ months }: { months: MonthLabel[] }) {
  return (
    <div className="relative w-full h-[14px]">
      {months.map((m) => (
        <span
          key={`${m.label}-${m.colIndex}`}
          className="absolute text-[10px] text-muted-foreground"
          style={{ left: `${(m.colIndex / NUM_WEEKS) * 100}%` }}
        >
          {m.label}
        </span>
      ))}
    </div>
  );
}

function DayLabels() {
  return (
    <div className="flex flex-col gap-[2px] pl-1">
      {DAY_LABELS.map((d, i) => (
        <span
          // biome-ignore lint/suspicious/noArrayIndexKey: stable day-of-week index
          key={i}
          className="text-[10px] text-muted-foreground flex items-center"
          style={{ height: ROW_SIZE }}
        >
          {d}
        </span>
      ))}
    </div>
  );
}

export function CommitsGraph({
  commits,
}: {
  commits: RepositoryCommitResource[];
}) {
  const { weeks, months } = buildCommitGrid(commits);

  return (
    <div className="w-full flex flex-col gap-1 h-45 p-3 border-b border-border">
      <div className="flex flex-row items-stretch flex-1 gap-2">
        <GraphGrid weeks={weeks} />
        <DayLabels />
      </div>
      <MonthLabels months={months} />
    </div>
  );
}
