import type {
  RepositoryCommitFilterResource,
  RepositoryCommitResource,
  RepositoryDiffStatResource,
} from "gitdot-api";
import { addDays, dateOnly, subtractDays } from "@/util";

// ---------------------------------------------------------------------------
// commit filtering utils
// ---------------------------------------------------------------------------
export const ALL_COMMITS_FILTER: RepositoryCommitFilterResource = {
  id: "00000000-0000-0000-0000-000000000000",
  repository_id: "00000000-0000-0000-0000-000000000000",
  name: "All commits",
  created_at: new Date(0).toISOString(),
  updated_at: new Date(0).toISOString(),
};

export function filterCommits(
  filter: RepositoryCommitFilterResource,
  commits: RepositoryCommitResource[],
): RepositoryCommitResource[] {
  return commits.filter((commit) => filterCommit(filter, commit));
}

function sameSet(a: string[] | undefined, b: string[] | undefined): boolean {
  const av = a ?? [];
  const bv = b ?? [];
  if (av.length !== bv.length) return false;
  const set = new Set(av);
  return bv.every((x) => set.has(x));
}

export function isFilterModified(
  active: RepositoryCommitFilterResource,
  original: RepositoryCommitFilterResource,
): boolean {
  return (
    !sameSet(active.authors, original.authors) ||
    !sameSet(active.tags, original.tags) ||
    !sameSet(active.paths, original.paths)
  );
}

function filterCommit(
  filter: RepositoryCommitFilterResource,
  commit: RepositoryCommitResource,
): boolean {
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

  if (filter.paths && filter.paths.length > 0) {
    const regexes = filter.paths.map((p) => new RegExp(p));
    const hasMatch = commit.diffs.some((diff) =>
      regexes.some((re) => re.test(diff.path)),
    );
    if (!hasMatch) return false;
  }

  return true;
}

// ---------------------------------------------------------------------------
// commits-grid utils
// ---------------------------------------------------------------------------
export const NUM_DAYS = 7;
const MS_PER_WEEK = 7 * 86400000;

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

// Date YYYY-MM-DD strings in this module are always in *local* time. Avoid
// `.toISOString().slice(0,10)` — that returns the UTC date, which shifts by a
// day for users in UTC+ timezones and breaks the snap-to-Sunday math.
function toLocalISODate(d: Date): string {
  const y = d.getFullYear();
  const m = String(d.getMonth() + 1).padStart(2, "0");
  const day = String(d.getDate()).padStart(2, "0");
  return `${y}-${m}-${day}`;
}

export function recentWindowEnd(
  commits: RepositoryCommitResource[] | null,
): string {
  const today = new Date();
  const todayStr = toLocalISODate(today);
  const mostRecent = commits?.[0]?.date.slice(0, 10);
  if (!mostRecent) return todayStr;
  const oneYearAgo = toLocalISODate(subtractDays(today, 365));
  return mostRecent >= oneYearAgo ? todayStr : mostRecent;
}

export function recentWindowStart(
  commits: RepositoryCommitResource[] | null,
): string {
  const end = recentWindowEnd(commits);
  // 365 days back, then snap further to the previous Sunday so the leftmost
  // grid column is always a complete Sun→Sat (up to ~53 weeks total).
  const rough = subtractDays(new Date(`${end}T00:00:00`), 365);
  return toLocalISODate(subtractDays(rough, rough.getDay()));
}

export function buildGrid(
  commits: RepositoryCommitResource[],
  windowStart: string,
  windowEnd: string,
): { weeks: Week[]; months: Month[]; numWeeks: number } {
  const countMap = new Map<string, number>();
  for (const commit of commits) {
    const date = commit.date.slice(0, 10); // iso date YYYY-MM-DD
    countMap.set(date, (countMap.get(date) ?? 0) + 1);
  }

  const today = dateOnly(new Date());
  const start = dateOnly(windowStart);
  const end = dateOnly(windowEnd);
  const firstSunday = subtractDays(start, start.getDay());
  const lastSunday = subtractDays(end, end.getDay());
  const numWeeks =
    Math.round((lastSunday.getTime() - firstSunday.getTime()) / MS_PER_WEEK) +
    1;

  const weeks: Week[] = [];
  const months: Month[] = [];
  let prevMonth = -1;

  // col 0 = most recent week (rightmost in the grid), going back to col numWeeks-1.
  for (let col = 0; col < numWeeks; col++) {
    const weekStart: Date = subtractDays(lastSunday, col * 7);
    const week: Day[] = [];

    for (let row = 0; row < NUM_DAYS; row++) {
      const d = addDays(weekStart, row);
      if (d < start || d > end || d > today) continue;
      const dateStr = toLocalISODate(d);
      week.push({
        date: dateStr,
        commitCount: countMap.get(dateStr) ?? 0,
      });
    }
    weeks.push(week);

    // use the earliest visible day in this column for the label, so the
    // leftmost partial week doesn't get labelled with the prior month/year.
    const labelDate = weekStart < start ? start : weekStart;
    if (labelDate.getMonth() !== prevMonth) {
      const isJanuary = labelDate.getMonth() === 0;
      months.push({
        label: isJanuary
          ? String(labelDate.getFullYear())
          : labelDate.toLocaleString("en-US", { month: "short" }),
        startingWeek: col,
        numWeeks: 0,
      });
      prevMonth = labelDate.getMonth();
    }
  }

  for (let i = 0; i < months.length; i++) {
    const next = months[i + 1];
    months[i].numWeeks = next
      ? next.startingWeek - months[i].startingWeek
      : numWeeks - months[i].startingWeek;
  }

  return { weeks, months, numWeeks };
}

