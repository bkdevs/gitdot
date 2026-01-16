import type { RepositoryFileDiff } from "@/lib/dto";
import { DiffHeader } from "./diff-header";
import { DiffSingle } from "./diff-single";
import { DiffSplit } from "./diff-split";

export async function DiffFile({ diff }: { diff: RepositoryFileDiff }) {
  const { left, right, hunks } = diff;
  const path = left?.path || right?.path;
  if (!path) return null;

  return (
    <div className="flex flex-col w-full">
      <DiffHeader path={path} />
      {left && !right && <DiffSingle file={left} side="left" />}
      {!left && right && <DiffSingle file={right} side="right" />}
      {left && right && <DiffSplit left={left} right={right} hunks={hunks} />}
    </div>
  );
}
