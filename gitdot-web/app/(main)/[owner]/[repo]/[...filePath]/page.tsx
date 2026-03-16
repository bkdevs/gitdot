import { Suspense } from "react";
import { getRepositoryBlob, getRepositoryFileCommits } from "@/dal";
import { FileBlobClient } from "./ui/file-blob-client";
import { FileHistoryLoader } from "./ui/file-history-loader";
import { FileViewer } from "./ui/file-viewer";
import { FolderViewer } from "./ui/folder-viewer";
import { parseLineSelection } from "./util";

export default async function Page({
  params,
  searchParams,
}: {
  params: Promise<{ owner: string; repo: string; filePath: string[] }>;
  searchParams: Promise<{
    lines?: string | string[];
    ref?: string;
  }>;
}) {
  const { owner, repo, filePath } = await params;
  const { lines, ref } = await searchParams;

  const filePathString = decodeURIComponent(filePath.join("/"));
  const selectedLines = parseLineSelection(lines);

  if (ref) {
    // Unchanged: existing behavior for historical ref views
    const blob = await getRepositoryBlob(owner, repo, {
      path: filePathString,
      ref_name: ref,
    });
    if (!blob) return <div>File not found.</div>;
    if (blob.type === "folder")
      return <FolderViewer owner={owner} repo={repo} entries={blob.entries} />;
    const commits = await getRepositoryFileCommits(owner, repo, {
      path: filePathString,
      ref_name: ref,
    });
    return (
      <FileViewer
        file={blob}
        commits={commits}
        selectedLines={selectedLines}
        selectedCommit={ref}
      />
    );
  }

  // Default ref: use context blobs, history in Suspense
  const commitsPromise = getRepositoryFileCommits(owner, repo, {
    path: filePathString,
  });

  return (
    <Suspense fallback={<div>Loading...</div>}>
      <FileBlobClient
        owner={owner}
        repo={repo}
        path={filePathString}
        selectedLines={selectedLines}
        historySlot={
          <Suspense fallback={<div className="w-64 border-l" />}>
            <FileHistoryLoader commitsPromise={commitsPromise} />
          </Suspense>
        }
      />
    </Suspense>
  );
}
