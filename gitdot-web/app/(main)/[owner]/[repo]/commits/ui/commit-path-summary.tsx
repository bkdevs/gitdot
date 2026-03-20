import type { RepositoryDiffStatResource } from "gitdot-api";
import { pluralize } from "@/util";
import { DiffStatBar } from "../[sha]/ui/diff-stat-bar";

function topPaths(
  diffs: RepositoryDiffStatResource[],
  n = 3,
): Array<{ path: string; added: number; removed: number }> {
  const groups = new Map<string, { added: number; removed: number }>();
  for (const diff of diffs) {
    const segments = diff.path.split("/");
    const key = segments.slice(0, -1).slice(0, 2).join("/") || ".";
    const g = groups.get(key) ?? { added: 0, removed: 0 };
    g.added += diff.lines_added;
    g.removed += diff.lines_removed;
    groups.set(key, g);
  }
  return Array.from(groups.entries())
    .sort((a, b) => (b[1].added + b[1].removed) - (a[1].added + a[1].removed))
    .slice(0, n)
    .map(([path, { added, removed }]) => ({ path, added, removed }));
}

export function CommitPathSummary({
  diffs,
  totalFiles,
}: {
  diffs: RepositoryDiffStatResource[];
  totalFiles: number;
}) {
  const paths = topPaths(diffs);
  if (paths.length === 0) return null;

  return (
    <div className="grid grid-cols-[auto_12rem] gap-x-2 gap-y-0.5 text-xs text-muted-foreground font-mono">
      {paths.map(({ path, added, removed }, i) => (
        <>
          <span key={`label-${path}`} className="shrink-0">
            {i === 0 ? `${pluralize(totalFiles, "file")}:` : ""}
          </span>
          <div key={path} className="flex justify-between gap-2 min-w-0">
            <span className="truncate">{path}</span>
            <span className="shrink-0"><DiffStatBar added={added} removed={removed} linesPerBar={10} /></span>
          </div>
        </>
      ))}
    </div>
  );
}
