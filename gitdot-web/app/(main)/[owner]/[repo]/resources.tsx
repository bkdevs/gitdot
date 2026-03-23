"use client";

import { createContext, useContext, useEffect, useState } from "react";
import { useWorkerContext } from "@/(main)/context/worker";

type RepoResourcesState = {
  resourcesReady: boolean;
  hastsReady: boolean;
};
const RepoResourcesState = createContext<RepoResourcesState | null>(null);

export function RepoResources({
  owner,
  repo,
  serverUrl,
  children,
}: {
  owner: string;
  repo: string;
  serverUrl: string;
  children: React.ReactNode;
}) {
  const { sync } = useWorkerContext();
  const [resourcesReady, setResourcesReady] = useState(false);
  const [hastsReady, setHastsReady] = useState(false);

  useEffect(() => {
    if (!sync) return;
    const handler = (e: MessageEvent<RepoResourcesState>) => {
      setResourcesReady(e.data.resourcesReady);
      setHastsReady(e.data.hastsReady);
    };

    sync.port.addEventListener("message", handler);
    sync.port.postMessage({ owner, repo, serverUrl });
    return () => sync.port.removeEventListener("message", handler);
  }, [sync, owner, repo, serverUrl]);

  return (
    <RepoResourcesState
      value={{
        resourcesReady,
        hastsReady,
      }}
    >
      {children}
    </RepoResourcesState>
  );
}

export function useRepoResources(): RepoResourcesState {
  const ctx = useContext(RepoResourcesState);
  if (!ctx) throw new Error("useRepoContext must be used within RepoClient");
  return ctx;
}
