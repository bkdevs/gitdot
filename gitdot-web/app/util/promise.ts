export function delay<T>(ms: number, value: T): Promise<T> {
  return new Promise((resolve) => setTimeout(() => resolve(value), ms));
}

export function racePromises<T>(
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
