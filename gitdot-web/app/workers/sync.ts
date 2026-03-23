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

self.onconnect = (event: MessageEvent) => {
  const port = event.ports[0];
  port.onmessage = (e: MessageEvent<Message>) => {
    if (ready) {
      process(e.data);
    } else {
      queue.push(e.data);
    }
  };
  port.start();
};

async function process({ owner, repo, serverUrl }: Message) {
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

  const json = await response.json();
  const result = GetRepositoryResourcesResponse.parse(json);
  console.log("[sync-worker]", result);
}

ready = true;
for (const msg of queue) process(msg);
queue.length = 0;
