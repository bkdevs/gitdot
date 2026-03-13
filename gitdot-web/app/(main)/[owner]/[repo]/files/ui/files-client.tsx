"use client";

import { useRepoResource } from "@/(main)/[owner]/[repo]/context";
import { getFolderEntries } from "../../util";
import { PathsFolderViewer } from "./paths-folder-viewer";

export function FilesClient({ owner, repo }: { owner: string; repo: string }) {
  const paths = useRepoResource("paths");
  const rootEntries = getFolderEntries("", paths);

  return <PathsFolderViewer owner={owner} repo={repo} entries={rootEntries} />;
}
