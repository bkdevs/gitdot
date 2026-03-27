"use client";

import type { Root } from "hast";
import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useRef,
} from "react";
import { createShikiWorker, createSyncWorker } from "@/workers";
import type { ShikiResponse } from "@/workers/shiki";
import type { SyncResponse } from "@/workers/sync";

interface SyncStatus {
  resources: () => void;
  hasts: () => void;
}

interface WorkerContext {
  syncRepo: (
    owner: string,
    repo: string,
  ) => { resources: Promise<void>; hasts: Promise<void> };
  highlightFile: (path: string, content: string) => Promise<Root>;
}

const WorkerContext = createContext<WorkerContext | null>(null);

/**
 * - worker is created in effects to avoid ssr
 * - refs are to associate sent messages with responses and wrap them in promises for user ergonimics
 * - queue is to handle worker instantiation (other effects may use syncRepo or highlightFile)
 */
export function WorkerProvider({ children }: { children: React.ReactNode }) {
  const syncWorker = useRef<SharedWorker | null>(null);
  const syncRepoRequests = useRef<Map<string, SyncStatus>>(new Map());
  const syncRepoQueue = useRef<Array<() => void>>([]);

  const shikiWorker = useRef<SharedWorker | null>(null);
  const highlightFileRequests = useRef<Map<string, (hast: Root) => void>>(
    new Map(),
  );
  const highlightFileQueue = useRef<Array<() => void>>([]);

  useEffect(() => {
    const sync = createSyncWorker();
    syncWorker.current = sync;
    sync.port.start();
    sync.port.onmessage = (e: MessageEvent<SyncResponse>) => {
      const entry = syncRepoRequests.current.get(e.data.id);
      if (!entry) return;
      entry[e.data.stage]();
      if (e.data.stage === "hasts") syncRepoRequests.current.delete(e.data.id);
    };
    for (const fn of syncRepoQueue.current) fn();
    syncRepoQueue.current = []

    const shiki = createShikiWorker();
    shikiWorker.current = shiki;
    shiki.port.start();
    shiki.port.onmessage = (e: MessageEvent<ShikiResponse>) => {
      const resolve = highlightFileRequests.current.get(e.data.id);
      if (resolve) {
        resolve(e.data.hast);
        highlightFileRequests.current.delete(e.data.id);
      }
    };
    for (const fn of highlightFileQueue.current) fn();
    highlightFileQueue.current = [];

    return () => {
      sync.port.close();
      shiki.port.close();
    };
  }, []);

  const syncRepo = useCallback((owner: string, repo: string) => {
    let resolveResources!: () => void;
    let resolveHasts!: () => void;
    const resources = new Promise<void>((r) => {
      resolveResources = r;
    });
    const hasts = new Promise<void>((r) => {
      resolveHasts = r;
    });

    const id = crypto.randomUUID();
    syncRepoRequests.current.set(id, {
      resources: resolveResources,
      hasts: resolveHasts,
    });

    const send = () =>
      syncWorker.current?.port.postMessage({ id, owner, repo });
    if (syncWorker.current) {
      send();
    } else {
      syncRepoQueue.current.push(send);
    }
    return { resources, hasts };
  }, []);

  const highlightFile = useCallback(
    (path: string, content: string): Promise<Root> =>
      new Promise((resolve) => {
        const id = crypto.randomUUID();
        highlightFileRequests.current.set(id, resolve);

        const send = () =>
          shikiWorker.current?.port.postMessage({ id, path, content });
        if (shikiWorker.current) {
          send();
        } else {
          highlightFileQueue.current.push(send);
        }
      }),
    [],
  );

  return (
    <WorkerContext value={{ syncRepo, highlightFile }}>
      {children}
    </WorkerContext>
  );
}

export function useWorkerContext(): WorkerContext {
  const context = useContext(WorkerContext);
  if (!context) {
    throw new Error("useWorkerContext must be used within a WorkerProvider");
  }
  return context;
}
