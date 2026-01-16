import type { RepositoryFileDiff } from "@/lib/dto";
import { DiffHeader } from "./diff-header";
import { DiffSingle } from "./diff-single";
import { DiffSplit } from "./diff-split";

export async function DiffFile({ diff }: { diff: RepositoryFileDiff }) {
  const { left, right, lines_added, lines_removed, hunks } = diff;

  return (
    <div className="flex flex-col w-full">
      <DiffHeader
        leftPath={left?.path}
        rightPath={right?.path}
        linesAdded={diff.lines_added}
        linesRemoved={diff.lines_removed}
      />

      {left && !right && <DiffSingle file={left} side="left" />}
      {!left && right && <DiffSingle file={right} side="right" />}
      {left && right && <DiffSplit left={left} right={right} hunks={hunks} />}
    </div>
  );
}
