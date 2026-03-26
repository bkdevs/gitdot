import type {
  RepositoryBlobResource,
  RepositoryPathsResource,
} from "gitdot-api";
import type { Root } from "hast";
import { Suspense } from "react";
import {
  getRepositoryBlob,
  getRepositoryFileCommits,
  getRepositoryPaths,
} from "@/dal";
import { fetchResources } from "@/provider/server";
import { Loading } from "@/ui/loading";
import { PageClient } from "./page.client";
import { FileHistoryLoader } from "./ui/file-history-loader";
import { FileViewer } from "./ui/file-viewer";
import { FolderViewer } from "./ui/folder-viewer";
import { expandPaths, parseLineSelection } from "./util";

export type Resources = {
  blob: RepositoryBlobResource | null;
  hast: Root | null;
  paths: RepositoryPathsResource | null;
};

export default async function Page({
  params,
  searchParams,
}: {
  params: Promise<{ owner: string; repo: string; path: string[] }>;
  searchParams: Promise<{
    lines?: string | string[];
    ref?: string;
  }>;
}) {
  const { owner, repo, path } = await params;
  const { lines, ref } = await searchParams;

  const filePathString = decodeURIComponent(path.join("/"));
  const selectedLines = parseLineSelection(lines);

  // TODO: add ref here + fix file history commits
  const { requests, promises } = fetchResources(owner, repo, {
    blob: (p) => p.getBlob(filePathString),
    hast: (p) => p.getHast(filePathString),
    paths: (p) => p.getPaths(),
  });

  if (ref) {
    // Unchanged: existing behavior for historical ref views
    const [blob, paths] = await Promise.all([
      getRepositoryBlob(owner, repo, { path: filePathString, ref_name: ref }),
      getRepositoryPaths(owner, repo),
    ]);
    if (!blob) return <div>File not found.</div>;
    if (blob.type === "folder") {
      const initialExpanded = paths ? expandPaths(blob.path, paths, 1) : undefined;
      return <FolderViewer path={blob.path} paths={paths} initialExpanded={initialExpanded} />;
    }
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
