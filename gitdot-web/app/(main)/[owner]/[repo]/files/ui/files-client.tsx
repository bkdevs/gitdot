"use client";

import { useRepoResource } from "@/(main)/[owner]/[repo]/context";
import { FolderViewer } from "../../[...filePath]/ui/folder-viewer";
import { getFolderEntries } from "../../util";

export function FilesClient({ owner, repo }: { owner: string; repo: string }) {
  const paths = useRepoResource("paths");
  const entries = getFolderEntries("", paths);

  return <FolderViewer owner={owner} repo={repo} entries={entries} />;
}
