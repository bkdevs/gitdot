"use client";

import { use } from "react";
import { useRepoContext } from "../../context";
import type { LineSelection } from "../util";
import { FileBody } from "./file-body";
import { FileViewerShortcuts } from "./file-viewer-shortcuts";
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
  const { blobs, hasts } = useRepoContext();
  const resolved = use(blobs);
  const blob = resolved?.blobs.find((b) => b.path === path) ?? null;

  if (!blob) {
    return <div>File not found.</div>;
  } else if (blob.type === "folder") {
    return <FolderViewer owner={owner} repo={repo} entries={blob.entries} />;
  }

  const hast = use(hasts).get(path);
  if (!hast) return null;

  return (
    <div className="flex w-full h-full min-h-0 overflow-hidden">
      <div data-page-scroll className="flex-1 min-w-0 overflow-auto scrollbar-thin">
        <FileBody selectedLines={selectedLines} hast={hast} />
      </div>
      {historySlot}
      <FileViewerShortcuts />
    </div>
  );
}
