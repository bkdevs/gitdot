/// <reference lib="webworker" />
declare const self: SharedWorkerGlobalScope;

import { GetRepositoryResourcesResponse } from "gitdot-api";
import { openIdb } from "../db/idb";

export interface SyncRequest {
  id: string;
  owner: string;
  repo: string;
  forceRefresh: boolean;
}

export interface SyncResponse {
  id: string;
  done: true;
}

console.log("[gitdot-sync] worker loaded");

self.onconnect = (event: MessageEvent) => {
  console.log("[gitdot-sync] worker connected");
  const port = event.ports[0];
  port.onmessage = (e: MessageEvent<SyncRequest>) => {
    process(e.data, port);
  };
  port.start();
};

async function process(
  { id, owner, repo, forceRefresh }: SyncRequest,
  port: MessagePort,
) {
  const start = performance.now();
  const db = openIdb();
  const metadata = forceRefresh ? null : await db.getMetadata(owner, repo);

  const url = new URL("/api/repository/resources", self.location.origin);
  url.searchParams.set("owner", owner);
  url.searchParams.set("repo", repo);
  if (metadata) {
    url.searchParams.set("last_commit", metadata.last_commit);
    url.searchParams.set("last_updated", metadata.last_updated);
  }
  const response = await fetch(url.toString());
  if (!response.ok) return;
  const result = GetRepositoryResourcesResponse.parse(await response.json());

  if (result.paths) await db.putPaths(owner, repo, result.paths);
  if (result.blobs) await db.putBlobs(owner, repo, result.blobs);
  if (result.commits) {
    for (const c of result.commits.commits) await db.putCommit(owner, repo, c);
  }
  await db.putMetadata(owner, repo, {
    last_commit: result.last_commit,
    last_updated: result.last_updated ?? new Date().toISOString(),
  });

  console.log(
    `[gitdot-sync] ${owner}/${repo} ${(performance.now() - start).toFixed(2)}ms`,
  );
  port.postMessage({ id, done: true } satisfies SyncResponse);
}
