"use client";

import type { Root } from "hast";
import { createContext, useContext, useEffect, useMemo } from "react";
import { setRepoCookie } from "@/cookie";
import { openIdb } from "@/db";
import { resolveResources } from "@/provider/client";
import { racePromises } from "@/util";
import { useRenderBlobs } from "./hooks/use-render-blobs";
import type { ResourcePromises, ResourceRequests } from "./layout";

type RepoContext = ResourcePromises & {
  hasts: Promise<Map<string, Root>>;
};
const RepoContext = createContext<RepoContext | null>(null);

export function RepoClient({
  owner,
  repo,
  serverRequests,
  serverPromises,
  children,
}: {
  owner: string;
  repo: string;
  serverRequests: ResourceRequests;
  serverPromises: ResourcePromises;
  children: React.ReactNode;
}) {
  const idb = useMemo(() => openIdb(), []);
  const racedPromises = useMemo(
    () => resolveResources(owner, repo, serverRequests, serverPromises),
    [owner, repo, serverRequests, serverPromises],
  );

  const shikiPromise = useRenderBlobs(owner, repo, racedPromises.blobs);
  const hastsPromise = useMemo(
    () => racePromises(idb.getHasts(owner, repo), shikiPromise),
    [idb, owner, repo, shikiPromise],
  );

  useEffect(() => {
    serverPromises.paths.then((p) => {
      if (!p) return;
      setRepoCookie(owner, repo, p.commit_sha);
    });
  }, [owner, repo, serverPromises]);

  return (
    <RepoContext
      value={{
        paths: racedPromises.paths,
        commits: racedPromises.commits,
        blobs: racedPromises.blobs,
        settings: racedPromises.settings,
        hasts: hastsPromise.then((m) => m ?? new Map()),
      }}
    >
      {children}
    </RepoContext>
  );
}

export function useRepoContext(): RepoContext {
  const ctx = useContext(RepoContext);
  if (!ctx) throw new Error("useRepoContext must be used within RepoClient");
  return ctx;
}
