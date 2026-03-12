import { openDB } from "idb";
import { Database } from "./types";

export async function openIdb(): Promise<Database> {
  const db = await openDB("gitdot", 1, {
    upgrade(db) {
      db.createObjectStore("commits", { keyPath: "sha" });
    },
  });

  return {
    getCommit(sha) {
      return db.get("commits", sha);
    },
    async putCommit(commit) {
      await db.put("commits", commit);
    },
    async putCommits(commits) {
      const tx = db.transaction("commits", "readwrite");
      await Promise.all([...commits.map((c) => tx.store.put(c)), tx.done]);
    },
    getAllCommits() {
      return db.getAll("commits");
    },
  };
}
