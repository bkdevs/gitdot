"use client";

import type {
  QuestionResource,
  RepositoryBlobResource,
  RepositoryBlobsResource,
  ReviewResource,
} from "gitdot-api";
import type { Root } from "hast";
import { type IDBPDatabase, openDB } from "idb";
import type { Database, RepositoryMetadata } from "./types";

const commitKey = (owner: string, repo: string, sha: string) =>
  `${owner}/${repo}/${sha}`;
const reviewKey = (owner: string, repo: string, number: number) =>
  `${owner}/${repo}/${number}`;
const repoKey = (owner: string, repo: string) => `${owner}/${repo}`;
const pathKey = (owner: string, repo: string, path: string) =>
  `${owner}/${repo}/${path}`;
const blobPrefix = (owner: string, repo: string, commit: string) =>
  `${owner}/${repo}/${commit.slice(0, 7)}/`;
const blobKey = (owner: string, repo: string, commit: string, path: string) =>
  `${blobPrefix(owner, repo, commit)}${path}`;

let dbPromise: Promise<IDBPDatabase> | null = null;
function getDb(): Promise<IDBPDatabase> {
  if (!dbPromise) {
    dbPromise = openDB("gitdot", 9, {
      upgrade(db) {
        if (!db.objectStoreNames.contains("commits"))
          db.createObjectStore("commits");
        if (!db.objectStoreNames.contains("paths"))
          db.createObjectStore("paths");
        if (!db.objectStoreNames.contains("blobs"))
          db.createObjectStore("blobs");
        if (!db.objectStoreNames.contains("hasts"))
          db.createObjectStore("hasts");
        if (!db.objectStoreNames.contains("settings"))
          db.createObjectStore("settings");
        if (!db.objectStoreNames.contains("metadata"))
          db.createObjectStore("metadata");
        if (!db.objectStoreNames.contains("questions"))
          db.createObjectStore("questions");
        if (!db.objectStoreNames.contains("reviews"))
          db.createObjectStore("reviews");
        if (!db.objectStoreNames.contains("builds"))
          db.createObjectStore("builds");
      },
    });
  }
  return dbPromise;
}

