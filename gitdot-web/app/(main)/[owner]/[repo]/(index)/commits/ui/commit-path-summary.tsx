import type { RepositoryDiffStatResource } from "gitdot-api";
import { Fragment } from "react";
import { pluralize } from "@/util";
import { DiffStatBar } from "../../../commits/[sha]/ui/diff-stat-bar";
import { computePrimaryPaths } from "../util";

export function CommitPathSummary({
  diffs,
  totalFiles,
}: {
  diffs: RepositoryDiffStatResource[];
  totalFiles: number;
}) {
  const paths = computePrimaryPaths(diffs);
  if (paths.length === 0) return null;

  return (
    <div className="grid grid-cols-[auto_12.5rem] gap-x-2 gap-y-0.5 text-xs text-muted-foreground font-mono">
      {paths.map(({ path, added, removed }, i) => (
        <Fragment key={path}>
          <span className="shrink-0">
            {i === 0 ? `${pluralize(totalFiles, "file")}:` : ""}
          </span>
          <div className="flex justify-between gap-2 min-w-0">
            <span className="truncate">{path}</span>
            <span className="shrink-0">
              <DiffStatBar added={added} removed={removed} linesPerBar={10} />
            </span>
          </div>
        </Fragment>
      ))}
    </div>
  );
}
