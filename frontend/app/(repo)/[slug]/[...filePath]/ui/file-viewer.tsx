import { getRepositoryFileHistory } from "@/lib/dal";
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
  // Fetch file history
  const fileHistory = await getRepositoryFileHistory("bkdevs", repo, {
    path: filePath,
    ref_name: "HEAD",
    page: 1,
    per_page: 100,
  });

  if (!fileHistory) {
    return <div>Failed to fetch file history.</div>;
  }

  // Determine which file version to display
  let file;
  let currentCommitSha: string;

  if (selectedCommit) {
    // Find the historical version
    const historyEntry = fileHistory.history.find(
      (entry) => entry.commit.sha === selectedCommit,
    );

    if (!historyEntry) {
      return <div>Commit not found in file history.</div>;
    }

    file = historyEntry.file;
    currentCommitSha = selectedCommit;
  } else {
    // Use latest version
    const latestEntry = fileHistory.history[0];
    if (!latestEntry) {
      return <div>No commits found for this file.</div>;
    }

    file = latestEntry.file;
    currentCommitSha = latestEntry.commit.sha;
  }

  return (
    <div className="flex flex-col w-full h-screen">
      <FileHeader repo={repo} filePath={file.path} />
      <div className="flex-1 overflow-hidden flex">
        <div className="flex-1 min-w-0">
          <FileBody file={file} selectedLines={selectedLines} />
        </div>
        <FileCommits
          repo={repo}
          filePath={filePath}
          history={fileHistory.history}
          selectedCommitSha={currentCommitSha}
        />
      </div>
    </div>
  );
}
