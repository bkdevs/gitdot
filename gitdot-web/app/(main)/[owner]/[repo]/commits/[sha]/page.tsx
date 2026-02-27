import { getRepositoryCommit, getRepositoryCommitStat } from "@/dal";
import { CommitBody } from "./ui/commit-body";
import { CommitHeader } from "./ui/commit-header";

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; repo: string; sha: string }>;
}) {
  const { owner, repo, sha } = await params;
  const [commit, stats] = await Promise.all([
    getRepositoryCommit(owner, repo, sha),
    getRepositoryCommitStat(owner, repo, sha),
  ]);
  if (!commit || !stats) return null;

  // a heuristic, use suspense if either more than 100 modified lines or more than 5 files in the diff
  const useSuspense =
    stats
      .map((stat) => stat.lines_added + stat.lines_removed)
      .reduce((acc, curr) => acc + curr, 0) > 100 || stats.length > 5;

  return (
    <div className="flex flex-col w-full">
      <CommitHeader commit={commit} stats={stats} />
      <CommitBody
        owner={owner}
        repo={repo}
        sha={sha}
        useSuspense={useSuspense}
      />
    </div>
  );
}
