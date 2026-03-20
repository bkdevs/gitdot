"use client";

import type {
  RepositoryBlobResource,
  RepositoryBlobsResource,
  RepositoryCommitResource,
  RepositoryPathsResource,
  RepositorySettingsResource,
} from "gitdot-api";
import { RepoProvider } from "./types";

type MemoryStore = {
  paths: RepositoryPathsResource | undefined;
  blobs: RepositoryBlobsResource | undefined;
  commits: RepositoryCommitResource[] | undefined;
  settings: RepositorySettingsResource | undefined;
  blob: Map<string, RepositoryBlobResource>;
  commit: Map<string, RepositoryCommitResource>;
};

export class MemoryProvider extends RepoProvider {
  private store: MemoryStore = {
    paths: undefined,
    blobs: undefined,
    commits: undefined,
    settings: undefined,
    blob: new Map(),
    commit: new Map(),
  };

  async getPaths(): Promise<RepositoryPathsResource | null> {
    return this.store.paths ?? null;
  }

  async getBlob(path: string): Promise<RepositoryBlobResource | null> {
    return this.store.blob.get(path) ?? null;
  }

  async getCommit(sha: string): Promise<RepositoryCommitResource | null> {
    return this.store.commit.get(sha) ?? null;
  }

  async getCommits(): Promise<RepositoryCommitResource[] | null> {
    return this.store.commits ?? null;
  }

  async getBlobs(): Promise<RepositoryBlobsResource | null> {
    return this.store.blobs ?? null;
  }

  async getSettings(): Promise<RepositorySettingsResource | null> {
    return this.store.settings ?? null;
  }
}
