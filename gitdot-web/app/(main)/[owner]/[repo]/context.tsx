"use client";

import type { Root } from "hast";
import { createContext, useContext, useEffect, useMemo, useRef } from "react";
import { useWorkerContext } from "@/(main)/context/worker";
import { IdbProvider } from "@/provider";
import { firstNonNull } from "@/util";
import { setRepoCookie } from "@/cookie";
import { type Promises, Resources } from "./resources";

type RepoContext = Promises & { hasts: Promise<Map<string, Root>> };
const RepoContext = createContext<RepoContext | null>(null);

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

  const { shiki } = useWorkerContext();
  const hastsRef = useRef(
    (() => {
      let resolve!: (map: Map<string, Root>) => void;
      const promise = new Promise<Map<string, Root>>((res) => {
        resolve = res;
      });
      return { promise, resolve };
    })(),
  );

  const context = useMemo(
    () => ({
      readme: firstNonNull(idbPromises.readme, serverPromises.readme),
      paths: firstNonNull(idbPromises.paths, serverPromises.paths),
      commits: firstNonNull(idbPromises.commits, serverPromises.commits),
      blobs: firstNonNull(idbPromises.blobs, serverPromises.blobs),
      hasts: hastsRef.current.promise,
    }),
    [idbPromises, serverPromises],
  );

  useEffect(() => {
    serverPromises.paths.then((p) => {
      if (!p) return;
      idb.putPaths(p);
      setRepoCookie(owner, repo, p.commit_sha);
    });
    serverPromises.commits.then((c) => c && idb.putCommits(c));
    serverPromises.blobs.then((b) => b && idb.putBlobs(b));
  }, [idb, serverPromises]);

  useEffect(() => {
    if (!shiki) return;

    let onMessage:
      | ((e: MessageEvent<{ path: string; hast: Root }>) => void)
      | null = null;

    serverPromises.blobs.then((blobs) => {
      const fileBlobs = blobs?.blobs.filter((b) => b.type === "file") ?? [];
      if (fileBlobs.length === 0) {
        hastsRef.current.resolve(new Map());
        return;
      }

      const map = new Map<string, Root>();
      let remaining = fileBlobs.length;

      onMessage = (event: MessageEvent<{ path: string; hast: Root }>) => {
        map.set(event.data.path, event.data.hast);
        if (--remaining === 0) {
          hastsRef.current.resolve(map);
        }
      };
      shiki.addEventListener("message", onMessage);

      for (const blob of fileBlobs) {
        shiki.postMessage({
          path: blob.path,
          code: blob.content,
          theme: "vitesse-light",
        });
      }
    });

    return () => {
      if (onMessage) shiki.removeEventListener("message", onMessage);
    };
  }, [shiki, serverPromises.blobs]);

  return <RepoContext value={context}>{children}</RepoContext>;
}

export function useRepoContext(): RepoContext {
  const ctx = useContext(RepoContext);
  if (!ctx) throw new Error("useRepoContext must be used within RepoClient");
  return ctx;
}
