"use client";

import { createSyncWorker } from "@/workers";
import { createContext, useContext, useEffect, useState } from "react";

interface WorkerContext {
  sync: SharedWorker | null;
}

const WorkerContext = createContext<WorkerContext | null>(null);

export function WorkerProvider({ children }: { children: React.ReactNode }) {
  const [sync, setSync] = useState<SharedWorker | null>(null);

  useEffect(() => {
    const worker = createSyncWorker();
    worker.port.start();
    setSync(worker);
    return () => worker.port.close();
  }, []);

  return <WorkerContext value={{ sync }}>{children}</WorkerContext>;
}

export function useWorkerContext(): WorkerContext {
  const context = useContext(WorkerContext);
  if (!context) {
    throw new Error("useWorkerContext must be used within a WorkerProvider");
  }
  return context;
}
