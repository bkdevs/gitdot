"use client";

import type { RepositoryBlobsResource } from "gitdot-api";
import type { Root } from "hast";
import { useEffect, useMemo, useRef } from "react";
import { useWorkerContext } from "@/(main)/context/worker";
import { openIdb } from "@/db";

export function useRenderBlobs(
  owner: string,
  repo: string,
  blobs: Promise<RepositoryBlobsResource | null>,
): Promise<Map<string, Root>> {
  const idb = useMemo(() => openIdb(), []);
  const hastsRef = useRef(
    (() => {
      let resolve!: (map: Map<string, Root>) => void;
      const promise = new Promise<Map<string, Root>>((res) => {
        resolve = res;
      });
      return { promise, resolve };
    })(),
  );

  const { shiki } = useWorkerContext();

  useEffect(() => {
    if (!shiki) return;

    let onMessage:
      | ((e: MessageEvent<{ path: string; hast: Root }>) => void)
      | null = null;

    blobs.then((blobs) => {
      const fileBlobs = blobs?.blobs.filter((b) => b.type === "file") ?? [];
      if (fileBlobs.length === 0) {
        hastsRef.current.resolve(new Map());
        return;
      }

      const map = new Map<string, Root>();
      let remaining = fileBlobs.length;

      onMessage = (event: MessageEvent<{ path: string; hast: Root }>) => {
        const { path, hast } = event.data;
        map.set(path, hast);
        idb.putHast(owner, repo, path, hast);
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
  }, [shiki, blobs, owner, repo, idb]);

  return hastsRef.current.promise;
}
