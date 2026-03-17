"use client";

import type { Root } from "hast";
import { createContext, useContext, useEffect, useMemo } from "react";
import { setRepoCookie } from "@/cookie";
import { IdbProvider } from "@/provider";
import { firstNonNull } from "@/util";
import { useRenderBlobs } from "./hooks/use-render-blobs";
import { type RepoPromises, RepoResources } from "./resources";

type RepoContext = RepoPromises & { hasts: Promise<Map<string, Root>> };
const RepoContext = createContext<RepoContext | null>(null);

export function RepoClient({
  owner,
  repo,
  serverPromises,
  children,
}: {
  owner: string;
  repo: string;
  serverPromises: RepoPromises;
  children: React.ReactNode;
}) {
  const idb = useMemo(() => new IdbProvider(owner, repo), [owner, repo]);
  const idbPromises = useMemo(() => idb.fetch(RepoResources), [idb]);

  const pathsPromise = useMemo(
    () => firstNonNull(idbPromises.paths, serverPromises.paths),
    [idbPromises, serverPromises],
  );
  const commitsPromise = useMemo(
    () => firstNonNull(idbPromises.commits, serverPromises.commits),
    [idbPromises, serverPromises],
  );
  const blobsPromise = useMemo(
    () => firstNonNull(idbPromises.blobs, serverPromises.blobs),
    [idbPromises, serverPromises],
  );
  const shikiPromise = useRenderBlobs(blobsPromise, idb);
  const hastsPromise = useMemo(
    () => firstNonNull(idb.getHasts(), shikiPromise),
    [idb, shikiPromise],
  );

  useEffect(() => {
    serverPromises.paths.then((p) => {
      if (!p) return;
      idb.putPaths(p);
      setRepoCookie(owner, repo, p.commit_sha);
    });
    serverPromises.commits.then((c) => c && idb.putCommits(c));
    serverPromises.blobs.then((b) => b && idb.putBlobs(b));
  }, [owner, repo, idb, serverPromises]);

  return (
    <RepoContext
      value={{
        paths: pathsPromise,
        commits: commitsPromise,
        blobs: blobsPromise,
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
