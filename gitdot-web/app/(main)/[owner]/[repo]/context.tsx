"use client";

import type {
  RepositoryCommitsResource,
  RepositoryTreeResource,
} from "gitdot-api";
import { createContext, use, useContext } from "react";

interface RepoContext {
  tree: Promise<RepositoryTreeResource>;
  commits: Promise<RepositoryCommitsResource>;
}

const RepoContext = createContext<RepoContext | null>(null);

export function RepoProvider({
  tree,
  commits,
  children,
}: {
  tree: Promise<RepositoryTreeResource>;
  commits: Promise<RepositoryCommitsResource>;
  children: React.ReactNode;
}) {
  return <RepoContext value={{ tree, commits }}>{children}</RepoContext>;
}

export function useRepoContext(): RepoContext {
  const context = useContext(RepoContext);
  if (!context) {
    throw new Error("useRepo must be used within an RepoProvider");
  }
  return context;
}

export function useRepoResource<K extends keyof RepoContext>(
  key: K,
): Awaited<RepoContext[K]> {
  const context = useRepoContext();
  return use(context[key] as Promise<Awaited<RepoContext[K]>>);
}
