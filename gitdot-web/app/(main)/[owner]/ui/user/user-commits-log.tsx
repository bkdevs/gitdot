"use client";

import type { UserCommitResource } from "gitdot-api";
import { inRange } from "@/util/date";

export function UserCommitsLog({
  commits,
  startDate,
  endDate,
  selectedMonth,
}: {
  commits: Map<string, UserCommitResource[]>;
  startDate: string;
  endDate: string;
  selectedMonth: string | null;
}) {
  const inRangeDays = [...commits.entries()]
    .filter(([date]) => inRange(date, startDate, endDate))
    .sort((a, b) => a[0].localeCompare(b[0]));

  const visibleMonth =
    selectedMonth ??
    inRangeDays[inRangeDays.length - 1]?.[0].slice(0, 7) ??
    null;

  const visibleDays = inRangeDays
    .filter(([date]) => visibleMonth !== null && date.startsWith(visibleMonth))
    .sort((a, b) =>
      selectedMonth ? a[0].localeCompare(b[0]) : b[0].localeCompare(a[0]),
    )
    .map(([date, cs]) => ({ date, commits: cs }));

  return (
    <div className="flex flex-col gap-8 mt-6 px-3">
      {visibleDays.map(({ date, commits: dayCommits }) => (
        <div key={date}>
          <p className="text-xs text-muted-foreground font-mono mb-1">
            {new Date(`${date}T00:00:00`).toLocaleDateString("en-US", {
              month: "long",
              day: "numeric",
            })}
          </p>
          {dayCommits.length === 0 ? (
            <p className="text-xs text-muted-foreground/50 font-mono">—</p>
          ) : (
            <div className="flex flex-col">
              {dayCommits.map((c) => (
                <CommitLogRow key={c.id} c={c} />
              ))}
            </div>
          )}
        </div>
      ))}
    </div>
  );
}

function CommitLogRow({ c }: { c: UserCommitResource }) {
  if (c.redacted) {
    return (
      <div className="flex items-center gap-2">
        <span className="text-xs font-mono text-muted-foreground/50 shrink-0">
          private
        </span>
        <span className="text-sm flex-1 truncate text-muted-foreground/50 italic">
          commit in a private repository
        </span>
      </div>
    );
  }

  const added = c.diffs.reduce((s, d) => s + d.lines_added, 0);
  const removed = c.diffs.reduce((s, d) => s + d.lines_removed, 0);
  const url = `/${c.owner_name}/${c.repo_name}/commits/${c.sha?.slice(0, 7)}`;

  return (
    <a
      href={url}
      target="_blank"
      rel="noopener noreferrer"
      className="group flex items-center gap-2"
    >
      <span className="text-xs font-mono text-muted-foreground shrink-0">
        {c.repo_name}
      </span>
      <span className="text-sm flex-1 truncate underline decoration-transparent group-hover:decoration-current">
        {c.message}
      </span>
      <span className="text-xs font-mono text-muted-foreground/50 shrink-0">
        {c.diffs.length} files
      </span>
      <span className="text-xs font-mono text-green-600 dark:text-green-500 shrink-0">
        +{added}
      </span>
      <span className="text-xs font-mono text-red-600 dark:text-red-500 shrink-0">
        -{removed}
      </span>
    </a>
  );
}
