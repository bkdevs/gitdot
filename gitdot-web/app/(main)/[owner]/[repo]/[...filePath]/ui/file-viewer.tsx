import type {
  RepositoryCommitsResource,
  RepositoryFileResource,
} from "gitdot-api";
import { fileToHast, inferLanguage } from "@/(main)/[owner]/[repo]/util";
import type { LineSelection } from "../util";
import { FileBody } from "./file-body";
import { FileCommits } from "./file-commits";

export async function FileViewer({
  file,
  commits,
  selectedLines,
  selectedCommit,
}: {
  file: RepositoryFileResource;
  commits: RepositoryCommitsResource | null;
  selectedLines: LineSelection | null;
  selectedCommit?: string;
}) {
  if (!commits) {
    return <div>Failed to fetch file.</div>;
  }

  const hast = await fileToHast(
    file.content,
    inferLanguage(file.path),
    "vitesse-light",
    [],
  );

  return (
    <div className="flex w-full h-full min-h-0 overflow-hidden">
      <div className="flex-1 min-w-0 overflow-auto scrollbar-thin">
        <FileBody selectedLines={selectedLines} hast={hast} />
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