export function openIdb(): Database {
  if (typeof indexedDB === "undefined") {
    return new Proxy({} as Database, {
      get: () => () => Promise.resolve(null),
    });
  }

  getDb();

  return {
    async getPaths(owner, repo) {
      const db = await getDb();
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
      const db = await getDb();
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

    async getCommit(owner, repo, sha) {
      const db = await getDb();
      return (await db.get("commits", commitKey(owner, repo, sha))) ?? null;
    },

    async getCommits(owner, repo) {
      const db = await getDb();
      const prefix = `${owner}/${repo}/`;
      const range = IDBKeyRange.bound(prefix, `${prefix}\uffff`);
      return db.getAll("commits", range);
    },

    async putCommit(owner, repo, commit) {
      const db = await getDb();
      await db.put("commits", commit, commitKey(owner, repo, commit.sha));
    },

    async putCommits(owner, repo, commits) {
      const db = await getDb();
      const tx = db.transaction("commits", "readwrite");
      await Promise.all([
        ...commits.map((c) => tx.store.put(c, commitKey(owner, repo, c.sha))),
        tx.done,
      ]);
    },

    async getBlob(owner: string, repo: string, path: string, commit: string) {
      const db = await getDb();
      return (
        (await db.get("blobs", blobKey(owner, repo, commit, path))) ?? null
      );
    },

    async getBlobs(owner: string, repo: string, commit: string) {
      const db = await getDb();
      const prefix = blobPrefix(owner, repo, commit);
      const range = IDBKeyRange.bound(prefix, `${prefix}\uffff`);
      const rows = await db.getAll("blobs", range);
      if (rows.length === 0) return null;
      return { blobs: rows };
    },

    async putBlob(
      owner: string,
      repo: string,
      path: string,
      commit: string,
      blob: RepositoryBlobResource,
    ) {
      const db = await getDb();
      await db.put("blobs", blob, blobKey(owner, repo, commit, path));
    },

    async putBlobs(
      owner: string,
      repo: string,
      blobs: RepositoryBlobsResource,
    ) {
      const db = await getDb();
      const tx = db.transaction("blobs", "readwrite");
      await Promise.all([
        ...blobs.blobs.map((b) =>
          tx.store.put(b, blobKey(owner, repo, b.commit_sha, b.path)),
        ),
        tx.done,
      ]);
    },

    async getHast(owner: string, repo: string, path: string, commit: string) {
      const db = await getDb();
      return (
        (await db.get("hasts", blobKey(owner, repo, commit, path))) ?? null
      );
    },

    async getHasts(owner: string, repo: string, commit: string) {
      const db = await getDb();
      const prefix = blobPrefix(owner, repo, commit);
      const range = IDBKeyRange.bound(prefix, `${prefix}\uffff`);
      const keys = await db.getAllKeys("hasts", range);
      const values = await db.getAll("hasts", range);
      if (values.length === 0) return null;
      const map = new Map<string, Root>();
      for (let i = 0; i < keys.length; i++) {
        const path = (keys[i] as string).slice(prefix.length);
        map.set(path, values[i]);
      }
      return map;
    },

    async putHast(
      owner: string,
      repo: string,
      path: string,
      hast: Root,
      commit: string,
    ) {
      const db = await getDb();
      await db.put("hasts", hast, blobKey(owner, repo, commit, path));
    },

    async getSettings(owner, repo) {
      const db = await getDb();
      return (await db.get("settings", repoKey(owner, repo))) ?? null;
    },

    async putSettings(owner, repo, settings) {
      const db = await getDb();
      await db.put("settings", settings, repoKey(owner, repo));
    },

    async getQuestions(owner, repo): Promise<QuestionResource[] | null> {
      const db = await getDb();
      return (await db.get("questions", repoKey(owner, repo))) ?? null;
    },

    async putQuestions(owner, repo, questions: QuestionResource[]) {
      const db = await getDb();
      await db.put("questions", questions, repoKey(owner, repo));
    },

    async getMetadata(owner, repo) {
      const db = await getDb();
      return (await db.get("metadata", repoKey(owner, repo))) ?? null;
    },

    async putMetadata(owner, repo, metadata: RepositoryMetadata) {
      const db = await getDb();
      await db.put("metadata", metadata, repoKey(owner, repo));
    },

    async getReview(owner, repo, number) {
      const db = await getDb();
      return (await db.get("reviews", reviewKey(owner, repo, number))) ?? null;
    },

    async getReviews(owner, repo) {
      const db = await getDb();
      const prefix = `${repoKey(owner, repo)}/`;
      const range = IDBKeyRange.bound(prefix, `${prefix}\uffff`);
      return db.getAll("reviews", range);
    },

    async putReview(owner, repo, number, review: ReviewResource) {
      const db = await getDb();
      await db.put("reviews", review, reviewKey(owner, repo, number));
    },

    async getBuilds(owner, repo) {
      const db = await getDb();
      return (await db.get("builds", repoKey(owner, repo))) ?? null;
    },

    async putBuilds(owner, repo, builds) {
      const db = await getDb();
      await db.put("builds", builds, repoKey(owner, repo));
    },

    async getBuild(owner, repo, number) {
      const builds = await this.getBuilds(owner, repo);
      return builds?.find((b) => b.number === number) ?? null;
    },

    async putBuild(owner, repo, build) {
      const builds = (await this.getBuilds(owner, repo)) ?? [];
      const idx = builds.findIndex((b) => b.number === build.number);
      if (idx >= 0) builds[idx] = build;
      else builds.push(build);
      await this.putBuilds(owner, repo, builds);
    },
  };
}
