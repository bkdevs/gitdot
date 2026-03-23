"use client";

import { createContext, useContext, useEffect, useRef, useState } from "react";
import { useWorkerContext } from "@/(main)/context/worker";
import { InMemoryProvider } from "@/provider/memory";

type RepoContext = {
  resourcesReady: boolean;
  hastsReady: boolean;
  provider: InMemoryProvider;
};
const RepoContext = createContext<RepoContext | null>(null);

export function RepoResources({
  owner,
  repo,
  children,
}: {
  owner: string;
  repo: string;
  children: React.ReactNode;
}) {
  const { sync } = useWorkerContext();
  const [resourcesReady, setResourcesReady] = useState(false);
  const [hastsReady, setHastsReady] = useState(false);
  const provider = useRef(new InMemoryProvider(owner, repo)).current;

  useEffect(() => {
    provider.initialize();
  }, [provider]);

  useEffect(() => {
    if (resourcesReady) provider.initialize();
  }, [resourcesReady, provider]);

  useEffect(() => {
    if (!sync) return;
    const handler = (
      e: MessageEvent<{ resourcesReady: boolean; hastsReady: boolean }>,
    ) => {
      setResourcesReady(e.data.resourcesReady);
      setHastsReady(e.data.hastsReady);
    };

    sync.port.addEventListener("message", handler);
    sync.port.postMessage({ owner, repo });
    return () => sync.port.removeEventListener("message", handler);
  }, [sync, owner, repo]);

  return (
    <RepoContext
      value={{
        resourcesReady,
        hastsReady,
        provider,
      }}
    >
      {children}
    </RepoContext>
  );
}

export function useRepoContext(): RepoContext {
  const ctx = useContext(RepoContext);
  if (!ctx) throw new Error("useRepoContext must be used within RepoResources");
  return ctx;
}
