import { use } from "react";
import { DiffFile } from "@/(main)/[owner]/[repo]/commits/[sha]/ui/diff-file";
import type { DiffEntry } from "@/actions";

export function ReviewDiffBody({
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
