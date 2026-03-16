"use client";

import { createContext, useContext, useEffect, useMemo } from "react";
import { IdbProvider } from "@/provider";
import { firstNonNull } from "@/util";
import { type Promises, Resources } from "./resources";

type RepoContext = Promises;
const RepoContext = createContext<Promises | null>(null);

export function RepoClient({
  owner,
  repo,
  serverPromises,
  children,
}: {
  owner: string;
  repo: string;
  serverPromises: Promises;
  children: React.ReactNode;
}) {
  const idb = useMemo(() => new IdbProvider(owner, repo), [owner, repo]);
  const idbPromises = useMemo(() => idb.fetch(Resources), [idb]);

  const context = useMemo(
    () => ({
      readme: firstNonNull(idbPromises.readme, serverPromises.readme),
      paths: firstNonNull(idbPromises.paths, serverPromises.paths),
      blobs: firstNonNull(idbPromises.blobs, serverPromises.blobs),
    }),
    [idbPromises, serverPromises],
  );

  useEffect(() => {
    serverPromises.paths.then((p) => p && idb.putPaths(p));
    serverPromises.blobs.then((b) => b && idb.putBlobs(b));
  }, [idb, serverPromises]);

  return <RepoContext value={context}>{children}</RepoContext>;
}

export function useRepoContext(): RepoContext {
  const ctx = useContext(RepoContext);
  if (!ctx) throw new Error("useRepoContext must be used within RepoClient");
  return ctx;
}
