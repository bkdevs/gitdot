import type {
  CommitFilterResource,
  RepositoryCommitResource,
} from "gitdot-api";
import { addDays, dateOnly, subtractDays } from "@/util";

export const NUM_WEEKS = 53;
export const NUM_DAYS = 7;

export type Day = {
  date: string;
  commitCount: number;
};

export type Week = Day[];

export type Month = {
  label: string;
  startingWeek: number;
  numWeeks: number;
};

// [low, med, high] buckets
export type Thresholds = [number, number, number];

export function buildGrid(commits: RepositoryCommitResource[]): {
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

export function computeThresholds(weeks: Week[]): Thresholds {
  const nonZero = weeks
    .flatMap((w) => w.map((d) => d.commitCount))
    .filter((c) => c > 0)
    .sort((a, b) => a - b);
  if (nonZero.length === 0) return [1, 2, 3];

  const q = (p: number) => nonZero[Math.floor(p * (nonZero.length - 1))];
  return [q(0.25), q(0.5), q(0.75)];
}

export function isSelected(
  date: string,
  start: string | null,
  end: string | null,
): boolean {
  if (!start || !end) return false;
  const lo = start <= end ? start : end;
  const hi = start <= end ? end : start;
  return date >= lo && date <= hi;
}

export function filterCommits(
  filter: CommitFilterResource,
  commits: RepositoryCommitResource[],
  start: string | null = null,
  end: string | null = null,
): RepositoryCommitResource[] {
  return commits.filter((commit) => filterCommit(filter, commit, start, end));
}

function filterCommit(
  filter: CommitFilterResource,
  commit: RepositoryCommitResource,
  start: string | null,
  end: string | null,
): boolean {
  if (!isSelected(commit.date.slice(0, 10), start, end) && start && end)
    return false;

  if (filter.authors && filter.authors.length > 0) {
    const match = filter.authors.some(
      (a) => commit.author.name === a || commit.author.email === a,
    );
    if (!match) return false;
  }

  if (filter.tags && filter.tags.length > 0) {
    const match = filter.tags.some((tag) => commit.message.includes(tag));
    if (!match) return false;
  }

  const { included_paths, excluded_paths } = filter;
  if (included_paths && included_paths.length > 0) {
    const includeRegexes = included_paths.map((p) => new RegExp(p));
    const hasInclude = commit.diffs.some((diff) =>
      includeRegexes.some((re) => re.test(diff.path)),
    );
    if (!hasInclude) return false;
  }
  if (excluded_paths && excluded_paths.length > 0) {
    const excludeRegexes = excluded_paths.map((p) => new RegExp(p));
    const hasExclude = commit.diffs.some((diff) =>
      excludeRegexes.some((re) => re.test(diff.path)),
    );
    if (hasExclude) return false;
  }

  return true;
}
