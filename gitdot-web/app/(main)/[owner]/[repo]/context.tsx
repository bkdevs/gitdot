"use client";

import type {
  RepositoryBlobsResource,
  RepositoryCommitResource,
  RepositoryCommitsResource,
  RepositoryPathsResource,
  RepositoryPreviewResource,
} from "gitdot-api";
import { createContext, use, useContext, useEffect, useMemo } from "react";
import { useDatabaseContext } from "@/(main)/context/database";
import { firstNonNull } from "@/util";

interface RepoContext {
  commits: Promise<RepositoryCommitResource[]>;
  preview: Promise<RepositoryPreviewResource>;
  paths: Promise<RepositoryPathsResource>;
  blobs: Promise<RepositoryBlobsResource>;
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
  commits,
  preview,
  paths,
  blobs,
  children,
}: {
  owner: string;
  repo: string;
  commits: Promise<RepositoryCommitsResource | null>;
  preview: Promise<RepositoryPreviewResource | null>;
  paths: Promise<RepositoryPathsResource | null>;
  blobs: Promise<RepositoryBlobsResource | null>;
  children: React.ReactNode;
}) {
  const { db } = useDatabaseContext();

  const value = useMemo(() => {
    const serverCommits = requireNotNull(commits).then((c) => c.commits);
    const serverPreview = requireNotNull(preview);
    const serverPaths = requireNotNull(paths);
    const serverBlobs = requireNotNull(blobs);

    if (!db) {
      console.log("[db] db is null, using server promises");
      return {
        commits: serverCommits,
        preview: serverPreview,
        paths: serverPaths,
        blobs: serverBlobs,
      };
    }

    console.log("[db] db is ready, racing IDB vs server");
    const t0 = performance.now();
    const ms = () => `${(performance.now() - t0).toFixed(1)}ms`;

    const commitsFromDb = db.getAllCommits(owner, repo).then((c) => {
      console.log(`[db] idb commits: ${ms()}`);
      return c.length > 0 ? c : null;
    });
    const previewFromDb = db.getPreview(owner, repo).then((v) => {
      console.log(`[db] idb preview: ${ms()}`);
      return v;
    });
    const pathsFromDb = db.getPaths(owner, repo).then((v) => {
      console.log(`[db] idb paths: ${ms()}`);
      return v;
    });
    const blobsFromDb = db.getBlobs(owner, repo).then((v) => {
      console.log(`[db] idb blobs: ${ms()}`);
      return v;
    });

    serverCommits.then(() => console.log(`[db] server commits: ${ms()}`));
    serverPreview.then(() => console.log(`[db] server preview: ${ms()}`));
    serverPaths.then(() => console.log(`[db] server paths: ${ms()}`));
    serverBlobs.then(() => console.log(`[db] server blobs: ${ms()}`));

    return {
      commits: firstNonNull(commitsFromDb, serverCommits),
      preview: firstNonNull(previewFromDb, serverPreview),
      paths: firstNonNull(pathsFromDb, serverPaths),
      blobs: firstNonNull(blobsFromDb, serverBlobs),
    };
  }, [db, owner, repo, commits, preview, paths, blobs]);

  useEffect(() => {
    if (!db) return;
    requireNotNull(commits).then((c) => db.putCommits(owner, repo, c.commits));
    requireNotNull(preview).then((p) => db.putPreview(owner, repo, p));
    requireNotNull(paths).then((p) => db.putPaths(owner, repo, p));
    requireNotNull(blobs).then((b) => db.putBlobs(owner, repo, b));
  }, [db, owner, repo, commits, preview, paths, blobs]);

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
