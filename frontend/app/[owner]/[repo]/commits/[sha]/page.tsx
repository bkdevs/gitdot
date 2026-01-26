import { getRepositoryCommitStats } from "@/lib/dal";
import { CommitBody } from "./ui/commit-body";
import { CommitHeader } from "./ui/commit-header";

export default async function Page({
  params,
}: {
  params: Promise<{ repo: string; sha: string }>;
}) {
  const { repo, sha } = await params;
  const commitStats = await getRepositoryCommitStats("bkdevs", repo, sha);
  if (!commitStats) return null;

  // a heuristic, use suspense if either more than 100 modified lines or more than 5 files in the diff
  const useSuspense =
    commitStats.diffs
      .map((diff) => diff.lines_added + diff.lines_removed)
      .reduce((acc, curr) => acc + curr, 0) > 100 ||
    commitStats.diffs.length > 5;

  return (
    <div className="flex flex-col w-full h-screen overflow-y-auto scrollbar-thin">
      <CommitHeader commit={commitStats.commit} diffs={commitStats.diffs} />
      <CommitBody repo={repo} sha={sha} useSuspense={useSuspense} />
    </div>
  );
}
