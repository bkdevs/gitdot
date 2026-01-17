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
  const maxPathLength = Math.max(
    ...diffs.map((d) => (d.left?.path || d.right?.path || "").length),
  );

  return (
    // height to match first height of header + three rows in commit history
    <div className="h-59.25 shrink-0">
      <div className="flex flex-row w-full h-full">
        <div className="flex flex-col w-1/3 h-full p-2 border-r border-border">
          <div className="flex items-center gap-1 text-xs text-muted-foreground h-4 mb-1">
            <span>{commit.author}</span>
            <span>•</span>
            <span>{formatDateTime(new Date(commit.date))}</span>
          </div>
          <div className="text-sm text-primary mb-2">
            {commit.message.split("\n")[0]}
          </div>
          <div className="text-sm text-primary flex-1">
            {commit.message.split("\n").slice(1).join("\n")}
          </div>
          <div className="flex items-center justify-between text-xs text-muted-foreground mt-auto pt-2">
            <span className="font-mono">#chore #backend</span>
            <span className="font-mono">{commit.sha.slice(0, 7)}</span>
          </div>
        </div>

        <div className="flex flex-col w-2/3 h-full p-2 overflow-y-auto scrollbar-none">
          <p className="font-mono text-xs text-muted-foreground h-4 mb-1 select-none">
            {diffs.length} files changed
          </p>
          <ul className="">
            {diffs.map((diff) => {
              const path = diff.left?.path || diff.right?.path || "";
              return (
                <li key={path} className="font-mono text-sm flex items-center">
                  <span
                    className="inline-block border-r border-border pr-3 mr-1 box-content"
                    style={{ width: `${maxPathLength}ch` }}
                  >
                    {path}
                  </span>
                  <span className="text-muted-foreground w-6 text-right mr-1.5 select-none">
                    {diff.lines_added + diff.lines_removed}
                  </span>
                  <DiffStatBar
                    added={diff.lines_added}
                    removed={diff.lines_removed}
                  />
                </li>
              );
            })}
          </ul>
        </div>
      </div>
    </div>
  );
}
