import type { RepositoryFileDiff } from "@/lib/dto";
import { DiffSingle } from "./diff-single";
import { DiffSplit } from "./diff-split";

export async function DiffBody({ diff }: { diff: RepositoryFileDiff }) {
  const { left, right, hunks } = diff;

  if (left && right && hunks.length > 0) {
    return <DiffSplit left={left} right={right} hunks={hunks} />;
  } else if (left && !right) {
    return <DiffSingle file={left} side="left" />;
  } else if (!left && right) {
    return <DiffSingle file={right} side="right" />;
  } else {
    return <div className="text-sm font-mono px-2">No changes made</div>;
  }
}
