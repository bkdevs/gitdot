"use client";

import { Suspense, use } from "react";
import {
  type ResourcePromisesType,
  type ResourceRequestsType,
  useResolvePromises,
} from "@/(main)/[owner]/[repo]/resources";
import { Loading } from "@/ui/loading";
import type { Resources } from "./page";
import { FileViewer } from "./ui/file-viewer";
import { FileViewerProvider } from "./ui/file-viewer-context";
import { FolderViewer } from "./ui/folder-viewer";
import type { LineSelection } from "./util";

type ResourceRequests = ResourceRequestsType<Resources>;
type ResourcePromises = ResourcePromisesType<Resources>;

export function PageClient({
  owner,
  repo,
  selectedLines,
  filePath,
  requests,
  promises,
}: {
  owner: string;
  repo: string;
  selectedLines: LineSelection | null;
  filePath: string;
  requests: ResourceRequests;
  promises: ResourcePromises;
}) {
  const resolvedPromises = useResolvePromises(owner, repo, requests, promises);
  return (
    <Suspense fallback={<Loading />}>
      <PageContent
        owner={owner}
        repo={repo}
        selectedLines={selectedLines}
        filePath={filePath}
        promises={resolvedPromises}
      />
    </Suspense>
  );
}

function PageContent({
  owner,
  repo,
  selectedLines,
  filePath,
  promises,
}: {
  owner: string;
  repo: string;
  selectedLines: LineSelection | null;
  filePath: string;
  promises: ResourcePromises;
}) {
  const blob = use(promises.blob);
  if (!blob) {
    return <div>File not found.</div>;
  }

  if (blob.type === "folder") {
    const paths = use(promises.paths);
    return <FolderViewer path={blob.path} paths={paths} />;
  } else {
    const hast = use(promises.hast);
    if (!hast) {
      return <div>File failed to render.</div>;
    }

    const allCommits = use(promises.commits);
    const fileCommits = (allCommits ?? []).filter((c) =>
      c.diffs.some((d) => d.path === filePath),
    );
    return (
      <FileViewerProvider hast={hast} selectedLines={selectedLines}>
        <FileViewer
          fileCommits={fileCommits}
          owner={owner}
          repo={repo}
          path={filePath}
        />
      </FileViewerProvider>
    );
  }
}
