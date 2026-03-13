"use client";

import type { RepositoryBlobResource } from "gitdot-api";
import { use } from "react";
import { useRepoContext } from "../../context";
import type { LineSelection } from "../util";
import { FileBody } from "./file-body";
import { FolderViewer } from "./folder-viewer";

export function FileBlobClient({
  owner,
  repo,
  path,
  selectedLines,
  historySlot,
}: {
  owner: string;
  repo: string;
  path: string;
  selectedLines: LineSelection | null;
  historySlot: React.ReactNode;
}) {
  const { blobs } = useRepoContext();
  const blob = use(
    blobs.then(
      (r): RepositoryBlobResource | null =>
        r.blobs.find((b) => b.path === path) ?? null,
    ),
  );

  if (!blob) return <div>File not found.</div>;
  if (blob.type === "folder")
    return <FolderViewer owner={owner} repo={repo} entries={blob.entries} />;

  return (
    <div className="flex w-full h-full min-h-0 overflow-hidden">
      <div className="flex-1 min-w-0 overflow-auto scrollbar-thin">
        <FileBody file={blob} selectedLines={selectedLines} />
      </div>
      {historySlot}
    </div>
  );
}
