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
  const blobSizes: { path: string; value: number }[] = [];
  const hastSizes: { path: string; value: number }[] = [];
  const hastTimes: { path: string; value: number }[] = [];
  await Promise.all(
    fileBlobs.map((blob) => {
      const lang = inferLanguage(blob.path) ?? "plaintext";
      const t0 = performance.now();
      const hast = highlighter.codeToHast(blob.content, {
        lang,
        themes: { light: "vitesse-light", dark: "vitesse-dark" },
        defaultColor: "light",
      });
      const elapsed = performance.now() - t0;
      const hastBytes = JSON.stringify(hast).length;
      blobSizes.push({ path: blob.path, value: blob.content.length });
      hastSizes.push({ path: blob.path, value: hastBytes });
      hastTimes.push({ path: blob.path, value: elapsed });
      console.log(
        `[gitdot-sync] codeToHast ${blob.path} (${lang}, ${blob.content.length}b → ${hastBytes}b hast) ${elapsed.toFixed(2)}ms`,
      );
      return db.putHast(owner, repo, blob.path, hast);
    }),
  );
  console.log(
    `[gitdot-sync] highlight + hast write took ${performance.now() - t}ms (${fileBlobs.length} files)`,
  );
  logStats("blob size", blobSizes, (n) => `${(n / 1024).toFixed(1)}kb`);
  logStats("hast size", hastSizes, (n) => `${(n / 1024).toFixed(1)}kb`);
  logStats("codeToHast", hastTimes, (n) => `${n.toFixed(2)}ms`);
  port.postMessage({ id, stage: "hasts" } satisfies SyncResponse);
}

function logStats(
  label: string,
  items: { path: string; value: number }[],
  fmt: (n: number) => string,
) {
  if (items.length === 0) return;
  const sorted = [...items].sort((a, b) => a.value - b.value);
  const total = sorted.reduce((s, x) => s + x.value, 0);
  const pct = (p: number) =>
    sorted[Math.min(sorted.length - 1, Math.floor((sorted.length - 1) * p))]
      .value;
  const biggest = sorted[sorted.length - 1];
  console.log(
    `[gitdot-sync] ${label}: n=${sorted.length} total=${fmt(total)} avg=${fmt(total / sorted.length)} p50=${fmt(pct(0.5))} p95=${fmt(pct(0.95))} max=${fmt(biggest.value)} (${biggest.path})`,
  );
}

const t = performance.now();
const highlighter = await createHighlighter();
console.log(`[gitdot-sync] createHighlighter took ${performance.now() - t}ms`);

ready = true;
console.log("[gitdot-sync] ready");
for (const { body, port } of queue) process(body, port);
queue.length = 0;
