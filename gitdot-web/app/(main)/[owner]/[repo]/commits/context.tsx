"use client";

import { IdbProvider } from "@/provider";
import { firstNonNull } from "@/util";
import { createContext, useContext, useEffect, useMemo } from "react";
import { type Promises, Resources } from "./resources";

type CommitsContext = Promises;
const CommitsContext = createContext<Promises | null>(null);

export function CommitsClient({
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
      commits: firstNonNull(idbPromises.commits, serverPromises.commits),
    }),
    [idbPromises, serverPromises],
  );

  useEffect(() => {
    serverPromises.commits.then((c) => c && idb.putCommits(c));
  }, [idb, serverPromises]);

  return <CommitsContext value={context}>{children}</CommitsContext>;
}

export function useCommitsContext(): CommitsContext {
  const ctx = useContext(CommitsContext);
  if (!ctx) throw new Error("useCommitsContext must be used within CommitsClient");
  return ctx;
}
