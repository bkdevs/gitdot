"use client";

import type {
  RepositoryBlobResource,
  RepositoryBlobsResource,
  RepositoryCommitResource,
  RepositoryPathsResource,
  RepositorySettingsResource,
} from "gitdot-api";
import type { Root } from "hast";
import { openIdb } from "@/db";
import { ClientProvider } from "./types";

type Store = {
  paths: RepositoryPathsResource | undefined;
  blobs: RepositoryBlobsResource | undefined;
  commits: RepositoryCommitResource[] | undefined;
  settings: RepositorySettingsResource | undefined;
  blob: Map<string, RepositoryBlobResource>;
  commit: Map<string, RepositoryCommitResource>;
  hast: Map<string, Root>;
};

export class InMemoryProvider extends ClientProvider {
  private store: Store = {
    paths: undefined,
    blobs: undefined,
    commits: undefined,
    settings: undefined,
    blob: new Map(),
    commit: new Map(),
    hast: new Map(),
  };

  async getPaths(): Promise<RepositoryPathsResource | null> {
    return this.store.paths ?? null;
  }

  async getBlob(path: string): Promise<RepositoryBlobResource | null> {
    return this.store.blob.get(path) ?? null;
  }

  async getHast(path: string): Promise<Root | null> {
    return this.store.hast.get(path) ?? null;
  }

  putHast(path: string, hast: Root): void {
    this.store.hast.set(path, hast);
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

  async initialize(): Promise<void> {
    const db = openIdb();
    const [paths, blobs, commits, settings, hasts] = await Promise.all([
      db.getPaths(this.owner, this.repo),
      db.getBlobs(this.owner, this.repo),
      db.getCommits(this.owner, this.repo),
      db.getSettings(this.owner, this.repo),
      db.getHasts(this.owner, this.repo),
    ]);
    if (paths) this.store.paths = paths;
    if (blobs) {
      this.store.blobs = blobs;
      for (const b of blobs.blobs) this.store.blob.set(b.path, b);
    }
    if (commits?.length) {
      this.store.commits = commits;
      for (const c of commits) this.store.commit.set(c.sha, c);
    }
    if (settings) this.store.settings = settings;
    if (hasts) this.store.hast = hasts;
  }
}
