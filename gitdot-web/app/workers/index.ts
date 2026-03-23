export function createShikiWorker(): Worker {
  return new Worker(new URL("./shiki.ts", import.meta.url));
}

export function createSyncWorker(): SharedWorker {
  return new SharedWorker(new URL("./shiki.ts", import.meta.url));
}
