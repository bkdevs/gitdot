"use client";

import {
  type ResourcePromisesType,
  type ResourceRequestsType,
  resolveResources,
} from "@/provider/client";
import { Suspense, use } from "react";
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
  const resolvedPromises = resolveResources(owner, repo, requests, promises);
  return (
    <Suspense>
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
  if (!blob) {
    return <div>File not found.</div>;
  }
  if (blob.type === "folder") {
    return <FolderViewer owner={owner} repo={repo} entries={blob.entries} />;
  }

  const hast = use(promises.hast);
  if (!hast) {
    return <div>File failed to render.</div>
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
