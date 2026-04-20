import { use } from "react";
import type { DiffEntry } from "@/actions";
import { DiffFile } from "./diff-file";

export function CommitBody({
  diffEntriesPromise,
}: {
  diffEntriesPromise: Promise<DiffEntry[]>;
}) {
  const entries = use(diffEntriesPromise);

  return (
    <div className="flex flex-col">
      {entries.map((entry) => (
        <DiffFile key={entry.resource.path} entry={entry} />
      ))}
    </div>
  );
}
