export function delay<T>(ms: number, value: T): Promise<T> {
  return new Promise((resolve) => setTimeout(() => resolve(value), ms));
}

export function firstNonNull<T>(
  ...promises: Promise<T | null | undefined>[]
): Promise<T> {
  return new Promise((resolve, reject) => {
    let remaining = promises.length;
    let resolved = false;
    for (const p of promises) {
      Promise.resolve(p).then((value) => {
        if (resolved) return;
        if (value != null) {
          resolved = true;
          resolve(value);
        } else if (--remaining === 0) {
          reject(new Error("all null"));
        }
      }).catch(() => {
        if (resolved) return;
        if (--remaining === 0) reject(new Error("all null"));
      });
    }
  });
}
