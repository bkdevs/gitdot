import { use } from "react";
import type { DiffEntry } from "@/actions";
import { DiffBody } from "./diff-body";

export function CommitBody({
  diffPromise,
}: {
  diffPromise: Promise<DiffEntry[]>;
}) {
  const entries = use(diffPromise);

  return (
    <div className="flex flex-col">
      {entries.map(({ diff, data }) => (
        <DiffBody key={diff.path} diff={diff} data={data} />
      ))}
    </div>
  );
}
