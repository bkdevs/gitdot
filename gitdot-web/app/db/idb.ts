"use client";

import type { Root } from "hast";
import { type IDBPDatabase, openDB } from "idb";
import type { Database } from "./types";

const commitKey = (owner: string, repo: string, sha: string) =>
  `${owner}/${repo}/${sha}`;
const repoKey = (owner: string, repo: string) => `${owner}/${repo}`;
const pathKey = (owner: string, repo: string, path: string) =>
  `${owner}/${repo}/${path}`;
const blobKey = (owner: string, repo: string, path: string) =>
  `${owner}/${repo}/${path}`;

let dbPromise: Promise<IDBPDatabase> | null = null;
function getDb(): Promise<IDBPDatabase> {
  if (!dbPromise) {
    dbPromise = openDB("gitdot", 4, {
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

    async getBlob(owner, repo, path) {
      const db = await getDb();
      const row = await db.get("blobs", blobKey(owner, repo, path));
      console.log(blobKey(owner, repo, path));
      if (!row) return null;
      const { ref_name, commit_sha, ...blob } = row;
      return blob;
    },

    async getBlobs(owner, repo) {
      const db = await getDb();
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
      const db = await getDb();
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

    async getHast(owner, repo, path) {
      const db = await getDb();
      return (await db.get("hasts", blobKey(owner, repo, path))) ?? null;
    },

    async getHasts(owner, repo) {
      const db = await getDb();
      const prefix = `${repoKey(owner, repo)}/`;
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

    async putHast(owner, repo, path, hast) {
      const db = await getDb();
      await db.put("hasts", hast, blobKey(owner, repo, path));
    },

    async getSettings(owner, repo) {
      const db = await getDb();
      return (await db.get("settings", repoKey(owner, repo))) ?? null;
    },

    async putSettings(owner, repo, settings) {
      const db = await getDb();
      await db.put("settings", settings, repoKey(owner, repo));
    },
  };
}
