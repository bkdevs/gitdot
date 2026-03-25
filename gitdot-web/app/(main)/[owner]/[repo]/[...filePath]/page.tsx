import type { RepositoryBlobResource } from "gitdot-api";
import type { Root } from "hast";
import { Suspense } from "react";
import { getRepositoryBlob, getRepositoryFileCommits } from "@/dal";
import { fetchResources } from "@/provider/server";
import { Loading } from "@/ui/loading";
import { PageClient } from "./page.client";
import { FileHistoryLoader } from "./ui/file-history-loader";
import { FileViewer } from "./ui/file-viewer";
import { FolderViewer } from "./ui/folder-viewer";
import { parseLineSelection } from "./util";

export type Resources = {
  blob: RepositoryBlobResource | null;
  hast: Root | null;
  readme: RepositoryBlobResource | null;
};

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

  // TODO: add ref here + fix file history commits
  const { requests, promises } = fetchResources(owner, repo, {
    blob: (p) => p.getBlob(filePathString),
    hast: (p) => p.getHast(filePathString),
    readme: (p) => p.getBlob(`${filePathString}/README.md`),
  });

  if (ref) {
    // Unchanged: existing behavior for historical ref views
    const blob = await getRepositoryBlob(owner, repo, {
      path: filePathString,
      ref_name: ref,
    });
    if (!blob) return <div>File not found.</div>;
    if (blob.type === "folder")
      return <FolderViewer />;
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

  // Default ref: use resources, but fetch file history manually still
  const commitsPromise = getRepositoryFileCommits(owner, repo, {
    path: filePathString,
  });

  return (
    <Suspense fallback={<Loading />}>
      <PageClient
        owner={owner}
        repo={repo}
        selectedLines={selectedLines}
        historySlot={
          <Suspense fallback={<div className="w-64 border-l" />}>
            <FileHistoryLoader commitsPromise={commitsPromise} />
          </Suspense>
        }
        requests={requests}
        promises={promises}
      />
    </Suspense>
  );
}
