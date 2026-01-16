import { getRepositoryCommitDiffs } from "@/lib/dal";
import { CommitHeader } from "./ui/commit-header";
import { DiffFile } from "./ui/diff-file";

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
      <div className="flex flex-col gap-8">
        {diffs.map((diff) => (
          <DiffFile key={diff.left?.path || diff.right?.path} diff={diff} />
        ))}
      </div>
    </div>
  );
}
