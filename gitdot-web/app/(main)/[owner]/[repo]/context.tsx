"use client";

import type {
  RepositoryCommitResource,
  RepositoryCommitsResource,
  RepositoryTreeResource,
} from "gitdot-api";
import { createContext, use, useContext } from "react";

interface RepoContext {
  tree: Promise<RepositoryTreeResource>;
  commits: Promise<RepositoryCommitResource[]>;
}

const RepoContext = createContext<RepoContext | null>(null);
class RepoError extends Error {}

export function RepoProvider({
  tree,
  commits,
  children,
}: {
  tree: Promise<RepositoryTreeResource | null>;
  commits: Promise<RepositoryCommitsResource | null>;
  children: React.ReactNode;
  }) {
  // TODO: think through, a bit hairy.
  async function requireNotNull<T>(promise: Promise<T | null>): Promise<T> {
    const value = await promise;
    if (value === null) {
      throw new RepoError("Resource fetch failed");
    }
    return value;
  }

  return (
    <RepoContext
      value={{
        tree: requireNotNull(tree),
        commits: requireNotNull(commits).then((c) => c.commits),
      }}
    >
      {children}
    </RepoContext>
  );
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
