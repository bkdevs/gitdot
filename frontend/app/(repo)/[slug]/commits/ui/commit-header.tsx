import type { RepositoryCommit, RepositoryFileDiff } from "@/lib/dto";
import { formatDateTime } from "@/util";

function DiffStatBar({ added, removed }: { added: number; removed: number }) {
  const total = added + removed;
  if (total === 0) return null;

  const linesPerBar = 5;
  const hasAdded = added > 0;
  const hasRemoved = removed > 0;
  const minBars = hasAdded && hasRemoved ? 2 : 1;
  const barCount = Math.max(
    minBars,
    Math.min(Math.ceil(total / linesPerBar), 10),
  );

  let addedBars = Math.round((added / total) * barCount);
  if (hasAdded && addedBars < 1) addedBars = 1;
  if (hasRemoved && addedBars > barCount - 1) addedBars = barCount - 1;
  const removedBars = barCount - addedBars;

  return (
    <span className="font-mono">
      <span className="text-green-600">{"+".repeat(addedBars)}</span>
      <span className="text-red-600">{"-".repeat(removedBars)}</span>
    </span>
  );
}

export default function CommitHeader({
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
    <div className="h-59.25 border-border border-b">
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

        <div className="flex flex-col w-2/3 h-full p-2 overflow-y-auto scrollbar-thin">
          <p className="font-mono text-xs text-muted-foreground h-4 mb-1">
            {diffs.length} files changed
          </p>
          <ul className="">
            {diffs.map((diff) => {
              const path = diff.left?.path || diff.right?.path || "";
              return (
                <li
                  key={diff.left?.sha || diff.right?.sha}
                  className="font-mono text-sm flex items-center"
                >
                  <span
                    className="inline-block border-r border-border pr-3 mr-1 box-content"
                    style={{ width: `${maxPathLength}ch` }}
                  >
                    {path}
                  </span>
                  <span className="text-muted-foreground w-6 text-right mr-1.5">
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