export function computeThresholds(counts: number[]): Thresholds {
  const nonZero = counts.filter((c) => c > 0).sort((a, b) => a - b);
  if (nonZero.length === 0) return [1, 2, 3];

  const q = (p: number) => nonZero[Math.floor(p * (nonZero.length - 1))];
  return [q(0.25), q(0.5), q(0.75)];
}

export function cellColor(count: number, thresholds: Thresholds): string {
  const [low, med, high] = thresholds;
  if (count === 0) return "bg-commit-grid-empty";
  if (count <= low) return "bg-commit-grid-low";
  if (count <= med) return "bg-commit-grid-med";
  if (count <= high) return "bg-commit-grid-high";
  return "bg-commit-grid-max";
}

// ---------------------------------------------------------------------------
// commits-list utils
// ---------------------------------------------------------------------------
const MAX_PATH_LEN = 22; // roughly char size from 12.5 rem with text-xs

function truncateFromRoot(path: string, maxLen: number): string {
  const parts = path.split("/");
  while (parts.join("/").length > maxLen && parts.length > 1) {
    parts.shift();
  }
  return parts.join("/");
}

type PathGroup = {
  key: string;
  depth: number;
  diffs: RepositoryDiffStatResource[];
};

/**
 * TODO: claude-coded, revisit logic in depth to make things better
 *
 * summarise `diffs` into up to `n` labelled path groups.
 *
 * Strategy: start with depth-1 grouping, then iteratively split the group
 * with the most files one level deeper until we have `n` distinct groups (or
 * can't split any further). Within each group, a single-file entry shows
 * `parent/filename` when it fits; multi-file entries show the directory key.
 */
export function computePrimaryPaths(
  diffs: RepositoryDiffStatResource[],
  n = 3,
): Array<{ path: string; added: number; removed: number }> {
  if (diffs.length === 0) return [];

  // seed: group every file by its first directory segment.
  const seed = new Map<string, PathGroup>();
  for (const diff of diffs) {
    const key = diff.path.split("/").slice(0, -1).slice(0, 1).join("/") || "./";
    if (!seed.has(key)) seed.set(key, { key, depth: 1, diffs: [] });
    seed.get(key)?.diffs.push(diff);
  }
  let groups = Array.from(seed.values());

  // iteratively split the largest group that still has sub-directories.
  const target = Math.min(n, diffs.length);
  while (groups.length < target) {
    const splittable = groups.filter((g) =>
      g.diffs.some((d) => d.path.split("/").length - 1 > g.depth),
    );
    if (splittable.length === 0) break;

    const toSplit = splittable.reduce((a, b) =>
      b.diffs.length > a.diffs.length ? b : a,
    );
    const newDepth = toSplit.depth + 1;
    const sub = new Map<string, PathGroup>();
    for (const diff of toSplit.diffs) {
      const key =
        diff.path.split("/").slice(0, -1).slice(0, newDepth).join("/") || "./";
      if (!sub.has(key)) sub.set(key, { key, depth: newDepth, diffs: [] });
      sub.get(key)?.diffs.push(diff);
    }
    groups = groups
      .filter((g) => g !== toSplit)
      .concat(Array.from(sub.values()));
  }

  // aggregate and sort by lines changed
  const sorted = groups
    .map((g) => ({
      key: g.key,
      added: g.diffs.reduce((s, d) => s + d.lines_added, 0),
      removed: g.diffs.reduce((s, d) => s + d.lines_removed, 0),
      files: g.diffs.length,
      singlePath: g.diffs.length === 1 ? g.diffs[0].path : undefined,
    }))
    .sort((a, b) => b.added + b.removed - (a.added + a.removed))
    .slice(0, n)
    .map(({ key, added, removed, files, singlePath }) => {
      const truncated =
        key.length <= MAX_PATH_LEN ? key : truncateFromRoot(key, MAX_PATH_LEN);
      return { key: truncated, added, removed, files, singlePath };
    });

  // prettify names, make single files go up to include parent folder if possible
  const candidates = sorted.map(
    ({ key, added, removed, files, singlePath }) => {
      let compact: string | undefined;
      if (files === 1 && singlePath) {
        const parts = singlePath.split("/");
        const filename = parts.at(-1) ?? "";
        const withParent =
          parts.length >= 2 ? `${parts.at(-2)}/${filename}` : filename;
        const c = withParent.length <= MAX_PATH_LEN ? withParent : filename;
        if (c.length <= MAX_PATH_LEN) compact = c;
      }
      return { key, added, removed, compact };
    },
  );

  // deduplicate compact forms — fall back to the dir key on collision.
  const counts = new Map<string, number>();
  for (const { compact } of candidates) {
    if (compact) counts.set(compact, (counts.get(compact) ?? 0) + 1);
  }

  return candidates
    .map(({ key, added, removed, compact }) => {
      const isCompact = compact && counts.get(compact) === 1;
      const dirPath = key.endsWith("/") ? key : `${key}/`;
      return { path: isCompact ? compact : dirPath, added, removed };
    })
    .sort((a, b) => {
      const aIsDir = a.path.endsWith("/");
      const bIsDir = b.path.endsWith("/");
      if (aIsDir !== bIsDir) return aIsDir ? -1 : 1; // dirs first
      // shallower paths first (fewer segments)
      const aDepth = a.path.split("/").length;
      const bDepth = b.path.split("/").length;
      return aDepth - bDepth;
    });
}
