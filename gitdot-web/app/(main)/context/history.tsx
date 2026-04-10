"use client";

import { usePathname } from "next/navigation";
import { createContext, useContext, useEffect, useState } from "react";
import { HistoryDialog } from "@/(main)/ui/history-dialog";

const HISTORY_KEY = "gd_nav_history";
const MAX_HISTORY = 10;

function readLocalHistory(): string[] {
  if (typeof window === "undefined") return [];
  try {
    return JSON.parse(localStorage.getItem(HISTORY_KEY) ?? "[]");
  } catch {
    return [];
  }
}

function addToLocalHistory(path: string): string[] {
  const prev = readLocalHistory().filter((p) => p !== path);
  const next = [path, ...prev].slice(0, MAX_HISTORY);
  localStorage.setItem(HISTORY_KEY, JSON.stringify(next));
  return next;
}

interface HistoryContextValue {
  history: string[];
}

const HistoryContext = createContext<HistoryContextValue>({ history: [] });

export function useHistoryContext() {
  return useContext(HistoryContext);
}

export function HistoryProvider({ children }: { children: React.ReactNode }) {
  const pathname = usePathname();
  const [history, setHistory] = useState<string[]>(readLocalHistory);

  useEffect(() => {
    setHistory(addToLocalHistory(pathname));
  }, [pathname]);

  return (
    <HistoryContext value={{ history }}>
      {children}
      <HistoryDialog />
    </HistoryContext>
  );
}
