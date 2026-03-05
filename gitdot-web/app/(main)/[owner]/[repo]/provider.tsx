"use client";

import type {
  RepositoryCommitsResource,
  RepositoryTreeResource,
} from "gitdot-api";
import { createContext } from "react";

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
