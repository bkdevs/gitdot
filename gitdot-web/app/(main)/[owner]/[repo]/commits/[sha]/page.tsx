import { getRepositoryCommit } from "@/dal";
import { CommitBody } from "./ui/commit-body";
import { CommitHeader } from "./ui/commit-header";

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; repo: string; sha: string }>;
}) {
  const { owner, repo, sha } = await params;
  const commit = await getRepositoryCommit(owner, repo, sha);
  if (!commit) return null;

  const diffs = commit.diffs;
  console.log(commit);

  // a heuristic, use suspense if either more than 100 modified lines or more than 5 files in the diff
  const useSuspense =
    diffs
      .map((stat) => stat.lines_added + stat.lines_removed)
      .reduce((acc, curr) => acc + curr, 0) > 100 || diffs.length > 5;

  return (
    <div className="flex flex-col w-full">
      <CommitHeader commit={commit} stats={diffs} />
      <CommitBody
        owner={owner}
        repo={repo}
        sha={sha}
        useSuspense={useSuspense}
      />
    </div>
  );
}
