import { getRepositoryCommitDiffs } from "@/lib/dal";
import { CommitHeader } from "../ui/commit-header";
import { FileDiff } from "../ui/file-diff";

export default async function Page({
  params,
}: {
  params: Promise<{ slug: string; sha: string }>;
}) {
  const { slug: repo, sha } = await params;
  const commitDiffs = await getRepositoryCommitDiffs("bkdevs", repo, sha);
  if (!commitDiffs) return null;
  const { commit, diffs } = commitDiffs;

  return (
    <div className="flex flex-col w-full h-screen overflow-y-auto scrollbar-thin">
      <CommitHeader commit={commit} diffs={diffs} />
      {/* TODO: consider suspense here? await seems cpu blocking.. */}
      {diffs.slice(2, 3).map((diff) => (
        <FileDiff key={diff.left?.path || diff.right?.path} diff={diff} />
      ))}
    </div>
  );
}
