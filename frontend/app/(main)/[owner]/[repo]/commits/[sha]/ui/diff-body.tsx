import { mergeHunks } from "@/(main)/[owner]/[repo]/util";
import type { RepositoryCommitDiff } from "@/lib/dto";
import { DiffSingle } from "./diff-single";
import { DiffSplit } from "./diff-split";

export async function DiffBody({ diff }: { diff: RepositoryCommitDiff }) {
  const { left, right, diff: fileDiff } = diff;

  const processedHunks = mergeHunks(fileDiff.hunks);

  const renderDiff = () => {
    if (left && right && fileDiff.hunks.length > 0) {
      return <DiffSplit left={left} right={right} hunks={processedHunks} />;
    } else if (left && !right) {
      return <DiffSingle file={left} side="left" />;
    } else if (!left && right) {
      return <DiffSingle file={right} side="right" />;
    } else {
      return <div className="text-sm font-mono px-2">No changes made</div>;
    }
  };

  return <div className="w-full border-b border-border">{renderDiff()}</div>;
}
