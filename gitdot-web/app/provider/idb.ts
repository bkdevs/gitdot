"use client";

import type {
  RepositoryBlobsResource,
  RepositoryCommitResource,
  RepositoryPathsResource,
} from "gitdot-api";
import { openDB } from "idb";
import type { Database } from "./types";
import { RepoProvider } from "./types";

const commitKey = (owner: string, repo: string, sha: string) =>
  `${owner}/${repo}/${sha}`;
const repoKey = (owner: string, repo: string) => `${owner}/${repo}`;
const pathKey = (owner: string, repo: string, path: string) =>
  `${owner}/${repo}/${path}`;
const blobKey = (owner: string, repo: string, path: string) =>
  `${owner}/${repo}/${path}`;

export class IdbProvider extends RepoProvider {
  private dbPromise: Promise<Database> | null = null;

  private db(): Promise<Database> {
    if (!this.dbPromise) this.dbPromise = openIdb();
    return this.dbPromise;
  }

  async getBlob(path: string) {
    if (typeof indexedDB === "undefined") return null;
    const db = await this.db();
    return db.getBlob(this.owner, this.repo, path);
  }

  async getCommit(sha: string) {
    if (typeof indexedDB === "undefined") return null;
    const db = await this.db();
    return db.getCommit(this.owner, this.repo, sha);
  }

  async getPaths() {
    if (typeof indexedDB === "undefined") return null;
    const db = await this.db();
    return db.getPaths(this.owner, this.repo);
  }

  async getCommits(): Promise<RepositoryCommitResource[] | null> {
    if (typeof indexedDB === "undefined") return null;
    const db = await this.db();
    const commits = await db.getAllCommits(this.owner, this.repo);
    return commits.length > 0 ? commits : null;
  }

  async getBlobs(): Promise<RepositoryBlobsResource | null> {
    if (typeof indexedDB === "undefined") return null;
    const db = await this.db();
    const blobs = await db.getBlobs(this.owner, this.repo);
    return blobs ?? null;
  }

  async putCommits(commits: RepositoryCommitResource[]): Promise<void> {
    if (typeof indexedDB === "undefined") return;
    const db = await this.db();
    await db.putCommits(this.owner, this.repo, commits);
  }

  async putPaths(paths: RepositoryPathsResource): Promise<void> {
    if (typeof indexedDB === "undefined") return;
    const db = await this.db();
    await db.putPaths(this.owner, this.repo, paths);
  }

  async putBlobs(blobs: RepositoryBlobsResource): Promise<void> {
    if (typeof indexedDB === "undefined") return;
    const db = await this.db();
    await db.putBlobs(this.owner, this.repo, blobs);
  }
}

export async function openIdb(): Promise<Database> {
  const db = await openDB("gitdot", 2, {
    upgrade(db) {
      if (!db.objectStoreNames.contains("commits"))
        db.createObjectStore("commits");
      if (!db.objectStoreNames.contains("paths")) db.createObjectStore("paths");
      if (!db.objectStoreNames.contains("blobs")) db.createObjectStore("blobs");
    },
  });

  return {
    async getCommit(owner, repo, sha) {
      return (await db.get("commits", commitKey(owner, repo, sha))) ?? null;
    },

    async putCommit(owner, repo, commit) {
      await db.put("commits", commit, commitKey(owner, repo, commit.sha));
    },

    async putCommits(owner, repo, commits) {
      const tx = db.transaction("commits", "readwrite");
      await Promise.all([
        ...commits.map((c) => tx.store.put(c, commitKey(owner, repo, c.sha))),
        tx.done,
      ]);
    },

    async getAllCommits(owner, repo) {
      const prefix = `${owner}/${repo}/`;
      const range = IDBKeyRange.bound(prefix, `${prefix}\uffff`);
      return db.getAll("commits", range);
    },

    async getPaths(owner, repo) {
      const prefix = `${repoKey(owner, repo)}/`;
      const range = IDBKeyRange.bound(prefix, `${prefix}\uffff`);
      const rows = await db.getAll("paths", range);
      if (rows.length === 0) return null;
      const { ref_name, commit_sha } = rows[0];
      return {
        ref_name,
        commit_sha,
        entries: rows.map(({ ref_name, commit_sha, ...e }) => e),
      };
    },

    async putPaths(owner, repo, paths) {
      const { ref_name, commit_sha } = paths;
      const tx = db.transaction("paths", "readwrite");
      await Promise.all([
        ...paths.entries.map((e) =>
          tx.store.put(
            { ref_name, commit_sha, ...e },
            pathKey(owner, repo, e.path),
          ),
        ),
        tx.done,
      ]);
    },

    async getBlob(owner, repo, path) {
      const row = await db.get("blobs", blobKey(owner, repo, path));
      if (!row) return null;
      const { ref_name, commit_sha, ...blob } = row;
      return blob;
    },

    async getBlobs(owner, repo) {
      const prefix = `${repoKey(owner, repo)}/`;
      const range = IDBKeyRange.bound(prefix, `${prefix}\uffff`);
      const rows = await db.getAll("blobs", range);
      if (rows.length === 0) return undefined;
      const { ref_name, commit_sha } = rows[0];
      return {
        ref_name,
        commit_sha,
        blobs: rows.map(({ ref_name, commit_sha, ...b }) => b),
      };
    },

    async putBlobs(owner, repo, blobs) {
      const { ref_name, commit_sha } = blobs;
      const tx = db.transaction("blobs", "readwrite");
      await Promise.all([
        ...blobs.blobs.map((b) =>
          tx.store.put(
            { ref_name, commit_sha, ...b },
            blobKey(owner, repo, b.path),
          ),
        ),
        tx.done,
      ]);
    },
  };
}
