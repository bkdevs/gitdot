"use client";

import { FolderViewer } from "../../[...filePath]/ui/folder-viewer";
import { useRepoContext } from "../../context";
import { getFolderEntries, parseRepositoryTree } from "../../util";

export function FilesClient({ owner, repo }: { owner: string; repo: string }) {
  const { tree } = useRepoContext();
  const { folders, entries } = parseRepositoryTree(tree);
  const rootEntries = getFolderEntries("", folders, entries);

  return <FolderViewer owner={owner} repo={repo} folderEntries={rootEntries} />;
}
