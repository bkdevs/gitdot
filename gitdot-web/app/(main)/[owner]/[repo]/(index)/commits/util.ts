import type {
  CommitFilterResource,
  RepositoryCommitResource,
  RepositoryDiffStatResource,
} from "gitdot-api";
import { addDays, dateOnly, subtractDays } from "@/util";

// ---------------------------------------------------------------------------
// commit filtering utils
// ---------------------------------------------------------------------------
export function filterCommits(
  filter: CommitFilterResource,
  commits: RepositoryCommitResource[],
): RepositoryCommitResource[] {
  return commits.filter((commit) => filterCommit(filter, commit));
}

function filterCommit(
  filter: CommitFilterResource,
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

// ---------------------------------------------------------------------------
// commits-grid utils
// ---------------------------------------------------------------------------
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
