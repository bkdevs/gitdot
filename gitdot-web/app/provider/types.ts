"use client";

import type {
  RepositoryBlobResource,
  RepositoryBlobsResource,
  RepositoryCommitResource,
  RepositoryPathsResource,
} from "gitdot-api";

export abstract class RepoProvider {
  protected owner: string;
  protected repo: string;
  private callLog: Array<{ method: string; args: any[] }> = [];

  constructor(owner: string, repo: string) {
    this.owner = owner;
    this.repo = repo;

    return new Proxy(this, {
      get(target, prop) {
        const val = (target as any)[prop];
        if (typeof val !== 'function' || prop === 'replay') return val;
        return (...args: any[]) => {
          target.callLog.push({ method: prop as string, args });
          return val.apply(target, args);
        };
      }
    });
  }

  async replay(callLog: Array<{ method: string; args: any[] }>) {
    for (const { method, args } of callLog) {
      await (this as any)[method](...args);
    }
  }

  abstract getBlob(id: string): Promise<RepositoryBlobResource | null>;
  abstract getCommit(sha: string): Promise<RepositoryCommitResource | null>;
  abstract getPaths(): Promise<RepositoryPathsResource | null>;
}

export interface Database {
  getCommit(
    owner: string,
    repo: string,
    sha: string,
  ): Promise<RepositoryCommitResource | undefined>;

  putCommit(
    owner: string,
    repo: string,
    commit: RepositoryCommitResource,
  ): Promise<void>;

  putCommits(
    owner: string,
    repo: string,
    commits: RepositoryCommitResource[],
  ): Promise<void>;

  getAllCommits(
    owner: string,
    repo: string,
  ): Promise<RepositoryCommitResource[]>;

  getPaths(
    owner: string,
    repo: string,
  ): Promise<RepositoryPathsResource | undefined>;

  putPaths(
    owner: string,
    repo: string,
    paths: RepositoryPathsResource,
  ): Promise<void>;

  getBlob(
    owner: string,
    repo: string,
    path: string,
  ): Promise<RepositoryBlobResource | undefined>;

  getBlobs(
    owner: string,
    repo: string,
  ): Promise<RepositoryBlobsResource | undefined>;

  putBlobs(
    owner: string,
    repo: string,
    blobs: RepositoryBlobsResource,
  ): Promise<void>;
}
