export function createShikiWorker(): Worker {
  return new Worker(new URL("./shiki.ts", import.meta.url));
}
