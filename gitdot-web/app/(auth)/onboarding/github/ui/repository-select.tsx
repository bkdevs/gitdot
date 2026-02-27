"use client";

import type { GitHubRepositoryResource } from "gitdot-api-ts";
import { useState } from "react";
import Link from "@/ui/link";

export function RepositorySelect({
  repositories,
}: {
  repositories: GitHubRepositoryResource[] | null;
}) {
  const [selectedRepos, setSelectedRepos] = useState<Set<string>>(new Set());

  function toggleRepo(name: string) {
    setSelectedRepos((prev) => {
      const next = new Set(prev);
      if (next.has(name)) {
        next.delete(name);
      } else {
        next.add(name);
      }
      return next;
    });
  }

  return (
    <div className="max-w-3xl mx-auto flex gap-4 items-center justify-center h-screen">
      <div className="flex flex-col text-sm w-sm">
        <p className="pb-2">Repositories.</p>
        <p className="text-primary/60 pb-4">
          Select the repositories you want to migrate to gitdot.
        </p>

        {repositories && repositories.length > 0 ? (
          <ul className="border border-border rounded divide-y divide-border max-h-64 overflow-y-auto">
            {repositories.map((repo) => (
              <li key={repo.id}>
                <label className="flex items-center gap-3 px-3 py-2 cursor-pointer hover:bg-accent/50">
                  <input
                    type="checkbox"
                    name="repositories"
                    value={repo.full_name}
                    checked={selectedRepos.has(repo.full_name)}
                    onChange={() => toggleRepo(repo.full_name)}
                  />
                  <span className="flex-1 truncate">{repo.full_name}</span>
                  {repo.private && (
                    <span className="text-xs text-primary/40">private</span>
                  )}
                </label>
              </li>
            ))}
          </ul>
        ) : (
          <p className="text-primary/60">No repositories found.</p>
        )}

        <div className="flex justify-end mt-4">
          <Link href="/home" className="decoration-primary/40">
            Continue.
          </Link>
        </div>
      </div>
    </div>
  );
}
