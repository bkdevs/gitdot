"use client";

import { createShikiWorker } from "@/workers";
import { createContext, useContext, useEffect, useState } from "react";

interface WorkerContext {
  shiki: Worker | null;
}

const WorkerContext = createContext<WorkerContext | null>(null);

export function WorkerProvider({ children }: { children: React.ReactNode }) {
  const [shiki, setShiki] = useState<Worker | null>(null);

  useEffect(() => {
    const worker = createShikiWorker();
    setShiki(createShikiWorker());
    return () => worker.terminate();
  }, []);

  return (
    <WorkerContext value={{ shiki }}>{children}</WorkerContext>
  );
}

export function useWorkerContext(): WorkerContext {
  const context = useContext(WorkerContext);
  if (!context) {
    throw new Error("useWorkerContext must be used within a WorkerProvider");
  }
  return context;
}
