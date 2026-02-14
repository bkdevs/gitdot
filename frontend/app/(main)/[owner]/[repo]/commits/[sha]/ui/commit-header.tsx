import type { RepositoryCommit, RepositoryCommitStat } from "@/lib/dto";
import { formatDateTime } from "@/util";
import { DiffStatBar } from "./diff-stat-bar";

export function CommitHeader({
  commit,
  stats,
}: {
  commit: RepositoryCommit;
  stats: RepositoryCommitStat[];
}) {
  const midpoint = Math.ceil(stats.length / 2);
  const leftColumn = stats.slice(0, midpoint);
  const rightColumn = stats.slice(midpoint);
  const author = commit.author.name;

  const renderStatItem = (stat: RepositoryCommitStat) => {
    return (
      <li key={stat.path} className="font-mono text-sm flex items-center">
        <a
          href={`#${stat.path}`}
          className="truncate flex-1 mr-2 hover:underline"
        >
          {stat.path}
        </a>
        <span className="text-muted-foreground w-6 text-right mr-1.5 select-none shrink-0">
          {stat.lines_added + stat.lines_removed}
        </span>
        <DiffStatBar added={stat.lines_added} removed={stat.lines_removed} />
      </li>
    );
  };

  return (
    <div className="shrink-0 border-border border-b p-2">
      <div className="mb-4">
        <div className="flex items-center gap-1 text-xs text-muted-foreground mb-1">
          <span>{author}</span>
          <span>•</span>
          <span>{formatDateTime(new Date(commit.date))}</span>
        </div>
        <div className="text-sm text-primary">{commit.message}</div>
      </div>
      <p className="font-mono text-xs text-muted-foreground h-4 mb-1 select-none">
        {stats.length} files changed
      </p>
      <div className="flex flex-row w-full">
        <ul className="w-1/2 pr-4">{leftColumn.map(renderStatItem)}</ul>
        <div className="border-l border-border" />
        <ul className="w-1/2 pl-4">{rightColumn.map(renderStatItem)}</ul>
      </div>
    </div>
  );
}
