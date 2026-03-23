export function createSyncWorker(): SharedWorker {
  return new SharedWorker(new URL("./sync.ts", import.meta.url), {
    name: "gitdot-sync",
  });
}
