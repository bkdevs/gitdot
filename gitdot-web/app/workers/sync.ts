/// <reference lib="webworker" />

import { GetRepositoryResourcesResponse } from "gitdot-api";

declare const self: SharedWorkerGlobalScope;

interface Message {
  owner: string;
  repo: string;
  serverUrl: string;
}

const queue: Message[] = [];
let ready = false;

console.log("[sync-worker] script loaded");

self.onconnect = (event: MessageEvent) => {
  console.log("[sync-worker] client connected");
  const port = event.ports[0];
  port.onmessage = (e: MessageEvent<Message>) => {
    console.log("[sync-worker] message received", e.data);
    if (ready) {
      process(e.data);
    } else {
      console.log("[sync-worker] not ready, queuing message");
      queue.push(e.data);
    }
  };
  port.start();
};

async function process({ owner, repo, serverUrl }: Message) {
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
}

ready = true;
console.log("[sync-worker] ready");
for (const msg of queue) process(msg);
queue.length = 0;
