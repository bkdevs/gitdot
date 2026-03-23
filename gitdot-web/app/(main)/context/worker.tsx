"use client";

import { createContext, useContext, useEffect, useState } from "react";
import { createShikiWorker, createSyncWorker } from "@/workers";

interface WorkerContext {
  shiki: Worker | null;
  sync: SharedWorker | null;
}

const WorkerContext = createContext<WorkerContext | null>(null);

export function WorkerProvider({ children }: { children: React.ReactNode }) {
  const [shiki, setShiki] = useState<Worker | null>(null);
  const [sync, setSync] = useState<SharedWorker | null>(null);

  useEffect(() => {
    const worker = createShikiWorker();
    setShiki(worker);
    return () => worker.terminate();
  }, []);

  useEffect(() => {
    const worker = createSyncWorker();
    worker.port.start();
    setSync(worker);
    return () => worker.port.close();
  }, []);

  return <WorkerContext value={{ shiki, sync }}>{children}</WorkerContext>;
}

export function useWorkerContext(): WorkerContext {
  const context = useContext(WorkerContext);
  if (!context) {
    throw new Error("useWorkerContext must be used within a WorkerProvider");
  }
  return context;
}
