"use client";

import type {
  RepositoryCommitResource,
  RepositoryCommitsResource,
  RepositoryPreviewResource,
  RepositoryTreeResource,
} from "gitdot-api";
import { createContext, use, useContext, useEffect, useMemo } from "react";
import { useDatabaseContext } from "@/(main)/context/database";
import { firstNonNull } from "@/util";

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
  owner,
  repo,
  tree,
  commits,
  preview,
  children,
}: {
  owner: string;
  repo: string;
  tree: Promise<RepositoryTreeResource | null>;
  commits: Promise<RepositoryCommitsResource | null>;
  preview: Promise<RepositoryPreviewResource | null>;
  children: React.ReactNode;
}) {
  const { db } = useDatabaseContext();

  const value = useMemo(() => {
    const serverTree = requireNotNull(tree);
    const serverCommits = requireNotNull(commits).then((c) => c.commits);
    const serverPreview = requireNotNull(preview);

    if (!db) {
      console.log("[db] db is null, using server promises");
      return {
        tree: serverTree,
        commits: serverCommits,
        preview: serverPreview,
      };
    }

    console.log("[db] db is ready, racing IDB vs server");
    const t0 = performance.now();
    const ms = () => `${(performance.now() - t0).toFixed(1)}ms`;

    const treeFromDb = db.getTree(owner, repo).then((v) => {
      console.log(`[db] idb tree: ${ms()}`);
      return v;
    });
    const commitsFromDb = db.getAllCommits(owner, repo).then((c) => {
      console.log(`[db] idb commits: ${ms()}`);
      return c.length > 0 ? c : null;
    });
    const previewFromDb = db.getPreview(owner, repo).then((v) => {
      console.log(`[db] idb preview: ${ms()}`);
      return v;
    });

    serverTree.then(() => console.log(`[db] server tree: ${ms()}`));
    serverCommits.then(() => console.log(`[db] server commits: ${ms()}`));
    serverPreview.then(() => console.log(`[db] server preview: ${ms()}`));

    return {
      tree: firstNonNull(treeFromDb, serverTree),
      commits: firstNonNull(commitsFromDb, serverCommits),
      preview: firstNonNull(previewFromDb, serverPreview),
    };
  }, [db, owner, repo, tree, commits, preview]);

  useEffect(() => {
    if (!db) return;
    requireNotNull(tree).then((t) => db.putTree(owner, repo, t));
    requireNotNull(commits).then((c) => db.putCommits(owner, repo, c.commits));
    requireNotNull(preview).then((p) => db.putPreview(owner, repo, p));
  }, [db, owner, repo, tree, commits, preview]);

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
