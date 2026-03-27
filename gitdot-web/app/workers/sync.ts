/// <reference lib="webworker" />
declare const self: SharedWorkerGlobalScope;

import { GetRepositoryResourcesResponse } from "gitdot-api";
import { openIdb } from "@/db/idb";
import { createHighlighter, inferLanguage } from "./util";

export interface SyncRequest {
  id: string;
  owner: string;
  repo: string;
}

export interface SyncResponse {
  id: string;
  stage: "resources" | "hasts";
}

interface Message {
  body: SyncRequest;
  port: MessagePort;
}

const queue: Message[] = [];
let ready = false;

console.log("[gitdot-sync] worker loaded");

self.onconnect = (event: MessageEvent) => {
  console.log("[gitdot-sync] worker connected");
  const port = event.ports[0];
  port.onmessage = (e: MessageEvent<SyncRequest>) => {
    if (ready) {
      process(e.data, port);
    } else {
      queue.push({ body: e.data, port });
    }
  };
  port.start();
};

// TODO: this is quite expensive, from one run
// [gitdot-sync] fetching resources for pybbae/gitdot-laptop
// sync.ts:60 [gitdot-sync] fetch took 1696.4000000953674ms
// sync.ts:65 [gitdot-sync] json parse took 36.09999990463257ms
// sync.ts:69 [gitdot-sync] zod parse took 8.099999904632568ms
// sync.ts:79 [gitdot-sync] idb write took 220.19999980926514ms
// sync.ts:97 [gitdot-sync] highlight + hast write took 6603.199999809265ms (1075 files)
async function process({ id, owner, repo }: SyncRequest, port: MessagePort) {
  const db = openIdb();
  const metadata = await db.getMetadata(owner, repo);

  console.log(`[gitdot-sync] fetching resources for ${owner}/${repo}`);
  const url = new URL(`/${owner}/${repo}/resources`, self.location.origin);
  if (metadata) {
    url.searchParams.set("last_commit", metadata.last_commit);
    url.searchParams.set("last_updated", metadata.last_updated);
  }
  let t = performance.now();
  const response = await fetch(url.toString());
  console.log(`[gitdot-sync] fetch took ${performance.now() - t}ms`);

  if (!response.ok) return;
  t = performance.now();
  const json = await response.json();
  console.log(`[gitdot-sync] json parse took ${performance.now() - t}ms`);

  t = performance.now();
  const result = GetRepositoryResourcesResponse.parse(json);
  console.log(`[gitdot-sync] zod parse took ${performance.now() - t}ms`);
  console.log(result);

  t = performance.now();
  const writes: Promise<void>[] = [
    db.putMetadata(owner, repo, {
      last_commit: result.last_commit,
      last_updated: result.last_updated ?? new Date().toISOString(),
    }),
  ];
  if (result.paths) writes.push(db.putPaths(owner, repo, result.paths));
  if (result.blobs) writes.push(db.putBlobs(owner, repo, result.blobs));
  if (result.commits) {
    for (const c of result.commits.commits)
      writes.push(db.putCommit(owner, repo, c));
  }
  if (result.questions)
    writes.push(db.putQuestions(owner, repo, result.questions.questions));
  if (result.reviews) {
    for (const r of result.reviews.reviews)
      writes.push(db.putReview(owner, repo, r.number, r));
  }
  if (result.builds)
    writes.push(db.putBuilds(owner, repo, result.builds.builds));
  await Promise.all(writes);

  console.log(`[gitdot-sync] idb write took ${performance.now() - t}ms`);

  port.postMessage({ id, stage: "resources" } satisfies SyncResponse);
  if (!result.blobs) {
    port.postMessage({ id, stage: "hasts" } satisfies SyncResponse);
    return;
  }

  t = performance.now();
  const fileBlobs = result.blobs.blobs.filter((b) => b.type === "file");
  await Promise.all(
    fileBlobs.map((blob) => {
      const lang = inferLanguage(blob.path) ?? "plaintext";
      const hast = highlighter.codeToHast(blob.content, {
        lang,
        theme: "vitesse-light",
      });
      return db.putHast(owner, repo, blob.path, hast, blob.commit_sha);
    }),
  );
  console.log(
    `[gitdot-sync] highlight + hast write took ${performance.now() - t}ms (${fileBlobs.length} files)`,
  );
  port.postMessage({ id, stage: "hasts" } satisfies SyncResponse);
}

const t = performance.now();
const highlighter = await createHighlighter();
console.log(`[gitdot-sync] createHighlighter took ${performance.now() - t}ms`);

ready = true;
console.log("[gitdot-sync] ready");
for (const { body, port } of queue) process(body, port);
queue.length = 0;
