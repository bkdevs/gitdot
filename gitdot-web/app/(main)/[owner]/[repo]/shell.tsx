"use client";

import { RepositoryBlobResource } from "gitdot-api";
import { createContext, useContext } from "react";

interface Context {
  readme: Promise<RepositoryBlobResource | null>;
}
const Context = createContext<Context | null>(null);

export function Shell({ children, context }: { children: React.ReactNode, context: Context }) {
  return <Context value={context}>
    {children}
  </Context>
}

export function usePageContext(): Context {
  const context = useContext(Context);
  if (!context) {
    throw new Error("useContext must be used within a ContextProvider");
  }
  return context;
}
