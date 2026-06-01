"use client";

import { ClientProvider } from "gitdot-dal/client";
import { useEffect } from "react";

const SYNC_INTERVAL_MS = 5 * 60 * 1000;

export function RepoSync({
  owner,
  repo,
  children,
}: {
  owner: string;
  repo: string;
  children: React.ReactNode;
}) {
  useEffect(() => {
    ClientProvider.instance.syncRepo(owner, repo);
    const interval = setInterval(() => {
      ClientProvider.instance.syncRepo(owner, repo);
    }, SYNC_INTERVAL_MS);
    return () => clearInterval(interval);
  }, [owner, repo]);

  return <>{children}</>;
}
