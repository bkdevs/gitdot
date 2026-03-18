"use client";

import { useParams } from "next/navigation";
import { Suspense, use } from "react";
import { useRepoContext } from "@/(main)/[owner]/[repo]/context";
import { FolderViewer } from "../[...filePath]/ui/folder-viewer";
import { getFolderEntries } from "../util";

function FilesClient() {
  const { owner, repo } = useParams<{ owner: string; repo: string }>();

  const paths = use(useRepoContext().paths);
  if (!paths) return null;

  const entries = getFolderEntries("", paths);
  return <FolderViewer owner={owner} repo={repo} entries={entries} />;
}

export default function Page() {
  return (
    <Suspense>
      <FilesClient />
    </Suspense>
  );
}
