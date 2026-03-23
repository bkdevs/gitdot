export function createShikiWorker(): Worker {
  return new Worker(new URL("./shiki.ts", import.meta.url), {
    name: "gitdot-shiki",
  });
}

export function createSyncWorker(): SharedWorker {
  return new SharedWorker(new URL("./sync.ts", import.meta.url), {
    name: "gitdot-sync",
  });
}
