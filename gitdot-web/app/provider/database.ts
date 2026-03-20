"use client";

import type {
  RepositoryBlobsResource,
  RepositoryCommitResource,
} from "gitdot-api";
import { openIdb } from "@/db";
import { RepoProvider } from "./types";

export class DatabaseProvider extends RepoProvider {
  private db = openIdb();

  async getPaths() {
    return this.db.getPaths(this.owner, this.repo);
  }

  async getBlob(path: string) {
    return this.db.getBlob(this.owner, this.repo, path);
  }

  async getCommit(sha: string) {
    return this.db.getCommit(this.owner, this.repo, sha);
  }

  async getCommits(): Promise<RepositoryCommitResource[] | null> {
    const commits = await this.db.getCommits(this.owner, this.repo);
    return commits.length > 0 ? commits : null;
  }

  async getBlobs(): Promise<RepositoryBlobsResource | null> {
    const blobs = await this.db.getBlobs(this.owner, this.repo);
    return blobs ?? null;
  }

  async getSettings() {
    return this.db.getSettings(this.owner, this.repo);
  }
}
