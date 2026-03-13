import type { RepositoryDiffResource } from "gitdot-api";
import { mergeHunks } from "@/(main)/[owner]/[repo]/util";
import { getRepositoryBlob } from "@/dal";
import { DiffSingle } from "./diff-single";
import { DiffSplit } from "./diff-split";

export async function DiffBody({
  stat,
  owner,
  repo,
  sha,
  parentSha,
}: {
  stat: RepositoryDiffResource;
  owner: string;
  repo: string;
  sha: string;
  parentSha: string | undefined;
}) {
  const [rightBlob, leftBlob] = await Promise.all([
    getRepositoryBlob(owner, repo, { ref_name: sha, path: stat.path }),
    parentSha
      ? getRepositoryBlob(owner, repo, { ref_name: parentSha, path: stat.path })
      : null,
  ]);

  const right = rightBlob?.type === "file" ? rightBlob : null;
  const left = leftBlob?.type === "file" ? leftBlob : null;

  const processedHunks = mergeHunks(stat.hunks);

  const renderDiff = () => {
    if (left && right && stat.hunks.length > 0) {
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
