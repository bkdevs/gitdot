/// <reference lib="webworker" />
/// ==================================
import { createHighlighter, inferLanguage } from "./util";

/// ==================================

// necessary as while the browser will queue messages while the worker is downloading
// it does _NOT_ queue between download -> self.onmessage
// that is, the block of code from imports above to what you see below must be very very fast
interface Message {
  path: string;
  code: string;
  theme: "vitesse-light" | "gitdot-light";
}

const queue: Message[] = [];
let ready = false;

self.onmessage = (event: MessageEvent<Message>) => {
  if (ready) {
    process(event.data);
  } else {
    queue.push(event.data);
  }
};

function process({ path, code, theme }: Message) {
  const lang = inferLanguage(path) ?? "plaintext";
  const hast = highlighter.codeToHast(code, { lang, theme });
  self.postMessage({ path, hast });
}

const start = performance.now();
const highlighter = await createHighlighter();

console.log(`[shiki-worker] took ${performance.now() - start}ms to load`);
ready = true;
for (const msg of queue) process(msg);
queue.length = 0;
