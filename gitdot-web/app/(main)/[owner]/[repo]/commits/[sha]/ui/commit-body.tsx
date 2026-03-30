import { use } from "react";
import type { DiffEntry } from "@/actions";
import { DiffFile } from "./diff-file";

export function CommitBody({
  diffPromise,
}: {
  diffPromise: Promise<DiffEntry[]>;
}) {
  const entries = use(diffPromise);

  return (
    <div className="flex flex-col">
      {entries.map(({ diff, data }) => (
        <DiffFile key={diff.path} diff={diff} data={data} />
      ))}
    </div>
  );
}
