"use client";

import { createContext, useContext, useEffect, useState } from "react";
import { useWorkerContext } from "@/(main)/context/worker";

type RepoContext = {
  resourcesReady: boolean;
  hastsReady: boolean;
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
  const { syncRepo } = useWorkerContext();
  const [resourcesReady, setResourcesReady] = useState(false);
  const [hastsReady, setHastsReady] = useState(false);

  useEffect(() => {
    const { resources, hasts } = syncRepo(owner, repo);
    resources.then(() => setResourcesReady(true));
    hasts.then(() => setHastsReady(true));
  }, [syncRepo, owner, repo]);

  return (
    <RepoContext
      value={{
        resourcesReady,
        hastsReady,
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
