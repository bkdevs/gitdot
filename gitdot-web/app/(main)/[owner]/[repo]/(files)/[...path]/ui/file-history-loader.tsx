import type { RepositoryCommitsResource } from "gitdot-api";
import { FileCommits } from "./file-commits";

export async function FileHistoryLoader({
  commitsPromise,
  selectedCommit,
}: {
  commitsPromise: Promise<RepositoryCommitsResource | null>;
  selectedCommit?: string;
}) {
  const commits = await commitsPromise;
  if (!commits) return <div className="w-64 border-l" />;
  const selectedCommitSha =
    selectedCommit ?? commits.commits[0]?.sha.substring(0, 7) ?? "";
  return (
    <FileCommits commits={commits} selectedCommitSha={selectedCommitSha} />
  );
}
