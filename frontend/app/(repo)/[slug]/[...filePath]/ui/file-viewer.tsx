import { getRepositoryFile, getRepositoryFileCommits } from "@/lib/dal";
import type { LineSelection } from "../util";
import { FileBody } from "./file-body";
import { FileCommits } from "./file-commits";
import { FileHeader } from "./file-header";

export async function FileViewer({
  repo,
  filePath,
  selectedLines,
  selectedCommit,
}: {
  repo: string;
  filePath: string;
  selectedLines: LineSelection | null;
  selectedCommit?: string;
}) {
  const [commits, file] = await Promise.all([
    getRepositoryFileCommits("bkdevs", repo, { path: filePath, }),
    getRepositoryFile("bkdevs", repo, { path: filePath, ref_name: selectedCommit }),
  ]);

  if (!commits || !file) {
    return <div>Failed to fetch file.</div>;
  }

  return (
    <div className="flex flex-col w-full h-screen">
      <FileHeader repo={repo} filePath={file.path} />
      <div className="flex-1 overflow-hidden flex">
        <div className="flex-1 min-w-0">
          <FileBody file={file} selectedLines={selectedLines} />
        </div>
        <FileCommits
          commits={commits}
          selectedCommitSha={selectedCommit || commits.commits[0].sha.substring(0, 7)} />
      </div>
    </div>
  );
}
