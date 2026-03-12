"use client";

import { openDB } from "idb";
import type { Database } from "./types";

const commitKey = (owner: string, repo: string, sha: string) =>
  `${owner}/${repo}/${sha}`;
const repoKey = (owner: string, repo: string) => `${owner}/${repo}`;

export async function openIdb(): Promise<Database> {
  const db = await openDB("gitdot", 1, {
    upgrade(db) {
      if (!db.objectStoreNames.contains("commits"))
        db.createObjectStore("commits");
      if (!db.objectStoreNames.contains("tree")) db.createObjectStore("tree");
      if (!db.objectStoreNames.contains("preview"))
        db.createObjectStore("preview");
    },
  });

  return {
    getCommit(owner, repo, sha) {
      return db.get("commits", commitKey(owner, repo, sha));
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
    getTree(owner, repo) {
      return db.get("tree", repoKey(owner, repo));
    },
    async putTree(owner, repo, tree) {
      await db.put("tree", tree, repoKey(owner, repo));
    },
    getPreview(owner, repo) {
      return db.get("preview", repoKey(owner, repo));
    },
    async putPreview(owner, repo, preview) {
      await db.put("preview", preview, repoKey(owner, repo));
    },
  };
}
