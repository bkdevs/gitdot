import type { RepositoryCommit, RepositoryFileDiff } from "@/lib/dto";
import { formatDateTime } from "@/util";
import { DiffStatBar } from "./diff-stat-bar";

export function CommitHeader({
  commit,
  diffs,
}: {
  commit: RepositoryCommit;
  diffs: RepositoryFileDiff[];
}) {
  const midpoint = Math.ceil(diffs.length / 2);
  const leftColumn = diffs.slice(0, midpoint);
  const rightColumn = diffs.slice(midpoint);

  const renderDiffItem = (diff: RepositoryFileDiff) => {
    const path = diff.left?.path || diff.right?.path || "";
    return (
      <li key={path} className="font-mono text-sm flex items-center">
        <a href={`#${path}`} className="truncate flex-1 mr-2 hover:underline">
          {path}
        </a>
        <span className="text-muted-foreground w-6 text-right mr-1.5 select-none shrink-0">
          {diff.lines_added + diff.lines_removed}
        </span>
        <DiffStatBar added={diff.lines_added} removed={diff.lines_removed} />
      </li>
    );
  };

  return (
    <div className="shrink-0 border-border border-b p-2">
      <div className="mb-4">
        <div className="flex items-center gap-1 text-xs text-muted-foreground mb-1">
          <span>{commit.author.name}</span>
          <span>•</span>
          <span>{formatDateTime(new Date(commit.date))}</span>
        </div>
        <div className="text-sm text-primary">{commit.message}</div>
      </div>
      <p className="font-mono text-xs text-muted-foreground h-4 mb-1 select-none">
        {diffs.length} files changed
      </p>
      <div className="flex flex-row w-full">
        <ul className="w-1/2 pr-4">{leftColumn.map(renderDiffItem)}</ul>
        <div className="border-l border-border" />
        <ul className="w-1/2 pl-4">{rightColumn.map(renderDiffItem)}</ul>
      </div>
    </div>
  );
}
