import type { DiffEntry } from "gitdot-dal/client";
import { CommitDiffFile } from "./commit-diff-file";

export function CommitBody({ entries }: { entries: DiffEntry[] }) {
  return (
    <div className="flex flex-col gap-4">
      {entries.map((entry) => (
        <CommitDiffFile key={entry.resource.path} entry={entry} />
      ))}
    </div>
  );
}
