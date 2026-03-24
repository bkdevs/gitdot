"use client";

import { Suspense, use, useEffect, useMemo, useState } from "react";
import {
  type ResourcePromisesType,
  type ResourceRequestsType,
  useResolvePromises,
} from "@/(main)/[owner]/[repo]/resources";
import { Loading } from "@/ui/loading";
import { DatabaseProvider } from "@/provider/database";
import type { RepositoryPathResource } from "gitdot-api";
import type { Resources } from "./page";
import { FileBody } from "./ui/file-body";
import { FolderViewer } from "./ui/folder-viewer";
import type { LineSelection } from "./util";

type ResourceRequests = ResourceRequestsType<Resources>;
type ResourcePromises = ResourcePromisesType<Resources>;

export function PageClient({
  owner,
  repo,
  selectedLines,
  historySlot,
  requests,
  promises,
}: {
  owner: string;
  repo: string;
  selectedLines: LineSelection | null;
  historySlot: React.ReactNode;
  requests: ResourceRequests;
  promises: ResourcePromises;
}) {
  // TODO: this is being re-invoked many times as history slot streams in? selected lines too?
  // in general resolveResources is fine to use as long as it is strictly owner, repo, requests, promises
  const resolvedPromises = useResolvePromises(owner, repo, requests, promises);
  return (
    <Suspense fallback={<Loading />}>
      <PageContent
        owner={owner}
        repo={repo}
        selectedLines={selectedLines}
        historySlot={historySlot}
        promises={resolvedPromises}
      />
    </Suspense>
  );
}

function PageContent({
  owner,
  repo,
  selectedLines,
  historySlot,
  promises,
}: {
  owner: string;
  repo: string;
  selectedLines: LineSelection | null;
  historySlot: React.ReactNode;
  promises: ResourcePromises;
}) {
  const blob = use(promises.blob);
  const readme = use(promises.readme);
  if (!blob) {
    return <div>File not found.</div>;
  }
  if (blob.type === "folder") {
    const readmeContent =
      readme?.type === "file" ? readme.content : null;
    if (!readmeContent) {
      return (
        <FolderViewerWithPaths
          owner={owner}
          repo={repo}
          folderPath={blob.path}
          entries={blob.entries}
        />
      );
    }
    return (
      <FolderViewer
        owner={owner}
        repo={repo}
        folderPath={blob.path}
        entries={blob.entries}
        readme={readmeContent}
      />
    );
  }

  const hast = use(promises.hast);
  if (!hast) {
    return <div>File failed to render.</div>;
  }

  return (
    <div className="flex w-full h-full min-h-0 overflow-hidden">
      <div
        data-page-scroll
        className="flex-1 min-w-0 overflow-auto scrollbar-thin"
      >
        <FileBody selectedLines={selectedLines} hast={hast} />
      </div>
      {historySlot}
    </div>
  );
}

function FolderViewerWithPaths({
  owner,
  repo,
  folderPath,
  entries,
}: {
  owner: string;
  repo: string;
  folderPath: string;
  entries: RepositoryPathResource[];
}) {
  const [allFiles, setAllFiles] = useState<RepositoryPathResource[] | null>(null);
  const db = useMemo(() => new DatabaseProvider(owner, repo), [owner, repo]);

  useEffect(() => {
    db.getPaths().then((paths) => {
      if (!paths) return;
      const prefix = folderPath ? `${folderPath}/` : "";
      setAllFiles(
        paths.entries.filter(
          (e) =>
            (e.path_type === "blob" || e.path_type === "tree") &&
            e.path.startsWith(prefix),
        ),
      );
    });
  }, [db, folderPath]);

  return (
    <FolderViewer
      owner={owner}
      repo={repo}
      folderPath={folderPath}
      entries={entries}
      allFiles={allFiles}
    />
  );
}
