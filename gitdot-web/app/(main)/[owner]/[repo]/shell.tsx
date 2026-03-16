"use client";

import type { RepositoryBlobResource } from "gitdot-api";
import { createContext, useContext, useMemo } from "react";
import { IdbProvider } from "@/provider";
import { firstNonNull } from "@/util";
import { repoDef } from "./def";

interface Context {
  readme: Promise<RepositoryBlobResource | null>;
}

interface ServerContext {
  readme: Promise<RepositoryBlobResource | null>;
}

const Context = createContext<Context | null>(null);

export function Shell({
  children,
  owner,
  repo,
  serverContext,
}: {
  children: React.ReactNode;
  owner: string;
  repo: string;
  serverContext: ServerContext;
}) {
  const idbContext = useMemo(
    () => new IdbProvider(owner, repo).define(repoDef),
    [owner, repo],
  );

  const context = useMemo(
    () => ({
      readme: firstNonNull(idbContext.readme, serverContext.readme),
    }),
    [idbContext, serverContext],
  );

  return <Context value={context}>{children}</Context>;
}

export function usePageContext(): Context {
  const context = useContext(Context);
  if (!context) {
    throw new Error("useContext must be used within a ContextProvider");
  }
  return context;
}
