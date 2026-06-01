/// <reference lib="webworker" />
declare const self: SharedWorkerGlobalScope;

import type { RepositoryBlobPairResource } from "gitdot-api";
import type { Root } from "hast";
import { inferLanguage, renderDiff, renderHast } from "../diff/shiki";
import type { DiffData } from "../diff/types";

export type ShikiRequest =
  | { id: string; kind: "blob"; path: string; content: string }
  | { id: string; kind: "diff"; pairs: RepositoryBlobPairResource[] };

export type ShikiResponse =
  | { id: string; kind: "blob"; hast: Root }
  | { id: string; kind: "diff"; data: DiffData };

console.log("[gitdot-shiki] worker loaded");

self.onconnect = (event: MessageEvent) => {
  console.log("[gitdot-shiki] worker connected");
  const port = event.ports[0];
  port.onmessage = (e: MessageEvent<ShikiRequest>) => {
    process(e.data, port);
  };
  port.start();
};

async function process(req: ShikiRequest, port: MessagePort) {
  const t = performance.now();
  if (req.kind === "blob") {
    const lang = inferLanguage(req.path);
    const hast = await renderHast(req.content, lang, "gitdot");
    console.log(
      `[gitdot-shiki] blob ${req.path} (${lang ?? "plaintext"}, ${req.content.length}b) ${(performance.now() - t).toFixed(2)}ms`,
    );
    port.postMessage({
      id: req.id,
      kind: "blob",
      hast,
    } satisfies ShikiResponse);
  } else {
    const data = await renderDiff(req.pairs);
    console.log(
      `[gitdot-shiki] diff ${req.pairs.length} files ${(performance.now() - t).toFixed(2)}ms`,
    );
    port.postMessage({
      id: req.id,
      kind: "diff",
      data,
    } satisfies ShikiResponse);
  }
}
