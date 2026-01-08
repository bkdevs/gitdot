import { getRepositoryCommits, getRepositoryFile } from "@/lib/dal";
import type { LineSelection } from "../util";
import { FileBody } from "./file-body";
import { FileCommits } from "./file-commits";
import { FileHeader } from "./file-header";

export async function FileViewer({
  repo,
  filePath,
  selectedLines,
}: {
  repo: string;
  filePath: string;
  selectedLines: LineSelection | null;
}) {
  const file = await getRepositoryFile("bkdevs", repo, { path: filePath });
  if (!file) {
    return <div>Failed to fetch file.</div>;
  }

  const commits = await getRepositoryCommits("bkdevs", repo);
  const mostRecentCommit = commits?.commits.find(
    (commit) => commit.sha === file.commit_sha,
  );
  if (!mostRecentCommit) {
    return <div>Commit not found.</div>;
  }

  return (
    <div className="flex flex-col w-full h-screen">
      <FileHeader repo={repo} filePath={filePath} />
      <div className="flex-1 overflow-hidden flex">
        <div className="flex-1 min-w-0">
          <FileBody file={file} selectedLines={selectedLines} />
        </div>
        <FileCommits commits={[mostRecentCommit]} />
      </div>
    </div>
  );
}
