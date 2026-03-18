"use client";

import type { Root } from "hast";
import { createContext, useContext, useEffect, useMemo } from "react";
import { setRepoCookie } from "@/cookie";
import { openIdb } from "@/db";
import { DbProvider } from "@/provider";
import { firstNonNull } from "@/util";
import { useRenderBlobs } from "./hooks/use-render-blobs";
import { type RepoPromises, RepoResources } from "./resources";
import { sortCommits } from "./util/commit";

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
  const idb = useMemo(() => openIdb(), []);
  const dbPromises = useMemo(
    () => new DbProvider(owner, repo).fetch(RepoResources),
    [owner, repo],
  );

  const pathsPromise = useMemo(
    () => firstNonNull(dbPromises.paths, serverPromises.paths),
    [dbPromises, serverPromises],
  );
  const commitsPromise = useMemo(
    () =>
      firstNonNull(dbPromises.commits, serverPromises.commits).then(
        sortCommits,
      ),
    [dbPromises, serverPromises],
  );
  const blobsPromise = useMemo(
    () => firstNonNull(dbPromises.blobs, serverPromises.blobs),
    [dbPromises, serverPromises],
  );
  const shikiPromise = useRenderBlobs(owner, repo, blobsPromise);
  const hastsPromise = useMemo(
    () => firstNonNull(idb.getHasts(owner, repo), shikiPromise),
    [idb, owner, repo, shikiPromise],
  );

  useEffect(() => {
    serverPromises.paths.then((p) => {
      if (!p) return;
      idb.putPaths(owner, repo, p);
      setRepoCookie(owner, repo, p.commit_sha);
    });
    serverPromises.commits.then((c) => c && idb.putCommits(owner, repo, c));
    serverPromises.blobs.then((b) => b && idb.putBlobs(owner, repo, b));
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
