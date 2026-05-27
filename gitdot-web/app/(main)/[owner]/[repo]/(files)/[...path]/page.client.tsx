"use client";

import {
  type ResourcePromisesType,
  type ResourceResultType,
  useResources,
} from "gitdot-dal/client";
import { Suspense, use } from "react";
import { Loading } from "@/ui/loading";
import type { Resources } from "./page";
import { FileViewer } from "./ui/file-viewer";
import { FileViewerProvider } from "./ui/file-viewer-context";
import { FolderViewer } from "./ui/folder-viewer";
import type { LineSelection } from "./util";

type ResourcePromises = ResourcePromisesType<Resources>;

export function PageClient({
  owner,
  repo,
  selectedLines,
  filePath,
  resources,
}: {
  owner: string;
  repo: string;
  selectedLines: LineSelection | null;
  filePath: string;
  resources: ResourceResultType<Resources>;
}) {
  const resourcePromises = useResources(resources);
  return (
    <Suspense fallback={<Loading />}>
      <PageContent
        owner={owner}
        repo={repo}
        selectedLines={selectedLines}
        filePath={filePath}
        promises={resourcePromises}
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
  const paths = use(promises.paths);
  const entry = paths?.entries.find((e) => e.path === filePath);
  if (!entry) {
    return <div>File not found.</div>;
  }

  if (entry.path_type === "tree") {
    return <FolderViewer path={filePath} paths={paths} />;
  }

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
