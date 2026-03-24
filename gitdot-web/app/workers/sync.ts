/// <reference lib="webworker" />
declare const self: SharedWorkerGlobalScope;

import { GetRepositoryResourcesResponse } from "gitdot-api";
import { openIdb } from "@/db/idb";
import { createHighlighter, inferLanguage } from "./util";

interface MessageBody {
  owner: string;
  repo: string;
}

interface MessageResponse {
  resourcesReady: boolean;
  hastsReady: boolean;
}

interface Message {
  body: MessageBody;
  port: MessagePort;
}

const queue: Message[] = [];
let ready = false;

console.log("[sync-worker] script loaded");

self.onconnect = (event: MessageEvent) => {
  console.log("[sync-worker] client connected");
  const port = event.ports[0];
  port.onmessage = (e: MessageEvent<MessageBody>) => {
    console.log("[sync-worker] message received", e.data);
    if (ready) {
      process(e.data, port);
    } else {
      console.log("[sync-worker] not ready, queuing message");
      queue.push({ body: e.data, port });
    }
  };
  port.start();
};

// TODO: this is quite expensive, from one run
// [sync-worker] fetching resources for pybbae/gitdot-laptop
// sync.ts:60 [sync-worker] fetch took 1696.4000000953674ms
// sync.ts:65 [sync-worker] json parse took 36.09999990463257ms
// sync.ts:69 [sync-worker] zod parse took 8.099999904632568ms
// sync.ts:79 [sync-worker] idb write took 220.19999980926514ms
// sync.ts:97 [sync-worker] highlight + hast write took 6603.199999809265ms (1075 files)
//
// i'm unsure why highlight is so slow, but we should likely do all this on on a separate spawned child worker
// a bit iffy with concurrency potentially for multiple repos
async function process({ owner, repo }: MessageBody, port: MessagePort) {
  const db = openIdb();
  const metadata = await db.getMetadata(owner, repo);

  console.log(`[sync-worker] fetching resources for ${owner}/${repo}`);
  const url = new URL(`/${owner}/${repo}/resources`, self.location.origin);
  if (metadata) {
    url.searchParams.set("last_commit", metadata.last_commit);
    url.searchParams.set("last_updated", metadata.last_updated);
  }
  let t = performance.now();
  const response = await fetch(url.toString());
  console.log(`[sync-worker] fetch took ${performance.now() - t}ms`);

  if (!response.ok) return;
  t = performance.now();
  const json = await response.json();
  console.log(`[sync-worker] json parse took ${performance.now() - t}ms`);

  t = performance.now();
  const result = GetRepositoryResourcesResponse.parse(json);
  console.log(`[sync-worker] zod parse took ${performance.now() - t}ms`);
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
  console.log(`[sync-worker] idb write took ${performance.now() - t}ms`);
  port.postMessage({
    resourcesReady: true,
    hastsReady: false,
  } satisfies MessageResponse);

  if (!result.blobs) {
    port.postMessage({
      resourcesReady: true,
      hastsReady: true,
    } satisfies MessageResponse);
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
      return db.putHast(owner, repo, blob.path, hast);
    }),
  );
  console.log(
    `[sync-worker] highlight + hast write took ${performance.now() - t}ms (${fileBlobs.length} files)`,
  );
  port.postMessage({
    resourcesReady: true,
    hastsReady: true,
  } satisfies MessageResponse);
}

const highlighter = await createHighlighter();
ready = true;
console.log("[sync-worker] ready");
for (const { body, port } of queue) process(body, port);
queue.length = 0;
