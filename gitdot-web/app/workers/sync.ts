/// <reference lib="webworker" />
/// ==================================

import { GetRepositoryResourcesResponse } from "gitdot-api";
import { openIdb } from "@/db/idb";

declare const self: SharedWorkerGlobalScope;

interface MessageBody {
  owner: string;
  repo: string;
  serverUrl: string;
}
interface MessageResponse {
  ready: true;
}
interface Message {
  body: MessageBody;
  port: MessagePort;
}
/// ==================================

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

async function process(
  { owner, repo, serverUrl }: MessageBody,
  port: MessagePort,
) {
  console.log(`[sync-worker] fetching resources for ${owner}/${repo}`);
  const response = await fetch(
    `${serverUrl}/repository/${owner}/${repo}/resources`,
    {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({}),
    },
  );

  console.log(`[sync-worker] response status: ${response.status}`);
  const json = await response.json();
  console.log("[sync-worker] response json", json);
  if (!response.ok) return;
  const result = GetRepositoryResourcesResponse.parse(json);
  console.log("[sync-worker] result", result);

  const db = openIdb();
  await Promise.all([
    db.putPaths(owner, repo, result.paths),
    db.putCommits(owner, repo, result.commits.commits),
    db.putBlobs(owner, repo, result.blobs),
    db.putSettings(owner, repo, result.settings),
  ]);
  console.log("[sync-worker] wrote resources to idb");
  port.postMessage({ ready: true } satisfies MessageResponse);
}

ready = true;
console.log("[sync-worker] ready");
for (const { body, port } of queue) process(body, port);
queue.length = 0;
