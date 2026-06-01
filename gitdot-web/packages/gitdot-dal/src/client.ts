import { ClientProvider } from "./provider/client";
import type {
  ResourcePromisesType,
  ResourceRequestType,
  ResourceResultType,
} from "./provider/types";

export * from "./diff/client";
export * from "./diff/shiki";
export { default as gitdotDark } from "./diff/themes/gitdot-dark";
export { default as gitdotLight } from "./diff/themes/gitdot-light";
export { default as gitdotMono } from "./diff/themes/gitdot-mono";
export * from "./diff/types";
export * from "./provider/client";
export * from "./provider/types";
export { createShikiWorker, createSyncWorker } from "./workers";
export type { ShikiRequest, ShikiResponse } from "./workers/shiki";
export type { SyncRequest, SyncResponse } from "./workers/sync";

export function useResources<S>(
  resources: ResourceResultType<S>,
): ResourcePromisesType<S> {
  const provider = ClientProvider.instance;
  const localPromises: Record<string, Promise<unknown>> = {};
  const requests = resources.requests as Record<string, ResourceRequestType>;

  for (const [key, { method, args }] of Object.entries(requests)) {
    const func = provider[method as keyof ClientProvider];
    if (typeof func !== "function") {
      throw new Error(`ClientProvider has no method "${method}"`);
    }
    localPromises[key] = (func as (...a: unknown[]) => Promise<unknown>).apply(
      provider,
      args,
    );
  }
  const requestKeys = Object.keys(requests);
  const timings: Record<
    string,
    { client: number | null; server: number | null }
  > = {};

  for (const key of requestKeys) timings[key] = { client: null, server: null };
  let remaining = requestKeys.length * 2;
  const tick = () => {
    if (--remaining === 0) console.log("[useResources]", timings);
  };

  const start = performance.now();
  const result: Record<string, Promise<unknown>> = {};
  for (const key of requestKeys) {
    const serverPromise = resources.promises[key as keyof S];
    const clientPromise = localPromises[key];
    Promise.resolve(clientPromise).finally(() => {
      timings[key].client = +(performance.now() - start).toFixed(2);
      tick();
    });
    Promise.resolve(serverPromise).finally(() => {
      timings[key].server = +(performance.now() - start).toFixed(2);
      tick();
    });
    result[key] = racePromises(serverPromise, clientPromise);
  }
  return result as ResourcePromisesType<S>;
}

function racePromises<T>(
  first: Promise<T>,
  ...rest: Promise<unknown>[]
): Promise<T | null> {
  return new Promise((resolve) => {
    let remaining = 1 + rest.length;
    let resolved = false;

    const all = [first, ...rest] as Promise<T | null | undefined>[];
    for (const p of all) {
      Promise.resolve(p)
        .then((value) => {
          if (resolved) return;
          if (value != null) {
            resolved = true;
            resolve(value);
          } else if (--remaining === 0) {
            resolve(null);
          }
        })
        .catch(() => {
          if (resolved) return;
          if (--remaining === 0) resolve(null);
        });
    }
  });
}
