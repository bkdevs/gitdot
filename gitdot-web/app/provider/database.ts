"use client";

import type {
  RepositoryBlobsResource,
  RepositoryCommitResource,
  RepositoryPathsResource,
  RepositorySettingsResource,
} from "gitdot-api";
import type { Root } from "hast";
import { openIdb } from "@/db";
import { ClientProvider } from "./types";

export class DatabaseProvider extends ClientProvider {
  private db = openIdb();

  async getPaths() {
    return this.db.getPaths(this.owner, this.repo);
  }

  async putPaths(paths: RepositoryPathsResource) {
    return this.db.putPaths(this.owner, this.repo, paths);
  }

  async getBlob(path: string) {
    return this.db.getBlob(this.owner, this.repo, path);
  }

  async getHast(path: string): Promise<Root | null> {
    return this.db.getHast(this.owner, this.repo, path);
  }

  async putHast(path: string, hast: Root): Promise<void> {
    return this.db.putHast(this.owner, this.repo, path, hast);
  }

  async getCommit(sha: string) {
    return this.db.getCommit(this.owner, this.repo, sha);
  }

  async getCommits(): Promise<RepositoryCommitResource[] | null> {
    const commits = await this.db.getCommits(this.owner, this.repo);
    if (commits === null || commits.length === 0) return null;
    return commits.sort(
      (a, b) => new Date(b.date).getTime() - new Date(a.date).getTime(),
    );
  }

  async putCommits(commits: RepositoryCommitResource[]) {
    return this.db.putCommits(this.owner, this.repo, commits);
  }

  async getBlobs(): Promise<RepositoryBlobsResource | null> {
    const blobs = await this.db.getBlobs(this.owner, this.repo);
    return blobs ?? null;
  }

  async putBlobs(blobs: RepositoryBlobsResource) {
    return this.db.putBlobs(this.owner, this.repo, blobs);
  }

  async getSettings() {
    return this.db.getSettings(this.owner, this.repo);
  }

  async putSettings(settings: RepositorySettingsResource) {
    return this.db.putSettings(this.owner, this.repo, settings);
  }

  // TODO: this is a tad hacky (relying on the fact that provider get / put methods) are serialized as such
  // we can do better if we discrminate resource types directly with a "type" field.
  private writers: Record<string, (value: unknown) => void> = {
    getPaths: (v) => this.putPaths(v as RepositoryPathsResource),
    getCommits: (v) => this.putCommits(v as RepositoryCommitResource[]),
    getBlobs: (v) => this.putBlobs(v as RepositoryBlobsResource),
    getSettings: (v) => this.putSettings(v as RepositorySettingsResource),
  };

  write(method: string, value: unknown) {
    if (!value) return;
    this.writers[method]?.(value);
  }
}
