"use client";

import type { Root } from "hast";
import { createContext, useContext, useEffect, useMemo } from "react";
import { setRepoCookie } from "@/cookie";
import { openIdb } from "@/db";
import { DatabaseProvider } from "@/provider";
import { racePromises } from "@/util";
import { useRenderBlobs } from "./hooks/use-render-blobs";
import type { Promises, Requests } from "./layout";
import { sortCommits } from "./util/commit";

type RepoContext = Promises & {
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
  serverRequests: Requests;
  serverPromises: Promises;
  children: React.ReactNode;
}) {
  const idb = useMemo(() => openIdb(), []);
  const dbPromises = useMemo(
    () => new DatabaseProvider(owner, repo).replay(serverRequests),
    [owner, repo, serverRequests],
  );

  const pathsPromise = useMemo(
    () => racePromises(serverPromises.paths, dbPromises.paths),
    [dbPromises, serverPromises],
  );
  const commitsPromise = useMemo(
    () =>
      racePromises(serverPromises.commits, dbPromises.commits).then(
        sortCommits,
      ),
    [dbPromises, serverPromises],
  );
  const blobsPromise = useMemo(
    () => racePromises(serverPromises.blobs, dbPromises.blobs),
    [dbPromises, serverPromises],
  );
  const settingsPromise = useMemo(
    () => racePromises(serverPromises.settings, dbPromises.settings),
    [dbPromises, serverPromises],
  );
  const shikiPromise = useRenderBlobs(owner, repo, blobsPromise);
  const hastsPromise = useMemo(
    () => racePromises(idb.getHasts(owner, repo), shikiPromise),
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
    serverPromises.settings.then((s) => s && idb.putSettings(owner, repo, s));
  }, [owner, repo, idb, serverPromises]);

  return (
    <RepoContext
      value={{
        paths: pathsPromise,
        commits: commitsPromise,
        blobs: blobsPromise,
        settings: settingsPromise,
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
