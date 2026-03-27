/// <reference lib="webworker" />
declare const self: SharedWorkerGlobalScope;

import type { Root } from "hast";
import { createHighlighter, inferLanguage } from "./util";

export interface ShikiRequest {
  id: string;
  path: string;
  content: string;
}

export interface ShikiResponse {
  id: string;
  hast: Root;
}

interface Message {
  body: ShikiRequest;
  port: MessagePort;
}

const queue: Message[] = [];
let ready = false;

console.log("[gitdot-shiki] worker loaded");

self.onconnect = (event: MessageEvent) => {
  console.log("[gitdot-shiki] worker connected");
  const port = event.ports[0];
  port.onmessage = (e: MessageEvent<ShikiRequest>) => {
    if (ready) {
      process(e.data, port);
    } else {
      queue.push({ body: e.data, port });
    }
  };
  port.start();
};

function process({ id, path, content }: ShikiRequest, port: MessagePort) {
  const lang = inferLanguage(path) ?? "plaintext";
  const hast = highlighter.codeToHast(content, {
    lang,
    theme: "vitesse-light",
  });
  port.postMessage({ id, hast } satisfies ShikiResponse);
}

const t = performance.now();
const highlighter = await createHighlighter();
console.log(`[gitdot-shiki] createHighlighter took ${performance.now() - t}ms`);

ready = true;
for (const { body, port } of queue) process(body, port);
queue.length = 0;
