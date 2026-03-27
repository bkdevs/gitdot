export function createSyncWorker(): SharedWorker {
  return new SharedWorker(new URL("./sync.ts", import.meta.url), {
    name: "gitdot-sync",
  });
}

export function createShikiWorker(): SharedWorker {
  return new SharedWorker(new URL("./shiki.ts", import.meta.url), {
    name: "gitdot-shiki",
    type: "module",
  });
}
