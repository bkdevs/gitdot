"use client";

import { useEffect } from "react";

const KEY = "gitdot_recent_repos";
const MAX_ENTRIES = 25;

type RecentRepo = {
  owner: string;
  name: string;
  visitedAt: number;
};

export function RepoTracker({ owner, repo }: { owner: string; repo: string }) {
  useEffect(() => {
    let current: RecentRepo[] = [];
    try {
      const raw = localStorage.getItem(KEY);
      if (raw) {
        const parsed = JSON.parse(raw);
        if (Array.isArray(parsed)) current = parsed;
      }
    } catch {}

    const filtered = current.filter(
      (r) => !(r.owner === owner && r.name === repo),
    );
    const next: RecentRepo[] = [
      { owner, name: repo, visitedAt: Date.now() },
      ...filtered,
    ].slice(0, MAX_ENTRIES);

    try {
      localStorage.setItem(KEY, JSON.stringify(next));
    } catch {}
  }, [owner, repo]);

  return null;
}
