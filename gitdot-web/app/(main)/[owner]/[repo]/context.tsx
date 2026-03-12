"use client";

import type {
  RepositoryCommitsResource,
  RepositoryTreeResource,
} from "gitdot-api";
import { createContext, useContext } from "react";

interface RepoContext {
  tree: RepositoryTreeResource;
  commits: RepositoryCommitsResource;
}

const RepoContext = createContext<RepoContext | null>(null);

export function RepoProvider({
  tree,
  commits,
  children,
}: {
  tree: RepositoryTreeResource;
  commits: RepositoryCommitsResource;
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
