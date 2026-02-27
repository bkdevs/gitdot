import { getRepositoryFile, getRepositoryFileCommits, NotFound } from "@/dal";
import type { LineSelection } from "../util";
import { FileBody } from "./file-body";
import { FileCommits } from "./file-commits";

export async function FileViewer({
  owner,
  repo,
  filePath,
  selectedLines,
  selectedCommit,
}: {
  owner: string;
  repo: string;
  filePath: string;
  selectedLines: LineSelection | null;
  selectedCommit?: string;
}) {
  const [commits, file] = await Promise.all([
    getRepositoryFileCommits(owner, repo, { path: filePath }),
    getRepositoryFile(owner, repo, {
      path: filePath,
      ref_name: selectedCommit,
    }),
  ]);

  if (!commits || !file || file === NotFound) {
    return <div>Failed to fetch file.</div>;
  }

  return (
    <div className="flex w-full h-full min-h-0 overflow-hidden">
      <div className="flex-1 min-w-0 overflow-auto scrollbar-thin">
        <FileBody file={file} selectedLines={selectedLines} />
      </div>
      <FileCommits
        commits={commits}
        selectedCommitSha={
          selectedCommit || commits.commits[0].sha.substring(0, 7)
        }
      />
    </div>
  );
}
