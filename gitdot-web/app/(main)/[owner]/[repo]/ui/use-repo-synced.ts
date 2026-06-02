"use client";

import { ClientProvider } from "gitdot-dal/client";
import { useEffect, useState } from "react";

const POLL_INTERVAL_MS = 100;
const POLL_TIMEOUT_MS = 15_000;

/**
 * Whether the repo's data has finished syncing into IndexedDB. Polls the
 * provider's synchronous repoSynced() every 100ms until true (or a 15s cap),
 * so components gating on a background sync re-render once it lands.
 */
export function useRepoSynced(owner: string, repo: string): boolean {
  const [synced, setSynced] = useState(() =>
    ClientProvider.instance.repoSynced(owner, repo),
  );

  useEffect(() => {
    if (ClientProvider.instance.repoSynced(owner, repo)) {
      setSynced(true);
      return;
    }
    setSynced(false);
    const deadline = Date.now() + POLL_TIMEOUT_MS;
    const interval = setInterval(() => {
      if (ClientProvider.instance.repoSynced(owner, repo)) {
        setSynced(true);
        clearInterval(interval);
      } else if (Date.now() >= deadline) {
        clearInterval(interval);
      }
    }, POLL_INTERVAL_MS);
    return () => clearInterval(interval);
  }, [owner, repo]);

  return synced;
}
