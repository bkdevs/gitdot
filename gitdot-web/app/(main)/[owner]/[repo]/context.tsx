"use client";

import type {
  RepositoryCommitResource,
  RepositoryCommitsResource,
  RepositoryPreviewResource,
  RepositoryTreeResource,
} from "gitdot-api";
import { createContext, use, useContext, useMemo } from "react";

interface RepoContext {
  tree: Promise<RepositoryTreeResource>;
  commits: Promise<RepositoryCommitResource[]>;
  preview: Promise<RepositoryPreviewResource>;
}

const RepoContext = createContext<RepoContext | null>(null);
class RepoError extends Error {}

async function requireNotNull<T>(promise: Promise<T | null>): Promise<T> {
  const value = await promise;
  if (value === null) {
    throw new RepoError("Resource fetch failed");
  }
  return value;
}

export function RepoProvider({
  tree,
  commits,
  preview,
  children,
}: {
  tree: Promise<RepositoryTreeResource | null>;
  commits: Promise<RepositoryCommitsResource | null>;
  preview: Promise<RepositoryPreviewResource | null>;
  children: React.ReactNode;
}) {
  const value = useMemo(
    () => ({
      tree: requireNotNull(tree),
      commits: requireNotNull(commits).then((c) => c.commits),
      preview: requireNotNull(preview),
    }),
    [tree, commits, preview],
  );

  return <RepoContext value={value}>{children}</RepoContext>;
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
